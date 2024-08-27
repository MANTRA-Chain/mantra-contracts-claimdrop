use std::collections::HashMap;

use cosmwasm_std::{ensure, Addr, Coin, Decimal, DepsMut, MessageInfo, Timestamp, Uint128};
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

/// Calculates the amount a user can claim at this point in time
pub(crate) fn compute_claimable_amount(
    deps: &DepsMut,
    campaign: &Campaign,
    current_time: &Timestamp,
    address: &Addr,
    total_amount: Uint128,
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
            if !distribution.has_started(current_time) {
                println!("distribution not active");
                continue;
            }

            let claim_for_distribution = previous_claims_for_address.get(&distribution_slot);

            if distribution.has_started(current_time) {
                let claim_amount = match distribution {
                    DistributionType::LinearVesting {
                        percentage,
                        start_time,
                        end_time,
                    } => {
                        let elapsed_time = if claim_for_distribution.is_some() {
                            let (_, last_claimed) = claim_for_distribution.unwrap();

                            println!("current_time: {}", current_time.seconds());
                            println!("end_time: {}", end_time);
                            println!("last_claimed: {}", last_claimed);

                            if last_claimed > end_time {
                                0u64
                            } else {
                                current_time.seconds().min(end_time.to_owned()) - last_claimed
                            }
                        } else {
                            current_time.seconds().min(end_time.to_owned()) - start_time
                        };

                        println!("computing LinearVesting");

                        println!("elapsed_time: {}", elapsed_time);
                        let vesting_duration = end_time - start_time;
                        println!("vesting_duration: {}", vesting_duration);
                        let vesting_progress = Decimal::from_ratio(elapsed_time, vesting_duration);
                        println!("vesting_progress: {}", vesting_progress);

                        percentage
                            .checked_mul(Decimal::from_ratio(total_amount, Uint128::one()))?
                            .checked_mul(vesting_progress)?
                            .to_uint_floor()
                    }
                    DistributionType::PeriodicVesting { .. } => Uint128::zero(),
                    DistributionType::LumpSum { percentage, .. } => {
                        // it means the user has already claimed this distribution
                        if claim_for_distribution.is_some() {
                            continue;
                        }

                        println!("computing LumpSum");

                        percentage
                            .checked_mul(Decimal::from_ratio(total_amount, Uint128::one()))?
                            .to_uint_floor()
                    }
                };

                println!("{} - claim_amount: {}", distribution_slot, claim_amount);

                claimable_amount = claimable_amount.checked_add(claim_amount)?;

                new_claims.insert(
                    distribution_slot,
                    (claimable_amount, current_time.seconds()),
                );
            }
        }

        // println!("claimable_amount before tweak: {}", claimable_amount);
        //
        //
        // let (already_claimed, last_claimed) = get_claims_for_address(deps, campaign.id, address)?;
        // println!("already_claimed: {}", already_claimed);
        // claimable_amount = claimable_amount.checked_sub(already_claimed)?;
        // println!("claimable_amount: {}", claimable_amount);

        // compensate for rounding errors
        if campaign.has_ended(current_time) {
            let updated_claims = aggregate_claims(&previous_claims_for_address, &new_claims)?;

            println!("updated_claims: {:?}", updated_claims);

            let total_claimed = updated_claims
                .iter()
                .fold(Uint128::zero(), |acc, (_, (amount, _))| {
                    acc.checked_add(*amount).unwrap()
                });

            println!("total_claimed loop: {}", total_claimed);

            if total_claimed < total_amount {
                println!(
                    "total_amount.saturating_sub(total_claimed): {}",
                    total_amount.saturating_sub(total_claimed)
                );

                let remaining_amount = total_amount.saturating_sub(total_claimed);
                claimable_amount = claimable_amount.checked_add(remaining_amount)?;

                for (_, (amount, timestamp)) in new_claims.iter_mut() {
                    if *timestamp == current_time.seconds() {
                        *amount = amount.checked_add(remaining_amount)?;
                    }
                }
            }
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
