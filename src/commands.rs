use cosmwasm_std::{coins, ensure, BankMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{Campaign, CampaignAction, CampaignParams};
use crate::state::{
    get_claims_for_address, get_total_claims_amount_for_address, CAMPAIGNS, CAMPAIGN_COUNT, CLAIMS,
};

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

    //todo potentially end those campaigns that have expired after X months?

    Ok(Response::default().add_attributes(vec![
        ("action", "create_campaign".to_string()),
        ("campaign", campaign.to_string()),
    ]))
}

/// Ends an airdrop campaign. Only the owner or the contract admin can end a campaign. The remaining
/// funds in the campaign are refunded to the owner.
fn end_campaign(
    deps: DepsMut,
    info: MessageInfo,
    campaign_id: u64,
) -> Result<Response, ContractError> {
    cw_utils::nonpayable(&info)?;

    let mut campaign = CAMPAIGNS
        .may_load(deps.storage, campaign_id)?
        .ok_or(ContractError::CampaignNotFound { campaign_id })?;

    ensure!(
        campaign.owner == info.sender || cw_ownable::is_owner(deps.storage, &info.sender)?,
        ContractError::Unauthorized
    );

    //todo grace period to close a campaign once it finishes???

    let refund = campaign
        .reward_asset
        .amount
        .checked_sub(campaign.claimed.amount)?;

    let mut messages = vec![];

    if !refund.is_zero() {
        messages.push(BankMsg::Send {
            to_address: campaign.owner.to_string(),
            amount: coins(refund.u128(), campaign.reward_asset.denom.clone()),
        });
    }

    // Set the claimed amount to the total reward amount, so that the campaign is considered finished.
    campaign.claimed = campaign.reward_asset.clone();

    CAMPAIGNS.save(deps.storage, campaign.id, &campaign)?;

    Ok(Response::default()
        .add_messages(messages)
        .add_attributes(vec![
            ("action", "end_campaign".to_string()),
            ("campaign_id", campaign_id.to_string()),
        ]))
}

pub(crate) fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    campaign_id: u64,
    total_claimable_amount: Uint128,
    receiver: Option<String>,
    proof: Vec<String>,
    //todo make receiver optional so we can make a contract/gas station pay for the fees
) -> Result<Response, ContractError> {
    cw_utils::nonpayable(&info)?;

    let mut campaign = CAMPAIGNS
        .may_load(deps.storage, campaign_id)?
        .ok_or(ContractError::CampaignNotFound { campaign_id })?;

    ensure!(
        campaign.has_started(&env.block.time),
        ContractError::CampaignTimeMismatch {
            reason: "not started".to_string()
        }
    );

    let receiver = receiver
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    helpers::validate_claim(&campaign, &receiver, total_claimable_amount, &proof)?;

    let (claimable_amount, new_claims) = helpers::compute_claimable_amount(
        &deps,
        &campaign,
        &env.block.time,
        &receiver,
        total_claimable_amount,
    )?;

    println!("claimable_amount: {:?}", claimable_amount);

    ensure!(
        claimable_amount.amount > Uint128::zero(),
        ContractError::NothingToClaim
    );

    let previous_claims = get_claims_for_address(deps.as_ref(), campaign_id, &receiver)?;

    println!("new_claims: {:?}", new_claims);
    println!("previous_claims: {:?}", previous_claims);

    let updated_claims = helpers::aggregate_claims(&previous_claims, &new_claims)?;

    println!("updated_claims: {:?}", updated_claims);

    campaign.claimed.amount = campaign
        .claimed
        .amount
        .checked_add(claimable_amount.amount)?;

    ensure!(
        campaign.claimed.amount <= campaign.reward_asset.amount,
        ContractError::ExceededMaxClaimAmount
    );

    CAMPAIGNS.save(deps.storage, campaign.id, &campaign)?;
    CLAIMS.save(
        deps.storage,
        (receiver.to_string(), campaign.id),
        &updated_claims,
    )?;

    let x = get_total_claims_amount_for_address(deps.as_ref(), campaign.id, &receiver)?;
    println!("total claims for user: {:?}", x);

    // final sanity check to make sure the address can't claim more than the total amount it's entitled to
    ensure!(
        total_claimable_amount
            >= get_total_claims_amount_for_address(deps.as_ref(), campaign.id, &receiver)?,
        ContractError::ExceededMaxClaimAmount
    );

    Ok(Response::default()
        .add_message(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: vec![claimable_amount.clone()],
        })
        .add_attributes(vec![
            ("action", "claim".to_string()),
            ("campaign_id", campaign_id.to_string()),
            ("receiver", receiver.to_string()),
            ("claimed_amount", claimable_amount.to_string()),
        ]))
}
