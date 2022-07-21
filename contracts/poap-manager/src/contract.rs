#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_execute, wasm_instantiate, Addr, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Reply, Response, StdResult, SubMsg,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;

use desmos_bindings::posts::querier::PostsQuerier;
use poap::msg::ExecuteMsg as POAPExecuteMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg};
use crate::reply::*;
use crate::state::{Config, ADMIN, CONFIG, POAP_ADDRESS};

use std::ops::Deref;

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
    let admin = match &msg.admin {
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
        .add_submessage(SubMsg::reply_on_success(
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
    match msg.id {
        INSTANTIATE_POAP_REPLY_ID => resolve_instantiate_poap_reply(deps, msg),
        _ => Err(ContractError::InvalidReplyID {}),
    }
}

fn resolve_instantiate_poap_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(msg)?;
    let address = deps.api.addr_validate(&res.contract_address)?;
    POAP_ADDRESS.save(deps.storage, &address)?;
    Ok(Response::new().add_attribute("action", "instantiate_poap_reply"))
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
        ExecuteMsg::Claim { post_id } => claim(deps, info, post_id),
        ExecuteMsg::MintTo { recipient } => mint_to(deps, info, recipient),
        ExecuteMsg::UpdateAdmin { new_admin } => {
            Ok(ADMIN.execute_update_admin(deps, info, Some(api.addr_validate(&new_admin)?))?)
        }
    }
}

fn claim(deps: DepsMut, info: MessageInfo, post_id: u64) -> Result<Response, ContractError> {
    let poap_address = POAP_ADDRESS.load(deps.storage)?;
    let config = CONFIG.load(deps.storage)?;
    if !check_eligibility(deps, info.sender.clone(), config, post_id)? {
        return Err(ContractError::NoEligibilityError {});
    }
    Ok(Response::new()
        .add_attribute("action", "claim")
        .add_attribute("sender", &info.sender)
        .add_message(wasm_execute(
            poap_address,
            &POAPExecuteMsg::MintTo {
                recipient: info.sender.into(),
            },
            info.funds,
        )?))
}

fn check_eligibility(deps: DepsMut, user: Addr, config: Config, post_id: u64) -> Result<bool, ContractError> {
    let post_res = PostsQuerier::new(deps.querier.deref()).query_post(config.subspace_id, post_id)?;
    let post = post_res.post;
    if post.author != user {
        return Ok(false)
    }
    match post.conversation_id {
        Some(id) => return Ok(id.u64() == config.event_post_id),
        None => return Ok(false)
    }
}

fn mint_to(deps: DepsMut, info: MessageInfo, recipient: String) -> Result<Response, ContractError> {
    let poap_address = POAP_ADDRESS.load(deps.storage)?;
    Ok(Response::new()
        .add_attribute("action", "mint_to")
        .add_attribute("sender", &info.sender)
        .add_message(wasm_execute(
            poap_address,
            &POAPExecuteMsg::MintTo { recipient },
            info.funds,
        )?))
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
