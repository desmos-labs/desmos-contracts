use anyhow::Result as AnyResult;
use cosmwasm_std::{
    to_binary, Addr, Api, Binary, BlockInfo, Deps, DepsMut, Empty, Env, MessageInfo, Querier,
    Response, StdError, StdResult, Storage, Uint64,
};
use cw721_base::{
    ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721ExecuteMsg,
    InstantiateMsg as Cw721InstantiateMsg, QueryMsg as Cw721QueryMsg,
};
use cw721_remarkables::Metadata;
use cw_multi_test::{AppResponse, Contract, ContractWrapper, CosmosRouter, Module};
use desmos_bindings::{
    mocks::mock_apps::DesmosModule,
    msg::DesmosMsg,
    posts::mocks::mock_posts_query_response,
    query::DesmosQuery,
    reactions::{models_query::QueryReactionsResponse, query::ReactionsQuery},
    subspaces::mocks::mock_subspaces_query_response,
    types::PageResponse,
};

/// Defines the cw721 test contract.
pub struct CW721TestContract;
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

pub const ADMIN: &str = "cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t";
pub const SUBSPACE_ID: Uint64 = Uint64::new(1);
pub const POST_ID: Uint64 = Uint64::new(1);
pub const REMARKABLES_URI: &str = "ipfs://remarkables.com";
pub const AUTHOR: &str = "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc";
pub const ACCEPTED_RARITY_LEVEL: u32 = 0;
pub const ACCEPTED_ENGAGEMENT_THRESHOLD: u32 = 10;
pub const UNACCEPTED_ENGAGEMENT_THRESHOLD: u32 = 100;

fn get_reaction_response(number: u32) -> QueryReactionsResponse {
    QueryReactionsResponse {
        reactions: vec![],
        pagination: Some(PageResponse {
            next_key: None,
            total: Some(number.into()),
        }),
    }
}
fn get_reactions(user: Option<Addr>) -> QueryReactionsResponse {
    let self_reactions_count = 0;
    if user == Some(Addr::unchecked(AUTHOR)) {
        return get_reaction_response(self_reactions_count);
    }
    get_reaction_response(ACCEPTED_ENGAGEMENT_THRESHOLD)
}

/// Defines the mock keeper of Desmos modules.
pub struct DesmosKeeper {}
impl DesmosModule for DesmosKeeper {}
impl Module for DesmosKeeper {
    type ExecT = DesmosMsg;
    type QueryT = DesmosQuery;
    type SudoT = Empty;
    fn execute<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _sender: Addr,
        _msg: DesmosMsg,
    ) -> AnyResult<AppResponse> {
        unimplemented!()
    }
    fn query(
        &self,
        _api: &dyn Api,
        _storage: &dyn Storage,
        _querier: &dyn Querier,
        _block: &BlockInfo,
        request: DesmosQuery,
    ) -> AnyResult<Binary> {
        match request {
            DesmosQuery::Subspaces(query) => {
                AnyResult::Ok(mock_subspaces_query_response(&query).unwrap())
            }
            DesmosQuery::Posts(query) => AnyResult::Ok(mock_posts_query_response(&query).unwrap()),
            DesmosQuery::Reactions(query) => match query {
                ReactionsQuery::Reactions { user, .. } => {
                    AnyResult::Ok(to_binary(&get_reactions(user)).unwrap())
                }
                _ => unimplemented!(),
            },
        }
    }
    fn sudo<ExecC, QueryC>(
        &self,
        _api: &dyn Api,
        _storage: &mut dyn Storage,
        _router: &dyn CosmosRouter<ExecC = ExecC, QueryC = QueryC>,
        _block: &BlockInfo,
        _msg: Empty,
    ) -> AnyResult<AppResponse> {
        unimplemented!()
    }
}
