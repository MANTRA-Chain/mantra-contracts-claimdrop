use cosmwasm_std::{entry_point, to_json_binary};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::PROXY;
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

    if let Some(proxy) = msg.proxy {
        PROXY.save(deps.storage, &deps.api.addr_validate(&proxy)?)?;
    }

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
        ExecuteMsg::Claim {
            total_claimable_amount,
            receiver,
            proof,
        } => commands::claim(deps, env, info, total_claimable_amount, receiver, proof),
        ExecuteMsg::UpdateOwnership(action) => {
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
        QueryMsg::Rewards {
            total_claimable_amount,
            receiver,
            proof,
        } => Ok(to_json_binary(&queries::query_rewards(
            deps,
            env,
            total_claimable_amount,
            receiver,
            proof,
        )?)?),
        QueryMsg::Ownership {} => Ok(to_json_binary(&cw_ownable::get_ownership(deps.storage)?)?),
        QueryMsg::Claimed {
            address,
            start_from,
            limit,
        } => Ok(to_json_binary(&queries::query_claimed(
            deps, address, start_from, limit,
        )?)?),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    validate_contract!(deps, CONTRACT_NAME, CONTRACT_VERSION);
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
