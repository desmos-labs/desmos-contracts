use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult};
use cw721_base::{
    ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721ExecuteMsg,
    InstantiateMsg as Cw721InstantiateMsg, QueryMsg as Cw721QueryMsg,
};
use cw_multi_test::{Contract, ContractWrapper};

fn cw721_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721ExecuteMsg<String>,
) -> Result<Response, Cw721ContractError> {
    Cw721Contract::<'static, String, Empty>::default().execute(deps, env, info, msg)
}

fn cw721_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721InstantiateMsg,
) -> Result<Response, StdError> {
    Cw721Contract::<'static, String, Empty>::default().instantiate(deps, env, info, msg)
}

fn failing_cw721_instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: Cw721InstantiateMsg,
) -> Result<Response, StdError> {
    Err(StdError::generic_err("cw721 initialization failed"))
}

fn cw721_query(deps: Deps, env: Env, msg: Cw721QueryMsg) -> StdResult<Binary> {
    Cw721Contract::<'static, String, Empty>::default().query(deps, env, msg)
}

/// Provides an instance of a cw721 contract.
/// This instance can be used only during the integration tests.
pub fn contract_cw721() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(cw721_execute, cw721_instantiate, cw721_query);
    Box::new(contract)
}

/// Provides an instance of a cw721 contract that fails during the initialization.
/// This instance can be used only during the integration tests.
pub fn failing_cw721() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(cw721_execute, failing_cw721_instantiate, cw721_query);
    Box::new(contract)
}
