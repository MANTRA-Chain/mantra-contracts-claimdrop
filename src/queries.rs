use cosmwasm_std::{coin, Deps, Env, Uint128};

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{CampaignResponse, RewardsResponse};
use crate::state::{get_total_claims_amount_for_address, CAMPAIGN};

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
    let campaign = CAMPAIGN.load(deps.storage)?;
    let mut available_to_claim = vec![];
    let mut claimed = vec![];
    let mut pending = vec![];
    println!(">>>> query rewards");

    println!("campaign.endtime: {:?}", campaign.end_time);
    println!("current time: {:?}", env.block.time.seconds());
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

    Ok(RewardsResponse {
        claimed,
        pending,
        available_to_claim,
    })
}
