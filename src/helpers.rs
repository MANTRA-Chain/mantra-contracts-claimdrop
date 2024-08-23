use cosmwasm_std::{ensure, Addr, Coin, Decimal, MessageInfo, Timestamp, Uint128};
use sha2::Digest;

use crate::error::ContractError;
use crate::msg::{Campaign, CampaignParams, DistributionType};

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
    proof: &Vec<String>,
) -> Result<(), ContractError> {
    let user_input = format!("{}{}{}", campaign.id, sender, amount);
    let hash = sha2::Sha256::digest(user_input.as_bytes())
        .as_slice()
        .try_into()
        .map_err(|_| ContractError::WrongHashLength)?;

    let hash = proof.into_iter().try_fold(hash, |hash, p| {
        let mut proof_buf = [0; 32];
        hex::decode_to_slice(p, &mut proof_buf)?;
        let mut hashes = [hash, proof_buf];
        hashes.sort_unstable();
        sha2::Sha256::digest(&hashes.concat())
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
    campaign: &Campaign,
    current_time: &Timestamp,
    address: &Addr,
    total_amount: Uint128,
) -> Result<Coin, ContractError> {
    let mut claimable_amount = Uint128::zero();

    //todo will need to store in state the amount claimed by each address when there's linear or periodic vesting,
    // alongside with information such as what periods they already claimed, the last time they claimed and so on.

    for dist in campaign.distribution_type.clone() {
        match dist {
            DistributionType::LinearVesting {
                percentage,
                start_time,
                end_time,
            } => {}
            DistributionType::PeriodicVesting {
                percentage,
                start_time,
                end_time,
                period_duration,
            } => {}
            DistributionType::LumpSum {
                percentage,
                start_time,
                end_time,
            } => {
                if start_time > current_time.seconds() && end_time < current_time.seconds() {
                    claimable_amount = claimable_amount.checked_add(
                        percentage
                            .checked_mul(Decimal::from_ratio(total_amount, Uint128::zero()))?
                            .to_uint_floor(),
                    )?;
                }
            }
        }
    }

    Ok(Coin {
        denom: campaign.reward_asset.denom.clone(),
        amount: claimable_amount,
    })
}
