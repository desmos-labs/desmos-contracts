#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_execute, wasm_instantiate, Addr, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Reply, Response, StdResult, SubMsg,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;

use desmos_bindings::profiles::querier::ProfilesQuerier;
use poap::msg::ExecuteMsg as POAPExecuteMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg};
use crate::reply::*;
use crate::state::{Config, CONFIG};

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
    msg.validate()?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = deps.api.addr_validate(&msg.admin)?;
    instantiate_config(
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
            wasm_instantiate(
                msg.poap_code_id,
                &msg.poap_instantiate_msg,
                info.funds,
                "poap-manager".into(),
            )?,
            INSTANTIATE_POAP_REPLY_ID,
        )))
}

fn instantiate_config(
    deps: DepsMut,
    admin: Addr,
    poap_code_id: u64,
    subspace_id: u64,
    event_post_id: u64,
) -> Result<(), ContractError> {
    let config = Config {
        admin,
        poap_code_id: poap_code_id,
        poap_address: Addr::unchecked(""),
        subspace_id: subspace_id,
        event_post_id: event_post_id,
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply_msg: Reply) -> Result<Response, ContractError> {
    match reply_msg.id {
        INSTANTIATE_POAP_REPLY_ID => resolve_instantiate_poap_reply(deps, reply_msg),
        _ => Err(ContractError::InvalidReplyID {}),
    }
}

fn resolve_instantiate_poap_reply(deps: DepsMut, msg: Reply) -> Result<Response, ContractError> {
    let res = parse_reply_instantiate_data(msg)?;
    let address = deps.api.addr_validate(&res.contract_address)?;
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.poap_address = address;
        Ok(config)
    })?;
    Ok(Response::new().add_attribute("action", "instantiate_poap_reply"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    msg.validate()?;
    match msg {
        ExecuteMsg::Claim { post_id } => claim(deps, info, post_id),
        ExecuteMsg::MintTo { recipient } => mint_to(deps, info, recipient),
        ExecuteMsg::UpdateAdmin { new_admin } => update_admin(deps, info, new_admin),
    }
}

fn claim(deps: DepsMut, info: MessageInfo, post_id: u64) -> Result<Response, ContractError> {
    let poap_address = CONFIG.load(deps.storage)?.poap_address;
    if !check_eligibility(deps, info.sender.clone(), post_id)? {
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

fn check_eligibility(deps: DepsMut, user: Addr, _post_id: u64) -> Result<bool, ContractError> {
    ProfilesQuerier::new(deps.querier.deref()).query_profile(user)?;
    Ok(true)
}

fn mint_to(deps: DepsMut, info: MessageInfo, recipient: String) -> Result<Response, ContractError> {
    let poap_address = CONFIG.load(deps.storage)?.poap_address;
    Ok(Response::new()
        .add_attribute("action", "mint_to")
        .add_attribute("sender", &info.sender)
        .add_message(wasm_execute(
            poap_address,
            &POAPExecuteMsg::MintTo { recipient },
            info.funds,
        )?))
}

fn update_admin(deps: DepsMut, info: MessageInfo, user: String) -> Result<Response, ContractError> {
    let new_admin = deps.api.addr_validate(&user)?;
    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        if config.admin != info.sender {
            return Err(ContractError::NotAdmin {});
        }
        config.admin = new_admin.clone();
        Ok(config)
    })?;
    Ok(Response::new()
        .add_attribute("action", "update_admin")
        .add_attribute("new_admin", new_admin)
        .add_attribute("sender", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<QueryConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(QueryConfigResponse { config })
}
