use cosmwasm_std::{Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult};
use cw721_base::{
    ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721ExecuteMsg, Extension,
    InstantiateMsg as Cw721InstantiateMsg, QueryMsg as Cw721QueryMsg,
};
use cw_multi_test::{Contract, ContractWrapper};

fn cw721_execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721ExecuteMsg<Extension>,
) -> Result<Response, Cw721ContractError> {
    Cw721Contract::<'static, Extension, Empty>::default().execute(deps, env, info, msg)
}

fn cw721_instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: Cw721InstantiateMsg,
) -> Result<Response, StdError> {
    Cw721Contract::<'static, Extension, Empty>::default().instantiate(deps, env, info, msg)
}

fn cw721_query(deps: Deps, env: Env, msg: Cw721QueryMsg) -> StdResult<Binary> {
    Cw721Contract::<'static, Extension, Empty>::default().query(deps, env, msg)
}

pub fn contract_cw721() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new(cw721_execute, cw721_instantiate, cw721_query);
    Box::new(contract)
}
