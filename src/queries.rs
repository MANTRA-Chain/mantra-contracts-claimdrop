use cosmwasm_std::{coin, Coin, Deps, Env, Order, Uint128};
use cw_storage_plus::Bound;

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{CampaignResponse, ClaimedResponse, RewardsResponse};
use crate::state::{get_total_claims_amount_for_address, CAMPAIGN, CLAIMS};

/// Returns the active airdrop campaign.
pub(crate) fn query_campaign(deps: Deps) -> Result<CampaignResponse, ContractError> {
    Ok(CAMPAIGN.load(deps.storage)?)
}

pub(crate) fn query_rewards(
    deps: Deps,
    env: Env,
    total_claimable_amount: Uint128,
    receiver: String,
    proof: Vec<String>,
) -> Result<RewardsResponse, ContractError> {
    let campaign = CAMPAIGN
        .may_load(deps.storage)?
        .ok_or(ContractError::CampaignError {
            reason: "there's not an active campaign".to_string(),
        })?;

    let mut available_to_claim = vec![];
    let mut claimed = vec![];
    let mut pending = vec![];

    let receiver = deps.api.addr_validate(&receiver)?;

    let total_claimed = get_total_claims_amount_for_address(deps, &receiver)?;
    if total_claimed > Uint128::zero() {
        claimed.push(coin(total_claimed.u128(), &campaign.reward_asset.denom));
    }

    if campaign.has_funds_available() {
        let pending_rewards = coin(
            total_claimable_amount.saturating_sub(total_claimed).u128(),
            &campaign.reward_asset.denom,
        );

        if pending_rewards.amount > Uint128::zero() {
            pending.push(pending_rewards);
        }

        helpers::validate_claim(
            &env.contract.address,
            &receiver,
            total_claimable_amount,
            &proof,
            &campaign.merkle_root,
        )?;
        let (claimable_amount, _) = helpers::compute_claimable_amount(
            deps,
            &campaign,
            &env.block.time,
            &receiver,
            total_claimable_amount,
        )?;

        if claimable_amount.amount > Uint128::zero() {
            available_to_claim.push(claimable_amount);
        }
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
                let denom = CAMPAIGN.load(deps.storage)?.reward_asset.denom.clone();
                claimed.push((address, coin(total_claimed.u128(), denom)));
            }
        }
    } else {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_from.map(Bound::exclusive);

        let denom = CAMPAIGN.load(deps.storage)?.reward_asset.denom.clone();

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
