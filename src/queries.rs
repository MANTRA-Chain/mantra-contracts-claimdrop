use cosmwasm_std::{coin, ensure, Coin, Deps, Env, Order, Uint128};
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::msg::{
    AllocationResponse, BlacklistResponse, CampaignResponse, ClaimedResponse, DistributionType,
    RewardsResponse,
};
use crate::state::{get_allocation, is_blacklisted, CAMPAIGN, CLAIMS};

/// Returns the active airdrop campaign.
///
/// # Arguments
/// * `deps` - The dependencies
///
/// # Returns
/// * `Result<CampaignResponse, ContractError>` - The campaign information
pub fn query_campaign(deps: Deps) -> Result<CampaignResponse, ContractError> {
    let campaign = CAMPAIGN.load(deps.storage)?;
    Ok(campaign)
}

/// Returns the rewards information for a specific address.
/// This includes claimed, pending, and available to claim amounts.
///
/// # Arguments
/// * `deps` - The dependencies
/// * `env` - The environment
/// * `receiver` - The address to get rewards for
///
/// # Returns
/// * `Result<RewardsResponse, ContractError>` - The rewards information
pub fn query_rewards(
    deps: Deps,
    env: Env,
    receiver: String,
) -> Result<RewardsResponse, ContractError> {
    let campaign = CAMPAIGN.load(deps.storage)?;
    let receiver = deps.api.addr_validate(&receiver)?.to_string();

    // Check if address is blacklisted
    ensure!(
        !is_blacklisted(deps, &receiver)?,
        ContractError::AddressBlacklisted {}
    );

    // Get allocation for the address
    let allocation = get_allocation(deps, &receiver)?;
    ensure!(
        allocation.is_some(),
        ContractError::NoAllocationFound {
            address: receiver.clone(),
        }
    );

    let allocation = allocation.unwrap();

    //todo reuse helper function from claims??

    // Calculate claimable amount based on distribution type
    let mut claimed = Uint128::zero();
    let mut pending = Uint128::zero();
    let mut available_to_claim = Uint128::zero();

    for (idx, dist_type) in campaign.distribution_type.iter().enumerate() {
        let claimed_for_type = CLAIMS
            .may_load(deps.storage, receiver.clone())?
            .unwrap_or_default()
            .get(&idx)
            .map(|(amount, _)| *amount)
            .unwrap_or_else(Uint128::zero);

        let amount = match dist_type {
            DistributionType::LinearVesting {
                percentage,
                start_time,
                end_time,
                cliff_duration,
            } => {
                let total = allocation.checked_mul(percentage.atomics())?;
                let elapsed = env.block.time.seconds() - start_time;
                let duration = end_time - start_time;

                if let Some(cliff) = cliff_duration {
                    if elapsed < *cliff {
                        Uint128::zero()
                    } else {
                        total.multiply_ratio(elapsed, duration)
                    }
                } else {
                    total.multiply_ratio(elapsed, duration)
                }
            }
            DistributionType::LumpSum { percentage, .. } => {
                allocation.checked_mul(percentage.atomics())?
            }
        };

        claimed = claimed.checked_add(claimed_for_type)?;
        pending = pending.checked_add(amount)?;
        available_to_claim =
            available_to_claim.checked_add(amount.checked_sub(claimed_for_type)?)?;
        available_to_claim.checked_add(amount.checked_sub(claimed_for_type)?)?;
    }

    Ok(RewardsResponse {
        claimed: vec![Coin {
            denom: campaign.reward_denom.clone(),
            amount: claimed,
        }],
        pending: vec![Coin {
            denom: campaign.reward_denom.clone(),
            amount: pending,
        }],
        available_to_claim: vec![Coin {
            denom: campaign.reward_denom,
            amount: available_to_claim,
        }],
    })
}

// settings for pagination
pub(crate) const MAX_LIMIT: u16 = 5_000;
const DEFAULT_LIMIT: u16 = 100;

/// Returns the claimed amounts for addresses.
/// Can be filtered by a specific address and paginated.
///
/// # Arguments
/// * `deps` - The dependencies
/// * `address` - Optional address to filter by
/// * `start_from` - Optional address to start pagination from
/// * `limit` - Optional limit for pagination
///
/// # Returns
/// * `Result<ClaimedResponse, ContractError>` - The claimed amounts
pub(crate) fn query_claimed(
    deps: Deps,
    address: Option<String>,
    start_from: Option<String>,
    limit: Option<u16>,
) -> Result<ClaimedResponse, ContractError> {
    let mut claimed = vec![];

    let campaign = CAMPAIGN.may_load(deps.storage)?;

    // returns empty if the campaign is not set
    if campaign.is_none() {
        return Ok(ClaimedResponse { claimed });
    }

    if let Some(address) = address {
        let address = deps.api.addr_validate(&address)?.to_string();
        let claims = CLAIMS.may_load(deps.storage, address.clone())?;

        if let Some(claims) = claims {
            //iterate in hashmap and aggregate amount from claim
            let total_claimed = claims
                .iter()
                .fold(Uint128::zero(), |acc, (_, (amount, _))| {
                    acc.checked_add(*amount).unwrap()
                });

            if total_claimed > Uint128::zero() {
                let denom = CAMPAIGN.load(deps.storage)?.reward_denom.clone();
                claimed.push((address, coin(total_claimed.u128(), denom)));
            }
        }
    } else {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_from.map(Bound::exclusive);

        let denom = CAMPAIGN.load(deps.storage)?.reward_denom.clone();

        CLAIMS
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| {
                let (address, claims) = item?;
                //iterate in hashmap and aggregate amount from claim
                let total_claimed = claims
                    .iter()
                    .fold(Uint128::zero(), |acc, (_, (amount, _))| {
                        acc.checked_add(*amount).unwrap()
                    });

                Ok((address, coin(total_claimed.u128(), denom.clone())))
            })
            .collect::<Result<Vec<(String, Coin)>, ContractError>>()?
            .into_iter()
            .filter(|(_, coin)| coin.amount > Uint128::zero())
            .for_each(|claim| claimed.push(claim));
    }

    Ok(ClaimedResponse { claimed })
}

/// Returns the allocation for an address.
///
/// # Arguments
/// * `deps` - The dependencies
/// * `address` - The address to get allocation for
///
/// # Returns
/// * `Result<AllocationResponse, ContractError>` - The allocation information
pub fn query_allocation(deps: Deps, address: String) -> Result<AllocationResponse, ContractError> {
    let allocation = get_allocation(deps, &address)?;
    Ok(AllocationResponse { allocation })
}

/// Returns whether an address is blacklisted.
///
/// # Arguments
/// * `deps` - The dependencies
/// * `address` - The address to check
///
/// # Returns
/// * `Result<BlacklistResponse, ContractError>` - The blacklist status
pub fn query_is_blacklisted(
    deps: Deps,
    address: String,
) -> Result<BlacklistResponse, ContractError> {
    let is_blacklisted = is_blacklisted(deps, &address)?;
    Ok(BlacklistResponse { is_blacklisted })
}
