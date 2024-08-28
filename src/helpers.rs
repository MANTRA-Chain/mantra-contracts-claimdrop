use std::collections::HashMap;

use cosmwasm_std::{
    ensure, Addr, Coin, Decimal, DepsMut, MessageInfo, StdError, Timestamp, Uint128,
};
use sha2::Digest;

use crate::error::ContractError;
use crate::msg::{Campaign, CampaignParams, DistributionType};
use crate::state::{get_claims_for_address, Claim, DistributionSlot};

/// Validates the campaign parameters
pub(crate) fn validate_campaign_params(
    current_time: Timestamp,
    info: &MessageInfo,
    campaign_params: &CampaignParams,
) -> Result<(), ContractError> {
    campaign_params.validate_campaign_name_description()?;
    validate_merkle_root(&campaign_params.merkle_root)?;
    campaign_params.validate_campaign_distribution(current_time)?;
    campaign_params.validate_campaign_times(current_time)?;
    campaign_params.validate_cliff_duration()?;

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
pub(crate) fn validate_merkle_root(merkle_root: &String) -> Result<[u8; 32], ContractError> {
    let mut merkle_root_buf: [u8; 32] = [0; 32];
    hex::decode_to_slice(merkle_root, &mut merkle_root_buf)?;

    Ok(merkle_root_buf)
}

pub(crate) fn validate_claim(
    campaign: &Campaign,
    sender: &Addr,
    amount: Uint128,
    proof: &[String],
) -> Result<(), ContractError> {
    let user_input = format!("{}{}{}", campaign.id, sender, amount);
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

    let merkle_root_buf = validate_merkle_root(&campaign.merkle_root)?;

    ensure!(
        merkle_root_buf == hash,
        ContractError::MerkleRootVerificationFailed
    );

    Ok(())
}

const FALLBACK_TIME: u64 = 0u64;
const FALLBACK_DISTRIBUTION_SLOT: usize = 0usize;

/// Calculates the amount a user can claim at this point in time
pub(crate) fn compute_claimable_amount(
    deps: &DepsMut,
    campaign: &Campaign,
    current_time: &Timestamp,
    address: &Addr,
    total_claimable_amount: Uint128,
) -> Result<(Coin, HashMap<DistributionSlot, Claim>), ContractError> {
    let mut claimable_amount = Uint128::zero();
    let mut new_claims = HashMap::new();

    if campaign.has_started(current_time) {
        let previous_claims_for_address =
            get_claims_for_address(deps.as_ref(), campaign.id, address)?;

        for (distribution_slot, distribution) in
            campaign.distribution_type.iter().enumerate().clone()
        {
            println!("dist: {:?}", distribution);
            // skip distributions that have not started yet
            if !distribution.has_started(current_time) {
                println!("distribution not active");
                continue;
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

            println!("{} - claim_amount: {}", distribution_slot, claim_amount);

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

            let (amount, _) = new_claims
                .get_mut(&slot)
                .ok_or(StdError::generic_err("couldn't find claim"))?;
            *amount = amount.checked_add(rounding_error_compensation_amount)?;
        }
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
        } => {
            let elapsed_time =
                if let Some((_, last_claimed)) = previous_claim_for_address_for_distribution {
                    println!("current_time: {}", current_time.seconds());
                    println!("end_time: {}", end_time);
                    println!("last_claimed: {}", last_claimed);

                    if last_claimed > end_time {
                        FALLBACK_TIME
                    } else {
                        current_time.seconds().min(end_time.to_owned()) - last_claimed
                    }
                } else {
                    current_time.seconds().min(end_time.to_owned()) - start_time
                };

            println!("computing LinearVesting");

            let vesting_duration = end_time - start_time;
            let vesting_progress = Decimal::from_ratio(elapsed_time, vesting_duration);

            println!("elapsed_time: {}", elapsed_time);
            println!("vesting_duration: {}", vesting_duration);
            println!("vesting_progress: {}", vesting_progress);

            Ok(percentage
                .checked_mul(Decimal::from_ratio(total_claimable_amount, Uint128::one()))?
                .checked_mul(vesting_progress)?
                .to_uint_floor())
        }
        DistributionType::PeriodicVesting { .. } => unimplemented!(),
        DistributionType::LumpSum { percentage, .. } => {
            // it means the user has already claimed this distribution
            if previous_claim_for_address_for_distribution.is_some() {
                return Ok(Uint128::zero());
            }

            println!("computing LumpSum");

            Ok(percentage
                .checked_mul(Decimal::from_ratio(total_claimable_amount, Uint128::one()))?
                .to_uint_floor())
        }
    }
}

fn get_compensation_for_rounding_errors(
    campaign: &Campaign,
    current_time: &Timestamp,
    total_claimable_amount: Uint128,
    previous_claims_for_address: HashMap<DistributionSlot, Claim>,
    new_claims: &HashMap<DistributionSlot, Claim>,
) -> Result<(Uint128, DistributionSlot), ContractError> {
    if campaign.has_ended(current_time) {
        let updated_claims = aggregate_claims(&previous_claims_for_address, new_claims)?;

        println!("updated_claims: {:?}", updated_claims);

        let total_claimed = updated_claims
            .iter()
            .fold(Uint128::zero(), |acc, (_, (amount, _))| {
                acc.checked_add(*amount).unwrap()
            });

        println!("total_claimed loop: {}", total_claimed);

        // if the campaign has ended and the user still has dust to claim
        if total_claimed < total_claimable_amount {
            println!(
                "total_amount.saturating_sub(total_claimed): {}",
                total_claimable_amount.saturating_sub(total_claimed)
            );

            let (slot, _) = new_claims
                .iter()
                .find(|(_, (_, timestamp))| *timestamp == current_time.seconds())
                .unwrap_or((
                    &FALLBACK_DISTRIBUTION_SLOT,
                    &(Uint128::zero(), FALLBACK_TIME),
                ));
            return Ok((
                total_claimable_amount.saturating_sub(total_claimed),
                slot.to_owned(),
            ));
        }
    }

    Ok((Uint128::zero(), FALLBACK_DISTRIBUTION_SLOT))
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
