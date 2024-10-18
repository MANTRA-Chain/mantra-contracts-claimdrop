use cosmwasm_std::{coins, ensure, BankMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{Campaign, CampaignAction, CampaignParams};
use crate::state::{get_claims_for_address, get_total_claims_amount_for_address, CAMPAIGN, CLAIMS};

/// Manages a campaign
pub(crate) fn manage_campaign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    campaign_action: CampaignAction,
) -> Result<Response, ContractError> {
    match campaign_action {
        CampaignAction::CreateCampaign { params } => create_campaign(deps, env, info, *params),
        CampaignAction::TopUpCampaign {} => topup_campaign(deps, env, info),
        CampaignAction::CloseCampaign {} => close_campaign(deps, env, info),
    }
}

/// Creates a new airdrop campaign.
fn create_campaign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    campaign_params: CampaignParams,
) -> Result<Response, ContractError> {
    // only the owner of the contract can create a campaign
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    let campaign = CAMPAIGN.may_load(deps.storage)?;

    ensure!(
        campaign.is_none(),
        ContractError::CampaignError {
            reason: "existing campaign".to_string()
        }
    );

    helpers::validate_campaign_params(env.block.time, &info, &campaign_params)?;

    let owner = campaign_params
        .owner
        .as_ref()
        .map(|addr| deps.api.addr_validate(addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    let campaign = Campaign::from_params(campaign_params, owner);

    CAMPAIGN.save(deps.storage, &campaign)?;

    Ok(Response::default().add_attributes(vec![
        ("action", "create_campaign".to_string()),
        ("campaign", campaign.to_string()),
    ]))
}

/// Tops up an existing airdrop campaign.
fn topup_campaign(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    let mut campaign = CAMPAIGN
        .may_load(deps.storage)?
        .ok_or(ContractError::CampaignError {
            reason: "there's not an active campaign".to_string(),
        })?;

    ensure!(campaign.owner == info.sender, ContractError::Unauthorized);

    ensure!(
        campaign.end_time > env.block.time.seconds(),
        ContractError::CampaignError {
            reason: "campaign has ended".to_string()
        }
    );

    ensure!(
        campaign.closed.is_none(),
        ContractError::CampaignError {
            reason: "campaign has been closed".to_string()
        }
    );

    let topup = cw_utils::must_pay(&info, &campaign.reward_asset.denom)?;
    campaign.reward_asset.amount = campaign.reward_asset.amount.checked_add(topup)?;

    CAMPAIGN.save(deps.storage, &campaign)?;

    Ok(Response::default().add_attributes(vec![
        ("action", "topup_campaign".to_string()),
        ("topup", topup.to_string()),
        ("campaign", campaign.to_string()),
    ]))
}

/// Closes the existing airdrop campaign. Only the owner or the contract admin can end the campaign.
/// The remaining funds in the campaign are refunded to the owner.
fn close_campaign(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    cw_utils::nonpayable(&info)?;

    let mut campaign = CAMPAIGN
        .may_load(deps.storage)?
        .ok_or(ContractError::CampaignError {
            reason: "there's not an active campaign".to_string(),
        })?;

    ensure!(
        campaign.owner == info.sender || cw_ownable::is_owner(deps.storage, &info.sender)?,
        ContractError::Unauthorized
    );

    ensure!(
        campaign.closed.is_none(),
        ContractError::CampaignError {
            reason: "campaign has already been closed".to_string()
        }
    );

    let refund = campaign
        .reward_asset
        .amount
        .saturating_sub(campaign.claimed.amount);

    let mut messages = vec![];

    if !refund.is_zero() {
        messages.push(BankMsg::Send {
            to_address: campaign.owner.to_string(),
            amount: coins(refund.u128(), campaign.reward_asset.denom.clone()),
        });
    }

    campaign.closed = Some(env.block.time.seconds());

    CAMPAIGN.save(deps.storage, &campaign)?;

    Ok(Response::default()
        .add_messages(messages)
        .add_attributes(vec![
            ("action", "close_campaign".to_string()),
            ("campaign", campaign.to_string()),
            ("refund", refund.to_string()),
        ]))
}

pub(crate) fn claim(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    total_claimable_amount: Uint128,
    receiver: Option<String>,
    proof: Vec<String>,
) -> Result<Response, ContractError> {
    cw_utils::nonpayable(&info)?;

    let mut campaign = CAMPAIGN
        .may_load(deps.storage)?
        .ok_or(ContractError::CampaignError {
            reason: "there's not an active campaign".to_string(),
        })?;

    ensure!(
        campaign.has_started(&env.block.time),
        ContractError::CampaignError {
            reason: "not started".to_string()
        }
    );

    ensure!(
        campaign.closed.is_none(),
        ContractError::CampaignError {
            reason: "has been closed, cannot claim".to_string()
        }
    );

    ensure!(
        campaign.has_funds_available(),
        ContractError::CampaignError {
            reason: "no funds available to claim".to_string()
        }
    );

    let receiver = receiver
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    helpers::validate_claim(
        &env.contract.address,
        &receiver,
        total_claimable_amount,
        &proof,
        &campaign.merkle_root,
    )?;

    let (claimable_amount, new_claims) = helpers::compute_claimable_amount(
        deps.as_ref(),
        &campaign,
        &env.block.time,
        &receiver,
        total_claimable_amount,
    )?;

    ensure!(
        claimable_amount.amount > Uint128::zero(),
        ContractError::NothingToClaim
    );

    let previous_claims = get_claims_for_address(deps.as_ref(), &receiver)?;

    let updated_claims = helpers::aggregate_claims(&previous_claims, &new_claims)?;

    campaign.claimed.amount = campaign
        .claimed
        .amount
        .checked_add(claimable_amount.amount)?;

    ensure!(
        campaign.claimed.amount <= campaign.reward_asset.amount,
        ContractError::ExceededMaxClaimAmount
    );

    CAMPAIGN.save(deps.storage, &campaign)?;

    CLAIMS.save(deps.storage, receiver.to_string(), &updated_claims)?;

    ensure!(
        total_claimable_amount >= get_total_claims_amount_for_address(deps.as_ref(), &receiver)?,
        ContractError::ExceededMaxClaimAmount
    );

    Ok(Response::default()
        .add_message(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: vec![claimable_amount.clone()],
        })
        .add_attributes(vec![
            ("action", "claim".to_string()),
            ("receiver", receiver.to_string()),
            ("claimed_amount", claimable_amount.to_string()),
        ]))
}
