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
use crate::state::{Config, CONFIG};

use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:poap-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// actions for executing messages
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_INSTANTIATE_POAP_REPLY: &str = "instantiate_poap_reply";
const ACTION_CLAIM: &str = "claim";
const ACTION_MINT_TO: &str = "mint_to";
const ACTION_UPDATE_ADMIN: &str = "update_admin";

// attributes for executing messages
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_ADMIN: &str = "admin";
const ATTRIBUTE_POAP_CODE_ID: &str = "poap_code_id";
const ATTRIBUTE_SENDER: &str = "sender";
const ATTRIBUTE_NEW_ADMIN: &str = "new_admin";

const INSTANTIATE_POAP_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    msg.validate()?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = deps.api.addr_validate(&msg.admin)?;
    instantiate_config(deps, admin.clone(), msg.poap_code_id.u64())?;

    // assign the admin and minter of poap to the contract address
    let mut poap_instantiate_msg = msg.poap_instantiate_msg;
    poap_instantiate_msg.minter = env.contract.address.into();

    Ok(Response::new()
        .add_attribute("action", ACTION_INSTANTIATE)
        .add_attribute(ATTRIBUTE_ADMIN, admin)
        .add_attribute(ATTRIBUTE_POAP_CODE_ID, msg.poap_code_id)
        .add_submessage(SubMsg::reply_on_success(
            wasm_instantiate(
                msg.poap_code_id.u64(),
                &poap_instantiate_msg,
                info.funds,
                "poap".into(),
            )?,
            INSTANTIATE_POAP_REPLY_ID,
        )))
}

fn instantiate_config(deps: DepsMut, admin: Addr, poap_code_id: u64) -> Result<(), ContractError> {
    let config = Config {
        admin,
        poap_code_id: poap_code_id,
        poap_address: Addr::unchecked(""),
    };
    CONFIG.save(deps.storage, &config)?;
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
    CONFIG.update(deps.storage, |mut config| -> StdResult<_> {
        config.poap_address = address;
        Ok(config)
    })?;
    Ok(Response::new().add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE_POAP_REPLY))
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
        ExecuteMsg::Claim {} => claim(deps, info),
        ExecuteMsg::MintTo { recipient } => mint_to(deps, info, recipient),
        ExecuteMsg::UpdateAdmin { new_admin } => update_admin(deps, info, new_admin),
    }
}

fn claim(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    let poap_address = CONFIG.load(deps.storage)?.poap_address;
    if !check_eligibility(deps, info.sender.clone())? {
        return Err(ContractError::NoEligibilityError {});
    }
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_CLAIM)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_message(wasm_execute(
            poap_address,
            &POAPExecuteMsg::MintTo {
                recipient: info.sender.into(),
            },
            info.funds,
        )?))
}

fn check_eligibility(deps: DepsMut, user: Addr) -> Result<bool, ContractError> {
    ProfilesQuerier::new(deps.querier.deref()).query_profile(user)?;
    Ok(true)
}

fn mint_to(deps: DepsMut, info: MessageInfo, recipient: String) -> Result<Response, ContractError> {
    let poap_address = CONFIG.load(deps.storage)?.poap_address;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_MINT_TO)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
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
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_ADMIN)
        .add_attribute(ATTRIBUTE_NEW_ADMIN, new_admin)
        .add_attribute(ATTRIBUTE_SENDER, info.sender))
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockQuerier, MOCK_CONTRACT_ADDR,
    };
}