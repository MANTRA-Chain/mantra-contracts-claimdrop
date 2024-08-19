use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{Campaign, CampaignParams};
use crate::state::CAMPAIGN_COUNT;

pub(crate) fn create_campaign(
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

    let campaign = Campaign::default();

    CAMPAIGN_COUNT.save(deps.storage, &campaign_id)?;

    Ok(Response::default().add_attributes(vec![
        ("action", "create_campaign".to_string()),
        ("campaign", campaign.to_string()),
    ]))
}
