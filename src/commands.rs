use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{Campaign, CampaignAction, CampaignParams};
use crate::state::{CAMPAIGNS, CAMPAIGN_COUNT};

/// Manages a campaign
pub(crate) fn manage_campaign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    campaign_action: CampaignAction,
) -> Result<Response, ContractError> {
    match campaign_action {
        CampaignAction::CreateCampaign { params } => create_campaign(deps, env, info, params),
        CampaignAction::EndCampaign { campaign_id } => end_campaign(deps, info, campaign_id),
    }
}

/// Creates a new airdrop campaign.
fn create_campaign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    campaign_params: CampaignParams,
) -> Result<Response, ContractError> {
    let campaign_id = CAMPAIGN_COUNT
        .may_load(deps.storage)?
        .unwrap_or_default()
        .checked_add(1u64)
        .ok_or(ContractError::Overflow)?;

    helpers::validate_campaign_params(env.block.time, &info, &campaign_params)?;

    let owner = campaign_params
        .owner
        .as_ref()
        .map(|addr| deps.api.addr_validate(addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    let campaign = Campaign::from_params(campaign_params, campaign_id, owner);
    CAMPAIGN_COUNT.save(deps.storage, &campaign_id)?;
    CAMPAIGNS.save(deps.storage, campaign_id, &campaign)?;

    Ok(Response::default().add_attributes(vec![
        ("action", "create_campaign".to_string()),
        ("campaign", campaign.to_string()),
    ]))
}

/// Ends an airdrop campaign. Only the owner or the contract admin can end a campaign. The remaining
/// funds in the campaign are refunded to the owner.
fn end_campaign(
    _deps: DepsMut,
    _info: MessageInfo,
    _campaign_id: u64,
) -> Result<Response, ContractError> {
    unimplemented!()
}
