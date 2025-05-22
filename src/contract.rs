use cosmwasm_std::{entry_point, to_json_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::{commands, queries, validate_contract};

// version info for migration info
const CONTRACT_NAME: &str = "mantra_claimdrop-contract";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg.owner.unwrap_or(info.sender.into_string());
    cw_ownable::initialize_owner(deps.storage, deps.api, Some(&owner))?;

    Ok(Response::default().add_attributes(vec![
        ("action", "instantiate".to_string()),
        ("owner", owner),
    ]))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ManageCampaign { action } => commands::manage_campaign(deps, env, info, action),
        ExecuteMsg::Claim { receiver, amount } => {
            cw_utils::nonpayable(&info)?;
            commands::claim(deps, env, info, receiver, amount)
        }
        ExecuteMsg::AddAllocations { allocations } => {
            cw_utils::nonpayable(&info)?;
            commands::add_allocations(deps, env, info, allocations)
        }
        ExecuteMsg::ReplaceAddress {
            old_address,
            new_address,
        } => {
            cw_utils::nonpayable(&info)?;
            commands::replace_address(deps, info, old_address, new_address)
        }
        ExecuteMsg::BlacklistAddress { address, blacklist } => {
            cw_utils::nonpayable(&info)?;
            commands::blacklist_address(deps, info, address, blacklist)
        }
        ExecuteMsg::UpdateOwnership(action) => {
            cw_utils::nonpayable(&info)?;
            Ok(
                cw_ownable::update_ownership(deps, &env.block, &info.sender, action).map(
                    |ownership| {
                        Response::default()
                            .add_attribute("action", "update_ownership")
                            .add_attributes(ownership.into_attributes())
                    },
                )?,
            )
        }
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Campaign {} => Ok(to_json_binary(&queries::query_campaign(deps)?)?),
        QueryMsg::Rewards { receiver } => Ok(to_json_binary(&queries::query_rewards(
            deps, env, receiver,
        )?)?),
        QueryMsg::Ownership {} => Ok(to_json_binary(&cw_ownable::get_ownership(deps.storage)?)?),
        QueryMsg::Claimed {
            address,
            start_from,
            limit,
        } => Ok(to_json_binary(&queries::query_claimed(
            deps, address, start_from, limit,
        )?)?),
        QueryMsg::Allocations {
            address,
            start_after,
            limit,
        } => Ok(to_json_binary(&queries::query_allocation(
            deps,
            address,
            start_after,
            limit,
        )?)?),
        QueryMsg::IsBlacklisted { address } => Ok(to_json_binary(&queries::query_is_blacklisted(
            deps, address,
        )?)?),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    validate_contract!(deps, CONTRACT_NAME, CONTRACT_VERSION);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
