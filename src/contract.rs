use cosmwasm_std::{ensure, entry_point, to_json_binary, StdError};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response};
use cw2::{get_contract_version, set_contract_version, CONTRACT};
use semver::Version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::{commands, queries};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:airdrop-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    cw_ownable::initialize_owner(
        deps.storage,
        deps.api,
        Some(&msg.owner.unwrap_or(info.sender.into_string())),
    )?;

    Ok(Response::default())
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
            campaign_id,
            total_amount,
            proof,
        } => commands::claim(deps, env, info, campaign_id, total_amount, proof),
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
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Campaigns {
            filter_by,
            start_after,
            limit,
        } => Ok(to_json_binary(&queries::query_campaigns(
            deps,
            filter_by,
            start_after,
            limit,
        )?)?),
        QueryMsg::Ownership {} => Ok(to_json_binary(&cw_ownable::get_ownership(deps.storage)?)?),
    }
}

#[entry_point]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    //todo write macro for all this contract name/version validation
    let stored_contract_name = CONTRACT.load(deps.storage)?.contract;
    ensure!(
        &stored_contract_name == CONTRACT_NAME,
        StdError::generic_err("Contract name mismatch")
    );

    let version: Version = CONTRACT_VERSION.parse()?;
    let storage_version: Version = get_contract_version(deps.storage)?.version.parse()?;

    ensure!(
        storage_version < version,
        ContractError::MigrateInvalidVersion {
            current_version: storage_version,
            new_version: version,
        }
    );

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    Ok(Response::default())
}
