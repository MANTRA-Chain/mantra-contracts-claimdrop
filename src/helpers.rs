use std::collections::HashMap;

use cosmwasm_std::{
    ensure, Addr, Coin, Decimal256, Deps, MessageInfo, Timestamp, Uint128, Uint256,
};
use sha2::Digest;

use crate::error::ContractError;
use crate::msg::{Campaign, CampaignParams, DistributionType};
use crate::state::{get_claims_for_address, Claim, DistributionSlot};

/// Validates the provided campaign parameters are valid.
pub(crate) fn validate_campaign_params(
    current_time: Timestamp,
    info: &MessageInfo,
    campaign_params: &CampaignParams,
) -> Result<(), ContractError> {
    campaign_params.validate_campaign_name_description()?;
    validate_merkle_root(&campaign_params.merkle_root)?;
    campaign_params.validate_campaign_times(current_time)?;
    campaign_params.validate_campaign_distribution()?;

    let reward_amount = cw_utils::must_pay(info, &campaign_params.reward_asset.denom)?;
    ensure!(
        reward_amount == campaign_params.reward_asset.amount,
        ContractError::InvalidRewardAmount {
            expected: campaign_params.reward_asset.amount,
            actual: reward_amount
        }
    );

    Ok(())
}

/// Validates the merkle root, i.e. checks if it is a valid SHA-256 hash
pub(crate) fn validate_merkle_root(merkle_root: &str) -> Result<[u8; 32], ContractError> {
    let mut merkle_root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(merkle_root, &mut merkle_root_buf)?;

    Ok(merkle_root_buf)
}

/// Validates the claim proof
pub(crate) fn validate_claim(
    contract_addr: &Addr,
    receiver: &Addr,
    amount: Uint128,
    proof: &[String],
    merkle_root: &str,
) -> Result<(), ContractError> {
    let user_input = format!("{}{}{}", contract_addr, receiver, amount);
    let hash = sha2::Sha256::digest(user_input.as_bytes())
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongHashLength)?;

    let hash = proof.iter().try_fold(hash, |hash, p| {
        let mut proof_buf = [0; 32];
        hex::decode_to_slice(p, &mut proof_buf)?;
        let mut hashes = [hash, proof_buf];
        hashes.sort_unstable();
        sha2::Sha256::digest(hashes.concat())
            .as_slice()
            .try_into()
            .map_err(|_| ContractError::WrongHashLength {})
    })?;

    let merkle_root_buf = validate_merkle_root(merkle_root)?;

    ensure!(
        merkle_root_buf == hash,
        ContractError::MerkleRootVerificationFailed
    );

    Ok(())
}

/// Constant used for the fallback distribution slot
const FALLBACK_DISTRIBUTION_SLOT: usize = 0usize;

/// Calculates the amount a user can claim at this point in time
pub(crate) fn compute_claimable_amount(
    deps: Deps,
    campaign: &Campaign,
    current_time: &Timestamp,
    address: &Addr,
    total_claimable_amount: Uint128,
) -> Result<(Coin, HashMap<DistributionSlot, Claim>), ContractError> {
    let mut claimable_amount = Uint128::zero();
    let mut new_claims = HashMap::new();

    if campaign.has_started(current_time) {
        let previous_claims_for_address = get_claims_for_address(deps, address)?;

        for (distribution_slot, distribution) in
            campaign.distribution_type.iter().enumerate().clone()
        {
            // skip distributions that have not started yet
            if !distribution.has_started(current_time) {
                continue;
            }

            // check if the cliff period has passed for linear vesting distributions
            if let DistributionType::LinearVesting {
                cliff_duration: Some(cliff_duration),
                start_time,
                ..
            } = distribution
            {
                let cliff_end_time = start_time + cliff_duration;

                // if the cliff period has not passed yet, skip
                if current_time.seconds() < cliff_end_time {
                    continue;
                }
            }

            let previous_claim_for_address_for_distribution =
                previous_claims_for_address.get(&distribution_slot);

            let claim_amount = calculate_claim_amount_for_distribution(
                &current_time,
                total_claimable_amount,
                &distribution,
                &previous_claim_for_address_for_distribution,
            )?;

            // nothing to claim for the current distribution, skip
            if claim_amount == Uint128::zero() {
                continue;
            }

            claimable_amount = claimable_amount.checked_add(claim_amount)?;

            new_claims.insert(distribution_slot, (claim_amount, current_time.seconds()));
        }

        let (rounding_error_compensation_amount, slot) = get_compensation_for_rounding_errors(
            campaign,
            current_time,
            total_claimable_amount,
            previous_claims_for_address,
            &new_claims,
        )?;

        if rounding_error_compensation_amount > Uint128::zero() {
            claimable_amount = claimable_amount.checked_add(rounding_error_compensation_amount)?;

            let (amount, _) = match new_claims.get_mut(&slot) {
                Some(existing_claim) => existing_claim,
                None => {
                    let new_claim = (Uint128::zero(), current_time.seconds());
                    new_claims.insert(slot, new_claim);
                    new_claims.get_mut(&slot).unwrap()
                }
            };

            *amount = amount.checked_add(rounding_error_compensation_amount)?;
        }
    } else {
        return Err(ContractError::CampaignError {
            reason: "not started".to_string(),
        });
    }

    Ok((
        Coin {
            denom: campaign.reward_asset.denom.clone(),
            amount: claimable_amount,
        },
        new_claims,
    ))
}

