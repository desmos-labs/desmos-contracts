#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    has_coins, to_binary, wasm_execute, wasm_instantiate, Addr, Binary, Coin, Deps, DepsMut, Empty,
    Env, MessageInfo, Querier, Reply, Response, StdResult, Storage, SubMsg, Uint64,
};
use cw2::set_contract_version;
use cw721::{AllNftInfoResponse, TokensResponse};
use cw721_base::{
    ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg, MintMsg,
    QueryMsg as Cw721QueryMsg,
};
use cw721_remarkables::Metadata;
use cw_utils::parse_reply_instantiate_data;
use desmos_bindings::{
    msg::DesmosMsg,
    posts::querier::PostsQuerier,
    query::DesmosQuery,
    reactions::querier::ReactionsQuerier,
    subspaces::querier::SubspacesQuerier,
    types::{PageRequest, PageResponse},
};
use std::ops::Deref;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, QueryRaritiesResponse, Rarity,
};
use crate::state::{ConfigState, CONFIG, CW721_ADDRESS, MINTED_TOKEN, RARITIES};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:remarkables";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

// actions for executing messages
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_INSTANTIATE_CW721_REPLY: &str = "instantiate_cw721_reply";
const ACTION_MINT: &str = "mint";
const ACTION_UPDATE_ADMIN: &str = "update_admin";
const ACTION_UPDATE_RARITY_MINT_FEES: &str = "update_rarity_mint_fees";

// attributes for executing messages
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_SENDER: &str = "sender";
const ATTRIBUTE_ADMIN: &str = "admin";
const ATTRIBUTE_CW721_CODE_ID: &str = "cw721_code_id";
const ATTRIBUTE_NEW_ADMIN: &str = "new_admin";
const ATTRIBUTE_RARITY_LEVEL: &str = "rarity_level";
const ATTRIBUTE_RECIPIENT: &str = "recipient";
const ATTRIBUTE_TOKEN_ID: &str = "token_id";
const ATTRIBUTE_TOKEN_URI: &str = "token_uri";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    msg.validate()?;
    // Save the config
    let admin_addr = deps.api.addr_validate(&msg.admin)?;
    CONFIG.save(
        deps.storage,
        &ConfigState {
            admin: admin_addr,
            subspace_id: msg.subspace_id.into(),
            cw721_code_id: msg.cw721_code_id.into(),
        },
    )?;
    // Save the info of rarities
    RARITIES.save(deps.storage, &msg.rarities)?;
    let subspace_id = msg.subspace_id.u64();
    // Check subspace exists and it is owned by the sender.
    let subspace = SubspacesQuerier::new(deps.querier.deref())
        .query_subspace(subspace_id)
        .map_err(|_| ContractError::SubspaceNotFound { id: subspace_id })?
        .subspace;
    if info.sender != subspace.owner {
        return Err(ContractError::NotSubspaceOwner {
            caller: info.sender,
        });
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Submessage to instantiate cw721 contract
    let cw721_submessage = SubMsg::reply_on_success(
        wasm_instantiate(
            msg.cw721_code_id.into(),
            &Cw721InstantiateMsg {
                name: msg.cw721_instantiate_msg.name,
                symbol: msg.cw721_instantiate_msg.symbol,
                minter: env.contract.address.to_string(),
            },
            info.funds,
            "remarkables_cw721".to_string(),
        )?,
        INSTANTIATE_CW721_REPLY_ID,
    );
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_ADMIN, msg.admin)
        .add_attribute(ATTRIBUTE_CW721_CODE_ID, msg.cw721_code_id)
        .add_submessage(cw721_submessage))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    msg.validate()?;
    match msg {
        ExecuteMsg::Mint {
            post_id,
            remarkables_uri,
            rarity_level,
        } => execute_mint(deps, info, rarity_level, post_id.into(), remarkables_uri),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
        ExecuteMsg::UpdateRarityMintFees {
            rarity_level,
            new_fees,
        } => execute_update_rarity_mint_fees(deps, info, rarity_level, new_fees),
    }
}

