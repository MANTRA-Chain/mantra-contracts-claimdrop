use cosmwasm_std::{coin, Deps, Env, Uint128};

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{CampaignFilter, CampaignsResponse, RewardsResponse};
use crate::state::{
    get_campaign_by_id, get_campaigns, get_campaigns_by_owner, get_total_claims_amount_for_address,
};

/// Returns a list of campaigns based on the provided filter.
pub(crate) fn query_campaigns(
    deps: Deps,
    campaign_filter: Option<CampaignFilter>,
    start_from: Option<u64>,
    limit: Option<u8>,
) -> Result<CampaignsResponse, ContractError> {
    //do the same as above but matching if campaign_filter is some
    let campaigns = if let Some(campaign_filter) = campaign_filter {
        match campaign_filter {
            CampaignFilter::Owner(owner) => {
                deps.api.addr_validate(&owner)?;
                get_campaigns_by_owner(deps.storage, owner)?
            }
            CampaignFilter::CampaignId(campaign_id) => {
                vec![get_campaign_by_id(deps.storage, campaign_id)?]
            }
        }
    } else {
        get_campaigns(deps.storage, start_from, limit)?
    };

    Ok(CampaignsResponse { campaigns })
}

pub(crate) fn query_rewards(
    deps: Deps,
    env: Env,
    campaign_id: u64,
    total_claimable_amount: Uint128,
    receiver: String,
    proof: Vec<String>,
) -> Result<RewardsResponse, ContractError> {
    let campaign = get_campaign_by_id(deps.storage, campaign_id)?;
    let mut available_to_claim = vec![];
    let mut claimed = vec![];
    let mut pending = vec![];
    println!(">>>> query rewards");

    println!("campaign.endtime: {:?}", campaign.end_time);
    println!("current time: {:?}", env.block.time.seconds());
    let receiver = deps.api.addr_validate(&receiver)?;

    let total_claimed = get_total_claims_amount_for_address(deps, campaign_id, &receiver)?;
    if total_claimed > Uint128::zero() {
        claimed.push(coin(total_claimed.u128(), &campaign.reward_asset.denom));
    }

    if !campaign.is_closed() {
        let pending_rewards = coin(
            total_claimable_amount.saturating_sub(total_claimed).u128(),
            &campaign.reward_asset.denom,
        );

        if pending_rewards.amount > Uint128::zero() {
            pending.push(pending_rewards);
        }

        helpers::validate_claim(&campaign, &receiver, total_claimable_amount, &proof)?;
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
