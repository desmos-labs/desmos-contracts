#![cfg(test)]
use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult};
use cw721_base::{
    ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721ExecuteMsg,
    InstantiateMsg as Cw721InstantiateMsg, QueryMsg as Cw721QueryMsg,
};
use cw_multi_test::{Contract, ContractWrapper};
use poap::{
    contract::{
        instantiate as poap_instantiate,
        execute as poap_execute,
        query as poap_query,
    },
    error::ContractError as POAPContractError,
    msg::{
    InstantiateMsg as POAPInstantiateMsg,
    ExecuteMsg as POAPExecuteMsg,
    QueryMsg as POAPQueryMsg,
}};

pub struct POAPTestContract;
impl POAPTestContract {
    fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: POAPInstantiateMsg,
    ) -> Result<Response, POAPContractError> {
        poap_instantiate(deps, env, info, msg)
    }

    fn failing_instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: Cw721InstantiateMsg,
    ) -> Result<Response, POAPContractError> {
        Err(POAPContractError::Std(StdError::generic_err("cw721 initialization failed")))
    }

    fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: POAPExecuteMsg,
    ) -> Result<Response, POAPContractError> {
        poap_execute(deps, env, info, msg)
    }

    fn query(deps: Deps, env: Env, msg: POAPQueryMsg) -> StdResult<Binary> {
        poap_query(deps, env, msg)
    }

    /// Provides an instance of a poap contract.
    /// This instance can be used only during the integration tests.
    pub fn success_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(Self::execute, Self::instantiate, Self::query);
        Box::new(contract)
    }

    /// Provides an instance of a poap contract that fails during the initialization.
    /// This instance can be used only during the integration tests.
    pub fn failing_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(Self::execute, Self::failing_instantiate, Self::query);
        Box::new(contract)
    }
}

pub struct CW721TestContract;
impl CW721TestContract {
    fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721InstantiateMsg,
    ) -> Result<Response, StdError> {
        Cw721Contract::<'static, String, Empty>::default().instantiate(deps, env, info, msg)
    }

    fn failing_instantiate(
        _deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        _msg: Cw721InstantiateMsg,
    ) -> Result<Response, StdError> {
        Err(StdError::generic_err("cw721 initialization failed"))
    }

    fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721ExecuteMsg<String>,
    ) -> Result<Response, Cw721ContractError> {
        Cw721Contract::<'static, String, Empty>::default().execute(deps, env, info, msg)
    }

    fn query(deps: Deps, env: Env, msg: Cw721QueryMsg) -> StdResult<Binary> {
        Cw721Contract::<'static, String, Empty>::default().query(deps, env, msg)
    }

    /// Provides an instance of a cw721 contract.
    /// This instance can be used only during the integration tests.
    pub fn success_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(Self::execute, Self::instantiate, Self::query);
        Box::new(contract)
    }

    /// Provides an instance of a cw721 contract that fails during the initialization.
    /// This instance can be used only during the integration tests.
    pub fn failing_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(Self::execute, Self::failing_instantiate, Self::query);
        Box::new(contract)
    }
}