fn execute_mint(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    rarity_level: u32,
    post_id: u64,
    remarkables_uri: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let rarities = RARITIES.load(deps.storage)?;
    let rarity = rarities
        .get(rarity_level as usize)
        .ok_or(ContractError::RarityNotExists {
            level: rarity_level,
        })?;
    // Check if rarity mint fees is enough
    if !is_enough_fees(info.funds, &rarity.mint_fees) {
        return Err(ContractError::MintFeesNotEnough {});
    }
    // Check if post reaches the eligible threshold
    check_eligibility(
        deps.storage,
        deps.querier.deref(),
        info.sender.clone(),
        post_id,
        rarity.engagement_threshold,
    )?;
    // Check if token has been minted or not
    let token_id = convert_post_id_to_token_id(post_id, rarity_level);
    if MINTED_TOKEN
        .may_load(deps.storage, token_id.clone())?
        .is_some()
    {
        return Err(ContractError::TokenHasBeenMinted { token_id: token_id });
    }
    MINTED_TOKEN.save(deps.storage, token_id.clone(), &true)?;
    // Create the cw721 message to send to mint the remarkables
    let mint_msg = Cw721ExecuteMsg::<Metadata, Empty>::Mint(MintMsg::<Metadata> {
        token_id: token_id.clone(),
        owner: info.sender.clone().into(),
        token_uri: Some(remarkables_uri.clone()),
        extension: Metadata {
            rarity_level,
            subspace_id: CONFIG.load(deps.storage)?.subspace_id.into(),
            post_id,
        },
    });
    let wasm_execute_mint_msg = wasm_execute(CW721_ADDRESS.load(deps.storage)?, &mint_msg, vec![])?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_MINT)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_RARITY_LEVEL, rarity_level.to_string())
        .add_attribute(ATTRIBUTE_TOKEN_ID, token_id.to_string())
        .add_attribute(ATTRIBUTE_RECIPIENT, &info.sender)
        .add_attribute(ATTRIBUTE_TOKEN_URI, remarkables_uri)
        .add_message(wasm_execute_mint_msg))
}

/// Returns the token id as "<post-id>-<rarity-level>".
pub fn convert_post_id_to_token_id(post_id: u64, rarity_level: u32) -> String {
    post_id.to_string() + "-" + &rarity_level.to_string()
}

/// Checks if the funds reach the required mint fees.
fn is_enough_fees(funds: Vec<Coin>, requireds: &Vec<Coin>) -> bool {
    if requireds.len() == 0 {
        return true;
    }
    // It takes O(n^2) time complexity but both list are extremely small
    for required in requireds.iter() {
        if !has_coins(&funds, &required) {
            return false;
        }
    }
    true
}

/// Checks that the post reaches the engagement threshold.
fn check_eligibility<'a>(
    storage: &dyn Storage,
    querier: &'a dyn Querier,
    sender: Addr,
    post_id: u64,
    engagement_threshold: u32,
) -> Result<(), ContractError> {
    let subspace_id = CONFIG.load(storage)?.subspace_id;
    // Check if the post exists and it is owned by sender.
    let post = PostsQuerier::new(querier)
        .query_post(subspace_id, post_id)
        .map_err(|_| ContractError::PostNotFound { id: post_id })?
        .post;
    if post.author != sender {
        return Err(ContractError::MinterNotPostAuthor {
            minter: sender.into(),
            author: post.author.into(),
        });
    }
    // Check if the reactions of the post is larger than the threshold.
    let total_reactions_count = ReactionsQuerier::new(querier)
        .query_reactions(
            subspace_id,
            post_id,
            None,
            Some(PageRequest {
                key: None,
                offset: None,
                limit: Uint64::new(1),
                count_total: true,
                reverse: false,
            }),
        )?
        .pagination
        .unwrap_or(PageResponse::default())
        .total
        .unwrap_or(0u64.into());
    let self_reactions_count = ReactionsQuerier::new(querier)
        .query_reactions(
            subspace_id,
            post_id,
            Some(sender),
            Some(PageRequest {
                key: None,
                offset: None,
                limit: Uint64::new(1),
                count_total: true,
                reverse: false,
            }),
        )?
        .pagination
        .unwrap_or(PageResponse::default())
        .total
        .unwrap_or(0u64.into());
    if engagement_threshold as u64
        > (total_reactions_count.checked_sub(self_reactions_count)?).into()
    {
        return Err(ContractError::NoEligibilityError {});
    }
    Ok(())
}

