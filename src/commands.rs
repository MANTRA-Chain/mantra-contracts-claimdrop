use std::collections::HashMap;

use cosmwasm_std::{ensure, BankMsg, Coin, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::error::ContractError;
use crate::helpers;
use crate::msg::{Campaign, CampaignAction, CampaignParams, DistributionType};
use crate::state::{
    get_allocation, get_claims_for_address, get_total_claims_amount_for_address, is_blacklisted,
    Claim, DistributionSlot, ALLOCATIONS, BLACKLIST, CAMPAIGN, CLAIMS,
};

/// Manages a campaign
pub(crate) fn manage_campaign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    campaign_action: CampaignAction,
) -> Result<Response, ContractError> {
    match campaign_action {
        CampaignAction::CreateCampaign { params } => create_campaign(deps, env, info, *params),
        CampaignAction::CloseCampaign {} => {
            cw_utils::nonpayable(&info)?;
            close_campaign(deps, env, info)
        }
    }
}

/// Creates a new airdrop campaign.
fn create_campaign(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    campaign_params: CampaignParams,
) -> Result<Response, ContractError> {
    // only the owner can create a campaign
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    let campaign: Option<Campaign> = CAMPAIGN.may_load(deps.storage)?;

    ensure!(
        campaign.is_none(),
        ContractError::CampaignError {
            reason: "existing campaign".to_string()
        }
    );

    helpers::validate_campaign_params(env.block.time, &campaign_params)?;

    let campaign = Campaign::from_params(campaign_params);
    CAMPAIGN.save(deps.storage, &campaign)?;

    Ok(Response::default().add_attributes(vec![
        ("action", "create_campaign".to_string()),
        ("campaign", campaign.to_string()),
    ]))
}

