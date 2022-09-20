use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult};
use cw721_base::{
    ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721ExecuteMsg,
    InstantiateMsg as Cw721InstantiateMsg, QueryMsg as Cw721QueryMsg,
};
use cw_multi_test::{Contract, ContractWrapper};
pub struct CW721TestContract;
use cw721_remarkables::Metadata;
use desmos_bindings::{msg::DesmosMsg, query::DesmosQuery};
impl CW721TestContract {
    fn instantiate(
        deps: DepsMut<DesmosQuery>,
        env: Env,
        info: MessageInfo,
        msg: Cw721InstantiateMsg,
    ) -> Result<Response<DesmosMsg>, StdError> {
        Cw721Contract::<'static, Metadata, Empty, Empty, DesmosMsg, DesmosQuery>::default()
            .instantiate(deps, env, info, msg)
    }

    fn failing_instantiate(
        _deps: DepsMut<DesmosQuery>,
        _env: Env,
        _info: MessageInfo,
        _msg: Cw721InstantiateMsg,
    ) -> Result<Response<DesmosMsg>, StdError> {
        Err(StdError::generic_err("cw721 initialization failed"))
    }

    fn execute(
        deps: DepsMut<DesmosQuery>,
        env: Env,
        info: MessageInfo,
        msg: Cw721ExecuteMsg<Metadata, Empty>,
    ) -> Result<Response<DesmosMsg>, Cw721ContractError> {
        Cw721Contract::<'static, Metadata, Empty, Empty, DesmosMsg, DesmosQuery>::default()
            .execute(deps, env, info, msg)
    }

    fn query(deps: Deps<DesmosQuery>, env: Env, msg: Cw721QueryMsg<Empty>) -> StdResult<Binary> {
        Cw721Contract::<'static, Metadata, Empty, Empty, DesmosMsg, DesmosQuery>::default()
            .query(deps, env, msg)
    }

    /// Provides an instance of a cw721 contract.
    /// This instance can be used only during the integration tests.
    pub fn success_contract() -> Box<dyn Contract<DesmosMsg, DesmosQuery>> {
        let contract = ContractWrapper::new(Self::execute, Self::instantiate, Self::query);
        Box::new(contract)
    }

    /// Provides an instance of a cw721 contract that fails during the initialization.
    /// This instance can be used only during the integration tests.
    pub fn failing_contract() -> Box<dyn Contract<DesmosMsg, DesmosQuery>> {
        let contract = ContractWrapper::new(Self::execute, Self::failing_instantiate, Self::query);
        Box::new(contract)
    }
}