/// Calculates the claimable amount for a given distribution, total amount and previous claim.
fn calculate_claim_amount_for_distribution(
    current_time: &&Timestamp,
    total_claimable_amount: Uint128,
    distribution_type: &&DistributionType,
    previous_claim_for_address_for_distribution: &Option<&Claim>,
) -> Result<Uint128, ContractError> {
    match distribution_type {
        DistributionType::LinearVesting {
            percentage,
            start_time,
            end_time,
            ..
        } => {
            let elapsed_time = match previous_claim_for_address_for_distribution {
                Some((_, last_claimed)) if end_time >= last_claimed => {
                    current_time.seconds().min(end_time.to_owned()) - last_claimed
                }
                Some(_) => return Ok(Uint128::zero()), // it means the user has already claimed this distribution
                None => current_time.seconds().min(end_time.to_owned()) - start_time,
            };

            let vesting_progress = Decimal256::from_ratio(
                Uint256::from(elapsed_time),
                Uint256::from(end_time - start_time),
            );

            Ok(Uint128::try_from(
                Decimal256::from(*percentage)
                    .checked_mul(Decimal256::from_ratio(
                        Uint256::from_uint128(total_claimable_amount),
                        Uint256::one(),
                    ))?
                    .checked_mul(vesting_progress)?
                    .to_uint_floor(),
            )?)
        }
        DistributionType::LumpSum { percentage, .. } => {
            // it means the user has already claimed this distribution
            if previous_claim_for_address_for_distribution.is_some() {
                return Ok(Uint128::zero());
            }

            Ok(Uint128::try_from(
                Decimal256::from(*percentage)
                    .checked_mul(Decimal256::from_ratio(
                        Uint256::from_uint128(total_claimable_amount),
                        Uint256::one(),
                    ))?
                    .to_uint_floor(),
            )?)
        }
    }
}

/// Returns the compensation for rounding errors if the distribution types have ended. This is to claim
/// the potential remaining dust in the campaign for the user due to rounding errors.
fn get_compensation_for_rounding_errors(
    campaign: &Campaign,
    current_time: &Timestamp,
    total_claimable_amount: Uint128,
    previous_claims_for_address: HashMap<DistributionSlot, Claim>,
    new_claims: &HashMap<DistributionSlot, Claim>,
) -> Result<(Uint128, DistributionSlot), ContractError> {
    if distribution_types_ended(campaign, current_time) {
        let updated_claims = aggregate_claims(&previous_claims_for_address, new_claims)?;

        let total_claimed = updated_claims
            .iter()
            .fold(Uint128::zero(), |acc, (_, (amount, _))| {
                acc.checked_add(*amount).unwrap()
            });

        // get user dust to claim
        let (slot, _) = new_claims
            .iter()
            .find(|(_, (_, timestamp))| *timestamp == current_time.seconds())
            .unwrap_or((
                &FALLBACK_DISTRIBUTION_SLOT,
                &(Uint128::zero(), Default::default()),
            ));

        return Ok((
            total_claimable_amount.saturating_sub(total_claimed),
            slot.to_owned(),
        ));
    }

    Ok((Uint128::zero(), FALLBACK_DISTRIBUTION_SLOT))
}

/// Checks if all distribution types have ended
fn distribution_types_ended(campaign: &Campaign, current_time: &Timestamp) -> bool {
    let mut distribution_types_ended = true;

    for distribution_type in campaign.distribution_type.iter() {
        match distribution_type {
            DistributionType::LinearVesting { end_time, .. } => {
                if *end_time > current_time.seconds() {
                    distribution_types_ended = false;
                }
            }
            DistributionType::LumpSum { start_time, .. } => {
                // if the lumpsum distribution has not started yet, it means it has not ended as
                // by the time this function is called, the lumpsum distribution was already
                // processed and rewards paid out
                if *start_time > current_time.seconds() {
                    distribution_types_ended = false;
                }
            }
        }
    }

    distribution_types_ended
}

/// Aggregates the new claims with the existing claims
pub fn aggregate_claims(
    previous_claims: &HashMap<DistributionSlot, Claim>,
    new_claims: &HashMap<DistributionSlot, Claim>,
) -> Result<HashMap<DistributionSlot, Claim>, ContractError> {
    let mut updated_claims = previous_claims.clone();

    for (slot, claim) in new_claims.iter() {
        let default_claim = (Uint128::zero(), 0u64);
        let previous_claim = updated_claims.get(slot).unwrap_or(&default_claim);
        let total_claimed_for_distribution_slot = previous_claim.0.checked_add(claim.0)?;
        let new_timestamp = std::cmp::max(previous_claim.1, claim.1);

        updated_claims.insert(*slot, (total_claimed_for_distribution_slot, new_timestamp));
    }
    Ok(updated_claims)
}

/// Validates the contract version and name. To be taken from mantra-std in the future, for now,
/// it's duplicated from MANTRA-dex.
#[macro_export]
macro_rules! validate_contract {
    ($deps:expr, $contract_name:expr, $contract_version:expr) => {{
        let stored_contract_name = cw2::CONTRACT.load($deps.storage)?.contract;
        cosmwasm_std::ensure!(
            stored_contract_name == $contract_name,
            cosmwasm_std::StdError::generic_err("Contract name mismatch")
        );

        let version: semver::Version = $contract_version.parse()?;
        let storage_version: semver::Version =
            cw2::get_contract_version($deps.storage)?.version.parse()?;

        cosmwasm_std::ensure!(
            storage_version < version,
            ContractError::MigrateInvalidVersion {
                current_version: storage_version,
                new_version: version,
            }
        );
    }};
}