/// Closes the existing airdrop campaign. Only the owner can end the campaign.
/// The remaining funds in the campaign are refunded to the owner.
fn close_campaign(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    let mut campaign = CAMPAIGN
        .may_load(deps.storage)?
        .ok_or(ContractError::CampaignError {
            reason: "there's not an active campaign".to_string(),
        })?;

    ensure!(
        campaign.closed.is_none(),
        ContractError::CampaignError {
            reason: "campaign has already been closed".to_string()
        }
    );

    let refund: Coin = deps
        .querier
        .query_balance(env.contract.address, &campaign.reward_denom)?;

    let mut messages = vec![];

    if !refund.amount.is_zero() {
        let owner = cw_ownable::get_ownership(deps.storage)?.owner.unwrap();

        messages.push(BankMsg::Send {
            to_address: owner.to_string(),
            amount: vec![refund.clone()],
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
    receiver: Option<String>,
    amount: Option<Uint128>,
) -> Result<Response, ContractError> {
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

    let receiver = receiver
        .map(|addr| deps.api.addr_validate(&addr))
        .transpose()?
        .unwrap_or_else(|| info.sender.clone());

    ensure!(
        !is_blacklisted(deps.as_ref(), receiver.as_ref())?,
        ContractError::AddressBlacklisted
    );

    // Get allocation for the address
    let total_user_allocation = get_allocation(deps.as_ref(), receiver.as_ref())?.ok_or(
        ContractError::NoAllocationFound {
            address: receiver.to_string(),
        },
    )?;

    // new_claims is HashMap<DistributionSlot, Claim=(amount, timestamp)> representing newly available amounts per slot
    let (max_claimable_amount_coin, new_claims) = helpers::compute_claimable_amount(
        deps.as_ref(),
        &campaign,
        &env.block.time,
        &receiver,
        total_user_allocation,
    )?;

    let actual_claim_amount_coin = match amount {
        Some(requested_amount) => {
            ensure!(
                requested_amount > Uint128::zero(),
                ContractError::InvalidClaimAmount {
                    reason: "amount must be greater than zero".to_string()
                }
            );
            ensure!(
                requested_amount <= max_claimable_amount_coin.amount,
                ContractError::InvalidClaimAmount {
                    reason: format!(
                        "requested amount {} exceeds available claimable amount {}",
                        requested_amount, max_claimable_amount_coin.amount
                    )
                }
            );
            Coin {
                denom: campaign.reward_denom.clone(),
                amount: requested_amount,
            }
        }
        None => max_claimable_amount_coin,
    };

    ensure!(
        actual_claim_amount_coin.amount > Uint128::zero(),
        ContractError::NothingToClaim
    );

    let available_funds = deps
        .querier
        .query_balance(env.contract.address, &campaign.reward_denom)?;

    ensure!(
        actual_claim_amount_coin.amount <= available_funds.amount,
        ContractError::CampaignError {
            reason: "no funds available to claim".to_string()
        }
    );

    let previous_claims = get_claims_for_address(deps.as_ref(), receiver.to_string())?;
    let mut claims_to_record: HashMap<DistributionSlot, Claim> = HashMap::new();
    let mut remaining_to_distribute = actual_claim_amount_coin.amount;

    if remaining_to_distribute > Uint128::zero() {
        let mut lump_sum_slots_with_new_claims: Vec<DistributionSlot> = vec![];
        let mut linear_vesting_slots_with_new_claims: Vec<DistributionSlot> = vec![];

        for (idx, dist_type) in campaign.distribution_type.iter().enumerate() {
            if new_claims.contains_key(&idx) {
                // Only consider slots that have new claimable amounts
                match dist_type {
                    DistributionType::LumpSum { .. } => lump_sum_slots_with_new_claims.push(idx),
                    DistributionType::LinearVesting { .. } => {
                        linear_vesting_slots_with_new_claims.push(idx)
                    }
                }
            }
        }

        lump_sum_slots_with_new_claims.sort();
        linear_vesting_slots_with_new_claims.sort();

        // Phase 1: Distribute to LumpSum slots from new_claims
        for slot_idx in lump_sum_slots_with_new_claims {
            if remaining_to_distribute == Uint128::zero() {
                break;
            }
            // new_claims.get(&slot_idx) returns Option<&(Uint128, u64)>
            // The Uint128 is the amount newly available from this slot.
            if let Some((available_from_slot, _)) = new_claims.get(&slot_idx) {
                let take_from_slot = std::cmp::min(remaining_to_distribute, *available_from_slot);
                if take_from_slot > Uint128::zero() {
                    claims_to_record.insert(slot_idx, (take_from_slot, env.block.time.seconds()));
                    remaining_to_distribute =
                        remaining_to_distribute.saturating_sub(take_from_slot);
                }
            }
        }

        // Phase 2: Distribute remaining to LinearVesting slots from new_claims
        if remaining_to_distribute > Uint128::zero() {
            for slot_idx in linear_vesting_slots_with_new_claims {
                if remaining_to_distribute == Uint128::zero() {
                    break;
                }
                if let Some((available_from_slot, _)) = new_claims.get(&slot_idx) {
                    let take_from_slot =
                        std::cmp::min(remaining_to_distribute, *available_from_slot);
                    if take_from_slot > Uint128::zero() {
                        claims_to_record
                            .insert(slot_idx, (take_from_slot, env.block.time.seconds()));
                        remaining_to_distribute =
                            remaining_to_distribute.saturating_sub(take_from_slot);
                    }
                }
            }
        }
    }
    // At this point, if initial checks were correct (actual_claim_amount_coin.amount <= sum of new_claims),
    // remaining_to_distribute should be zero.

    let updated_claims = helpers::aggregate_claims(&previous_claims, &claims_to_record)?;

    campaign.claimed.amount = campaign
        .claimed
        .amount
        .checked_add(actual_claim_amount_coin.amount)?;

    CAMPAIGN.save(deps.storage, &campaign)?;
    CLAIMS.save(deps.storage, receiver.to_string(), &updated_claims)?;

    ensure!(
        total_user_allocation >= get_total_claims_amount_for_address(deps.as_ref(), &receiver)?,
        ContractError::ExceededMaxClaimAmount
    );

    Ok(Response::default()
        .add_message(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: vec![actual_claim_amount_coin.clone()],
        })
        .add_attributes(vec![
            ("action", "claim".to_string()),
            ("receiver", receiver.to_string()),
            ("claimed_amount", actual_claim_amount_coin.to_string()),
        ]))
}

/// Adds a batch of addresses and their allocations. This can only be done before the campaign has started.
///
/// # Arguments
/// * `deps` - The dependencies
/// * `env`  - The env context
/// * `info` - The message info
/// * `allocations` - Vector of (address, amount) pairs
///
/// # Returns
/// * `Result<Response, ContractError>` - The response with attributes
pub fn add_allocations(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    allocations: Vec<(String, Uint128)>,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    // Check if campaign has started
    let campaign = CAMPAIGN.may_load(deps.storage)?;

    if let Some(campaign) = campaign {
        ensure!(
            !campaign.has_started(&env.block.time),
            ContractError::CampaignError {
                reason: "cannot upload allocations after campaign has started".to_string(),
            }
        );
    }

    for (address, amount) in &allocations {
        let allocation = ALLOCATIONS.may_load(deps.storage, address.as_str())?;
        ensure!(
            allocation.is_none(),
            ContractError::AllocationAlreadyExists {
                address: address.clone(),
            }
        );
        ALLOCATIONS.save(deps.storage, address.as_str(), amount)?;
    }

    Ok(Response::default()
        .add_attribute("action", "add_allocations")
        .add_attribute("count", allocations.len().to_string()))
}

/// Replaces an address in the allocation list. This can only be done before the campaign has started.
///
/// # Arguments
/// * `deps` - The dependencies
/// * `info` - The message info
/// * `old_address` - The old address to replace
/// * `new_address` - The new address to use
///
/// # Returns
/// * `Result<Response, ContractError>` - The response with attributes
pub fn replace_address(
    deps: DepsMut,
    info: MessageInfo,
    old_address: String,
    new_address: String,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    // if the old address has claims, we need to move them to the new address
    let claims = get_claims_for_address(deps.as_ref(), old_address.clone())?;
    if !claims.is_empty() {
        CLAIMS.save(deps.storage, new_address.clone(), &claims)?;
        CLAIMS.remove(deps.storage, old_address.clone());
    }

    // Get old allocation
    let old_allocation = get_allocation(deps.as_ref(), &old_address)?;
    ensure!(
        old_allocation.is_some(),
        ContractError::NoAllocationFound {
            address: old_address.clone(),
        }
    );

    // Replace old allocation with new allocation
    ALLOCATIONS.save(deps.storage, &new_address, &old_allocation.unwrap())?;
    ALLOCATIONS.remove(deps.storage, &old_address);

    Ok(Response::default()
        .add_attribute("action", "replace_address")
        .add_attribute("old_address", old_address)
        .add_attribute("new_address", new_address))
}

/// Blacklists or unblacklists an address. This can be done at any time.
///
/// # Arguments
/// * `deps` - The dependencies
/// * `info` - The message info
/// * `address` - The address to blacklist/unblacklist
/// * `blacklist` - Whether to blacklist or unblacklist
///
/// # Returns
/// * `Result<Response, ContractError>` - The response with attributes
pub fn blacklist_address(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
    blacklist: bool,
) -> Result<Response, ContractError> {
    cw_ownable::assert_owner(deps.storage, &info.sender)?;

    if blacklist {
        BLACKLIST.save(deps.storage, &address, &true)?;
    } else {
        BLACKLIST.remove(deps.storage, &address);
    }

    Ok(Response::default()
        .add_attribute("action", "blacklist_address")
        .add_attribute("address", address)
        .add_attribute("blacklisted", blacklist.to_string()))
}