fn execute_update_admin(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    check_admin(deps.storage, &info)?;
    let new_admin_addr = deps.api.addr_validate(&new_admin)?;
    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        config.admin = new_admin_addr;
        Ok(config)
    })?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_ADMIN)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_ADMIN, &info.sender)
        .add_attribute(ATTRIBUTE_NEW_ADMIN, new_admin))
}

fn execute_update_rarity_mint_fees(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    level: u32,
    new_fees: Vec<Coin>,
) -> Result<Response<DesmosMsg>, ContractError> {
    check_admin(deps.storage, &info)?;
    RARITIES.update(deps.storage, |rarities| -> Result<_, ContractError> {
        let mut new_rarities = rarities;
        let new_rarity: &mut Rarity = new_rarities
            .get_mut(level as usize)
            .ok_or(ContractError::RarityNotExists { level })?;
        if new_rarity.mint_fees == new_fees {
            return Err(ContractError::NewMintFeesEqualToCurrent {});
        }
        new_rarity.mint_fees = new_fees;
        Ok(new_rarities)
    })?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_RARITY_MINT_FEES)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_RARITY_LEVEL, level.to_string()))
}

fn check_admin(storage: &dyn Storage, info: &MessageInfo) -> Result<(), ContractError> {
    let config = CONFIG.load(storage)?;
    if config.admin != info.sender {
        return Err(ContractError::NotAdmin {
            caller: info.sender.clone(),
        });
    }
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Rarities {} => to_binary(&query_rarities(deps)?),
        QueryMsg::AllNftInfo {
            token_id,
            include_expired,
        } => to_binary(&query_all_nft_info(deps, token_id, include_expired)?),
        QueryMsg::Tokens {
            owner,
            start_after,
            limit,
        } => to_binary(&query_tokens(deps, owner, start_after, limit)?),
    }
}

fn query_config(deps: Deps<DesmosQuery>) -> StdResult<QueryConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let cw721_address = CW721_ADDRESS.load(deps.storage)?;
    Ok(QueryConfigResponse {
        admin: config.admin,
        cw721_code_id: config.cw721_code_id.into(),
        cw721_address,
        subspace_id: config.subspace_id.into(),
    })
}

fn query_rarities(deps: Deps<DesmosQuery>) -> StdResult<QueryRaritiesResponse> {
    let rarities = RARITIES.load(deps.storage)?;
    Ok(QueryRaritiesResponse { rarities })
}

fn query_all_nft_info(
    deps: Deps<DesmosQuery>,
    token_id: String,
    include_expired: Option<bool>,
) -> StdResult<AllNftInfoResponse<Metadata>> {
    let cw721_address = CW721_ADDRESS.load(deps.storage)?;
    deps.querier.query_wasm_smart(
        cw721_address,
        &Cw721QueryMsg::<Empty>::AllNftInfo {
            token_id,
            include_expired,
        },
    )
}

