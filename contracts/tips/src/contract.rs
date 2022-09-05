use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_execute, wasm_instantiate, Addr, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Reply, Response, StdError, StdResult, SubMsg,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;
use desmos_bindings::{msg::DesmosMsg, query::DesmosQuery};
use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tips";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    msg.validate()?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = deps.api.addr_validate(&msg.admin)?;

    Err(ContractError::Std(StdError::generic_err("not implemented")))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    Err(ContractError::Std(StdError::generic_err("not implemented")))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    Err(StdError::generic_err("not implemented"))
}

#[cfg(test)]
mod tests {}
