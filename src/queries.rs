use cosmwasm_std::{coin, Coin, Deps, Env, Order, StdResult, Uint128};
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{
    AllocationsResponse, BlacklistResponse, CampaignResponse, ClaimedResponse, RewardsResponse,
};
use crate::state::{
    get_allocation, get_total_claims_amount_for_address, is_blacklisted, ALLOCATIONS, CAMPAIGN,
    CLAIMS,
};

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
pub(crate) fn query_rewards(
    deps: Deps,
    env: Env,
    receiver: String,
) -> Result<RewardsResponse, ContractError> {
    let campaign = CAMPAIGN
        .may_load(deps.storage)?
        .ok_or(ContractError::CampaignError {
            reason: "there's not an active campaign".to_string(),
        })?;

    let mut available_to_claim = vec![];
    let mut claimed = vec![];
    let mut pending = vec![];

    let validated_receiver_string = helpers::validate_raw_address(deps, &receiver)?;

    let total_claimable_amount = get_allocation(deps, validated_receiver_string.as_str())?.ok_or(
        ContractError::NoAllocationFound {
            address: receiver.to_string(),
        },
    )?;

    let total_claimed: Uint128 =
        get_total_claims_amount_for_address(deps, validated_receiver_string.as_str())?;
    if total_claimed > Uint128::zero() {
        claimed.push(coin(total_claimed.u128(), &campaign.reward_denom));
    }

    let pending_rewards = coin(
        total_claimable_amount.saturating_sub(total_claimed).u128(),
        &campaign.reward_denom,
    );

    if pending_rewards.amount > Uint128::zero() {
        pending.push(pending_rewards);
    }

    let (claimable_amount, _) = helpers::compute_claimable_amount(
        deps,
        &campaign,
        &env.block.time,
        &validated_receiver_string,
        total_claimable_amount,
    )?;

    if claimable_amount.amount > Uint128::zero() {
        available_to_claim.push(claimable_amount);
    }

    // if the campaign is closed, clear the pending and available to claim rewards as there's nothing else
    // to claim
    if campaign.closed.is_some() {
        pending.clear();
        available_to_claim.clear();
    }

    Ok(RewardsResponse {
        claimed,
        pending,
        available_to_claim,
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

const MAX_ALLOCATIONS: u16 = 5_000;
const DEFAULT_ALLOCATIONS_LIMIT: u16 = 100;

/// Returns the allocation for an address.
///
/// # Arguments
/// * `deps` - The dependencies
/// * `address` - Optional address to filter by
/// * `start_after` - Optional address to start pagination from
/// * `limit` - Optional limit for pagination
///
/// # Returns
/// * `Result<AllocationsResponse, ContractError>` - The allocations information
pub fn query_allocation(
    deps: Deps,
    address: Option<String>,
    start_after: Option<String>,
    limit: Option<u16>,
) -> Result<AllocationsResponse, ContractError> {
    let allocations = if let Some(address) = address {
        let allocation = get_allocation(deps, &address)?;
        if let Some(allocation) = allocation {
            vec![(address, allocation)]
        } else {
            vec![]
        }
    } else {
        let limit = limit
            .unwrap_or(DEFAULT_ALLOCATIONS_LIMIT)
            .min(MAX_ALLOCATIONS) as usize;
        let start = cw_utils::calc_range_start_string(start_after).map(Bound::ExclusiveRaw);

        ALLOCATIONS
            .range(deps.storage, start, None, Order::Ascending)
            .take(limit)
            .map(|item| {
                let (address, allocation) = item?;
                Ok((address, allocation))
            })
            .collect::<StdResult<Vec<(String, Uint128)>>>()?
    };

    Ok(AllocationsResponse { allocations })
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