fn query_tokens(
    deps: Deps<DesmosQuery>,
    owner: String,
    start_after: Option<String>,
    limit: Option<u32>,
) -> StdResult<TokensResponse> {
    let cw721_address = CW721_ADDRESS.load(deps.storage)?;
    deps.querier.query_wasm_smart(
        cw721_address,
        &Cw721QueryMsg::<Empty>::Tokens {
            owner,
            start_after,
            limit,
        },
    )
}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    msg: Reply,
) -> Result<Response<DesmosMsg>, ContractError> {
    if msg.id != INSTANTIATE_CW721_REPLY_ID {
        return Err(ContractError::InvalidReplyID {});
    }
    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => {
            CW721_ADDRESS.save(deps.storage, &Addr::unchecked(res.contract_address))?;
            Ok(Response::default().add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE_CW721_REPLY))
        }
        Err(_) => Err(ContractError::InstantiateCw721Error {}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::state::ConfigState;
    use cosmwasm_std::testing::{
        mock_env, mock_info, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{
        coins, from_binary, to_binary, ContractResult, OwnedDeps, StdError, SubMsgResponse,
        SubMsgResult, SystemError, SystemResult,
    };
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use desmos_bindings::mocks::mock_queriers::mock_desmos_dependencies;
    use desmos_bindings::{
        posts::{mocks::mock_posts_query_response, query::PostsQuery},
        reactions::{
            mocks::mock_reactions_query_response, models_query::QueryReactionsResponse,
            query::ReactionsQuery,
        },
        subspaces::{mocks::mock_subspaces_query_response, query::SubspacesQuery},
        types::PageResponse,
    };
    use std::marker::PhantomData;

    const ADMIN: &str = "cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t";
    const USER: &str = "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc";
    const NEW_ADMIN: &str = "new_admin";
    const SUBSPACE_ID: u64 = 1;
    const DENOM: &str = "test";
    const CW721_CODE_ID: u64 = 1;
    const MINT_FEES: u128 = 100;
    const POST_ID: u64 = 1;
    const RARITY_LEVEL: u32 = 0;
    const ENGAGEMENT_THRESHOLD: u32 = 100;

    fn get_instantiate_rarities() -> Vec<Rarity> {
        vec![Rarity {
            engagement_threshold: ENGAGEMENT_THRESHOLD,
            mint_fees: coins(MINT_FEES, DENOM),
        }]
    }
    fn get_valid_instantiate_msg() -> InstantiateMsg {
        InstantiateMsg {
            admin: ADMIN.into(),
            cw721_code_id: CW721_CODE_ID.into(),
            cw721_instantiate_msg: Cw721InstantiateMsg {
                minter: ADMIN.into(),
                name: "test".into(),
                symbol: "test".into(),
            },
            subspace_id: SUBSPACE_ID.into(),
            rarities: get_instantiate_rarities(),
        }
    }
    fn do_instantiate(deps: DepsMut<DesmosQuery>) {
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);
        let valid_msg = get_valid_instantiate_msg();
        instantiate(deps, env, info, valid_msg).unwrap();
    }
    mod instantiate {
        use super::*;
        #[test]
        fn instatiate_with_invalid_admin_address_error() {
            let mut deps = mock_desmos_dependencies();
            let env = mock_env();
            let info = mock_info(ADMIN, &vec![]);
            let mut invalid_msg = get_valid_instantiate_msg();
            invalid_msg.admin = "a".into();
            assert_eq!(
                instantiate(deps.as_mut(), env, info, invalid_msg).unwrap_err(),
                ContractError::Std(StdError::generic_err(
                    "Invalid input: human address too short"
                ))
            )
        }
        #[test]
        fn instatiate_with_non_existing_subspace_error() {
            let querier = MockQuerier::<DesmosQuery>::new(&[(MOCK_CONTRACT_ADDR, &[])])
                .with_custom_handler(|query| match query {
                    DesmosQuery::Subspaces(query) => match query {
                        SubspacesQuery::Subspace { .. } => {
                            SystemResult::Err(SystemError::InvalidRequest {
                                error: "subspace not found".to_string(),
                                request: Default::default(),
                            })
                        }
                        _ => SystemResult::Err(SystemError::Unknown {}),
                    },
                    _ => SystemResult::Err(SystemError::Unknown {}),
                });
            let mut deps = OwnedDeps {
                storage: MockStorage::default(),
                querier,
                api: MockApi::default(),
                custom_query_type: PhantomData,
            };
            let env = mock_env();
            let info = mock_info(NEW_ADMIN, &vec![]);
            let mut invalid_msg = get_valid_instantiate_msg();
            invalid_msg.admin = NEW_ADMIN.into();
            assert_eq!(
                instantiate(deps.as_mut(), env, info, invalid_msg).unwrap_err(),
                ContractError::SubspaceNotFound { id: SUBSPACE_ID }
            )
        }
        #[test]
        fn instatiate_without_permission_error() {
            let mut deps = mock_desmos_dependencies();
            let env = mock_env();
            let info = mock_info(NEW_ADMIN, &vec![]);
            let mut invalid_msg = get_valid_instantiate_msg();
            invalid_msg.admin = NEW_ADMIN.into();
            assert_eq!(
                instantiate(deps.as_mut(), env, info, invalid_msg).unwrap_err(),
                ContractError::NotSubspaceOwner {
                    caller: Addr::unchecked(NEW_ADMIN)
                }
            )
        }
        #[test]
        fn instatiate_properly() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let config = CONFIG.load(&deps.storage).unwrap();
            let expected_config = ConfigState {
                admin: Addr::unchecked(ADMIN),
                cw721_code_id: CW721_CODE_ID,
                subspace_id: SUBSPACE_ID,
            };
            assert_eq!(config, expected_config);

            let rarities: Vec<Rarity> = RARITIES.load(&deps.storage).unwrap();
            let expected_rarities = get_instantiate_rarities();
            assert_eq!(expected_rarities, rarities)
        }
    }
    mod reply {
        use super::*;
        #[test]
        fn cw721_instantiate_with_invalid_reply_id_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let result = reply(
                deps.as_mut(),
                env,
                Reply {
                    id: 2,
                    result: SubMsgResult::Ok(SubMsgResponse {
                        events: vec![],
                        data: None,
                    }),
                },
            );
            assert_eq!(result.unwrap_err(), ContractError::InvalidReplyID {},)
        }
        #[test]
        fn cw721_instantiate_with_invalid_instantiate_msg_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let result = reply(
                deps.as_mut(),
                env,
                Reply {
                    id: 1,
                    result: SubMsgResult::Ok(SubMsgResponse {
                        events: vec![],
                        data: None,
                    }),
                },
            );
            assert_eq!(result.unwrap_err(), ContractError::InstantiateCw721Error {})
        }
    }
    mod mint {
        use super::*;
        #[test]
        fn mint_with_not_existing_rarity_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(USER, &vec![]);
            let msg = ExecuteMsg::Mint {
                post_id: 1u64.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: 2,
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::RarityNotExists { level: 2 }
            )
        }
        #[test]
        fn mint_with_empty_fees_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(USER, &vec![]);
            let msg = ExecuteMsg::Mint {
                post_id: 1u64.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: RARITY_LEVEL,
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::MintFeesNotEnough {},
            )
        }
        #[test]
        fn mint_with_other_denom_fees_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(USER, &coins(100, "other"));
            let msg = ExecuteMsg::Mint {
                post_id: 1u64.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: RARITY_LEVEL,
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::MintFeesNotEnough {},
            )
        }
        #[test]
        fn mint_without_enough_fees_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(USER, &coins(MINT_FEES - 1, DENOM));
            let msg = ExecuteMsg::Mint {
                post_id: 1u64.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: RARITY_LEVEL,
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::MintFeesNotEnough {},
            )
        }
        #[test]
        fn mint_with_non_existing_post_error() {
            let querier = MockQuerier::<DesmosQuery>::new(&[(MOCK_CONTRACT_ADDR, &[])])
                .with_custom_handler(|query| match query {
                    DesmosQuery::Posts(query) => match query {
                        PostsQuery::Post { .. } => SystemResult::Err(SystemError::InvalidRequest {
                            error: "post not found".to_string(),
                            request: Default::default(),
                        }),
                        _ => SystemResult::Err(SystemError::Unknown {}),
                    },
                    DesmosQuery::Subspaces(query) => {
                        SystemResult::Ok(mock_subspaces_query_response(query))
                    }
                    _ => SystemResult::Err(SystemError::Unknown {}),
                });
            let mut deps = OwnedDeps {
                storage: MockStorage::default(),
                querier,
                api: MockApi::default(),
                custom_query_type: PhantomData,
            };
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(USER, &coins(MINT_FEES, DENOM));
            let msg = ExecuteMsg::Mint {
                post_id: POST_ID.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: RARITY_LEVEL,
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::PostNotFound { id: POST_ID },
            )
        }
        #[test]
        fn mint_from_non_author_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(ADMIN, &coins(100, DENOM));
            let msg = ExecuteMsg::Mint {
                post_id: 1u64.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: RARITY_LEVEL,
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::MinterNotPostAuthor {
                    minter: ADMIN.into(),
                    author: USER.into()
                },
            )
        }
        #[test]
        fn mint_without_eligible_amount_reactions_error() {
            let querier = MockQuerier::<DesmosQuery>::new(&[(MOCK_CONTRACT_ADDR, &[])])
                .with_custom_handler(|query| match query {
                    DesmosQuery::Posts(query) => SystemResult::Ok(mock_posts_query_response(query)),
                    DesmosQuery::Subspaces(query) => {
                        SystemResult::Ok(mock_subspaces_query_response(query))
                    }
                    DesmosQuery::Reactions(query) => {
                        SystemResult::Ok(mock_reactions_query_response(query))
                    }
                    #[allow(unreachable_patterns)]
                    _ => SystemResult::Err(SystemError::Unknown {}),
                });
            let mut deps = OwnedDeps {
                storage: MockStorage::default(),
                querier,
                api: MockApi::default(),
                custom_query_type: PhantomData,
            };
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(USER, &coins(MINT_FEES, DENOM));
            let msg = ExecuteMsg::Mint {
                post_id: POST_ID.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: RARITY_LEVEL,
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::NoEligibilityError {},
            )
        }
        fn get_reaction_response(number: u32) -> QueryReactionsResponse {
            QueryReactionsResponse {
                reactions: vec![],
                pagination: Some(PageResponse {
                    next_key: None,
                    total: Some(number.into()),
                }),
            }
        }
        fn get_reactions(user: &Option<Addr>, enough: bool) -> QueryReactionsResponse {
            let self_reactions_count = 1;
            if *user == Some(Addr::unchecked(USER)) {
                return get_reaction_response(self_reactions_count);
            }
            if !enough {
                return get_reaction_response(ENGAGEMENT_THRESHOLD);
            }
            get_reaction_response(ENGAGEMENT_THRESHOLD + self_reactions_count)
        }
        #[test]
        fn mint_without_eligible_amount_reactions_after_sub_self_reactions_error() {
            let querier = MockQuerier::<DesmosQuery>::new(&[(MOCK_CONTRACT_ADDR, &[])])
                .with_custom_handler(|query| match query {
                    DesmosQuery::Posts(query) => SystemResult::Ok(mock_posts_query_response(query)),
                    DesmosQuery::Subspaces(query) => {
                        SystemResult::Ok(mock_subspaces_query_response(query))
                    }
                    DesmosQuery::Reactions(query) => match query {
                        ReactionsQuery::Reactions { user, .. } => SystemResult::Ok(
                            ContractResult::Ok(to_binary(&get_reactions(user, false)).unwrap()),
                        ),
                        _ => SystemResult::Err(SystemError::Unknown {}),
                    },
                    #[allow(unreachable_patterns)]
                    _ => SystemResult::Err(SystemError::Unknown {}),
                });
            let mut deps = OwnedDeps {
                storage: MockStorage::default(),
                querier,
                api: MockApi::default(),
                custom_query_type: PhantomData,
            };
            do_instantiate(deps.as_mut());
            CW721_ADDRESS
                .save(deps.as_mut().storage, &Addr::unchecked("cw_address"))
                .unwrap();
            let env = mock_env();
            let info = mock_info(USER, &coins(MINT_FEES, DENOM));
            let msg = ExecuteMsg::Mint {
                post_id: POST_ID.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: RARITY_LEVEL,
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::NoEligibilityError {},
            )
        }
        #[test]
        fn mint_existing_token_error() {
            let querier = MockQuerier::<DesmosQuery>::new(&[(MOCK_CONTRACT_ADDR, &[])])
                .with_custom_handler(|query| match query {
                    DesmosQuery::Posts(query) => SystemResult::Ok(mock_posts_query_response(query)),
                    DesmosQuery::Subspaces(query) => {
                        SystemResult::Ok(mock_subspaces_query_response(query))
                    }
                    DesmosQuery::Reactions(query) => match query {
                        ReactionsQuery::Reactions { user, .. } => SystemResult::Ok(
                            ContractResult::Ok(to_binary(&get_reactions(user, true)).unwrap()),
                        ),
                        _ => SystemResult::Err(SystemError::Unknown {}),
                    },
                    #[allow(unreachable_patterns)]
                    _ => SystemResult::Err(SystemError::Unknown {}),
                });
            let mut deps = OwnedDeps {
                storage: MockStorage::default(),
                querier,
                api: MockApi::default(),
                custom_query_type: PhantomData,
            };
            do_instantiate(deps.as_mut());
            MINTED_TOKEN
                .save(
                    deps.as_mut().storage,
                    convert_post_id_to_token_id(POST_ID, RARITY_LEVEL),
                    &true,
                )
                .unwrap();
            CW721_ADDRESS
                .save(deps.as_mut().storage, &Addr::unchecked("cw_address"))
                .unwrap();
            let env = mock_env();
            let info = mock_info(USER, &coins(MINT_FEES, DENOM));
            let msg = ExecuteMsg::Mint {
                post_id: POST_ID.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: RARITY_LEVEL,
            };
            assert_eq!(
                ContractError::TokenHasBeenMinted {
                    token_id: convert_post_id_to_token_id(POST_ID, RARITY_LEVEL)
                },
                execute(deps.as_mut(), env, info, msg).unwrap_err()
            );
        }
        #[test]
        fn mint_properly() {
            let querier = MockQuerier::<DesmosQuery>::new(&[(MOCK_CONTRACT_ADDR, &[])])
                .with_custom_handler(|query| match query {
                    DesmosQuery::Posts(query) => SystemResult::Ok(mock_posts_query_response(query)),
                    DesmosQuery::Subspaces(query) => {
                        SystemResult::Ok(mock_subspaces_query_response(query))
                    }
                    DesmosQuery::Reactions(query) => match query {
                        ReactionsQuery::Reactions { user, .. } => SystemResult::Ok(
                            ContractResult::Ok(to_binary(&get_reactions(user, true)).unwrap()),
                        ),
                        _ => SystemResult::Err(SystemError::Unknown {}),
                    },
                    #[allow(unreachable_patterns)]
                    _ => SystemResult::Err(SystemError::Unknown {}),
                });
            let mut deps = OwnedDeps {
                storage: MockStorage::default(),
                querier,
                api: MockApi::default(),
                custom_query_type: PhantomData,
            };
            do_instantiate(deps.as_mut());
            CW721_ADDRESS
                .save(deps.as_mut().storage, &Addr::unchecked("cw_address"))
                .unwrap();
            let env = mock_env();
            let info = mock_info(USER, &coins(MINT_FEES, DENOM));
            let msg = ExecuteMsg::Mint {
                post_id: POST_ID.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: RARITY_LEVEL,
            };
            execute(deps.as_mut(), env, info, msg).unwrap();
        }
    }
    mod update_admin {
        use super::*;
        #[test]
        fn update_admin_without_permissions_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(USER, &vec![]);
            let msg = ExecuteMsg::UpdateAdmin {
                new_admin: NEW_ADMIN.into(),
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::NotAdmin {
                    caller: Addr::unchecked(USER)
                }
            )
        }
        #[test]
        fn update_admin_with_invalid_address_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(ADMIN, &vec![]);
            let msg = ExecuteMsg::UpdateAdmin {
                new_admin: "a".into(),
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::Std(StdError::generic_err(
                    "Invalid input: human address too short"
                ))
            )
        }
        #[test]
        fn update_admin_properly() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(ADMIN, &vec![]);
            let msg = ExecuteMsg::UpdateAdmin {
                new_admin: NEW_ADMIN.into(),
            };
            assert!(execute(deps.as_mut(), env, info, msg).is_ok());
            let config = CONFIG.load(&deps.storage).unwrap();
            let expected = ConfigState {
                admin: Addr::unchecked(NEW_ADMIN),
                cw721_code_id: CW721_CODE_ID,
                subspace_id: SUBSPACE_ID,
            };
            assert_eq!(config, expected)
        }
    }

    mod update_rarity_mint_fees {
        use super::*;
        #[test]
        fn update_rarity_mint_fees_without_permissions_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(USER, &vec![]);
            let msg = ExecuteMsg::UpdateRarityMintFees {
                rarity_level: RARITY_LEVEL,
                new_fees: coins(50, DENOM),
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::NotAdmin {
                    caller: Addr::unchecked(USER)
                }
            )
        }
        #[test]
        fn update_no_existing_rarity_mint_fees_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(ADMIN, &vec![]);
            let msg = ExecuteMsg::UpdateRarityMintFees {
                rarity_level: 2,
                new_fees: coins(50, DENOM),
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::RarityNotExists { level: 2 },
            )
        }
        #[test]
        fn update_rarity_mint_fees_same_as_the_current_error() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(ADMIN, &vec![]);
            let msg = ExecuteMsg::UpdateRarityMintFees {
                rarity_level: RARITY_LEVEL,
                new_fees: coins(MINT_FEES, DENOM),
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::NewMintFeesEqualToCurrent {},
            )
        }
        #[test]
        fn update_rarity_mint_fees_properly() {
            let mut deps = mock_desmos_dependencies();
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(ADMIN, &vec![]);
            let msg = ExecuteMsg::UpdateRarityMintFees {
                rarity_level: RARITY_LEVEL,
                new_fees: coins(50, DENOM),
            };
            execute(deps.as_mut(), env, info, msg).unwrap();
            let new_rarities = RARITIES.load(&deps.storage).unwrap();
            let expected = Rarity {
                engagement_threshold: 100,
                mint_fees: coins(50, DENOM),
            };
            assert_eq!(expected, *new_rarities.get(0).unwrap())
        }
    }
    mod query {
        use super::*;
        #[test]
        fn query_config() {
            let mut deps = mock_desmos_dependencies();
            let env = mock_env();
            CONFIG
                .save(
                    deps.as_mut().storage,
                    &ConfigState {
                        admin: Addr::unchecked(ADMIN),
                        cw721_code_id: 1u64,
                        subspace_id: SUBSPACE_ID,
                    },
                )
                .unwrap();
            CW721_ADDRESS
                .save(deps.as_mut().storage, &Addr::unchecked("cw721_address"))
                .unwrap();
            let bz = query(deps.as_ref(), env, QueryMsg::Config {}).unwrap();
            let config: QueryConfigResponse = from_binary(&bz).unwrap();
            assert_eq!(
                QueryConfigResponse {
                    admin: Addr::unchecked(ADMIN),
                    cw721_code_id: 1u64.into(),
                    subspace_id: SUBSPACE_ID.into(),
                    cw721_address: Addr::unchecked("cw721_address"),
                },
                config
            )
        }
        #[test]
        fn query_rarities() {
            let mut deps = mock_desmos_dependencies();
            let env = mock_env();
            RARITIES
                .save(
                    deps.as_mut().storage,
                    &vec![Rarity {
                        engagement_threshold: 100,
                        mint_fees: coins(1, DENOM),
                    }],
                )
                .unwrap();
            let bz = query(deps.as_ref(), env, QueryMsg::Rarities {}).unwrap();
            let rarities_response: QueryRaritiesResponse = from_binary(&bz).unwrap();
            assert_eq!(
                QueryRaritiesResponse {
                    rarities: vec![Rarity {
                        engagement_threshold: 100,
                        mint_fees: coins(1, DENOM)
                    }]
                },
                rarities_response
            )
        }
    }
}
