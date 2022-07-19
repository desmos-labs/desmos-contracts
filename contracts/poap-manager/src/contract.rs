#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_instantiate, Addr, Deps, DepsMut, Env, MessageInfo, QueryResponse, Reply,
    Response, StdResult, SubMsg,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg};
use crate::state::{Config, ADMIN, CONFIG, POAP_ADDRESS};
use crate::reply::*;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:poap-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Validate the admin address
    let admin = match msg.admin.clone() {
        // Fallback to sender if the admin is not defined
        None => info.sender.clone(),
        // Admin defined, make sure that is a valid address
        Some(admin_address) => deps.api.addr_validate(&admin_address)?,
    };
    save_config(
        deps,
        admin.clone(),
        msg.poap_code_id,
        msg.subspace_id,
        msg.event_post_id,
    )?;
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("admin", admin)
        .add_submessage(SubMsg::reply_always(
            wasm_instantiate(msg.poap_code_id, &msg, info.funds, "poap-manager".into())?,
            INSTANTIATE_POAP_REPLY_ID,
        )))
}

fn save_config(
    deps: DepsMut,
    admin: Addr,
    poap_code_id: u64,
    subspace_id: u64,
    event_post_id: u64,
) -> Result<(), ContractError> {
    let config = Config {
        poap_code_id: poap_code_id,
        subspace_id: subspace_id,
        event_post_id: event_post_id,
    };
    CONFIG.save(deps.storage, &config)?;
    ADMIN.set(deps, Some(admin))?;
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
    if msg.id != INSTANTIATE_POAP_REPLY_ID {
        return Err(ContractError::InvalidReplyID {});
    }
    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => {
            let address = deps.api.addr_validate(&res.contract_address)?;
            POAP_ADDRESS.save(deps.storage, &address)?;
            Ok(Response::new().add_attribute("action", "instantiate_poap_reply"))
        }
        Err(_) => Err(ContractError::InstantiatePOAPError {}),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    let api = deps.api;
    match msg {
        ExecuteMsg::Claim { .. } => claim(),
        ExecuteMsg::MintTo { user } => mint_to(),
        ExecuteMsg::UpdateAdmin { new_admin } => Ok(ADMIN.execute_update_admin(
            deps,
            info,
            Some(api.addr_validate(&new_admin)?),
        )?),
    }
}

fn claim() -> Result<Response, ContractError> {
    Ok(Response::new())
}

fn mint_to() -> Result<Response, ContractError> {
    Ok(Response::new())
}


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(_deps: Deps) -> StdResult<QueryConfigResponse> {
    Ok(QueryConfigResponse {})
}
