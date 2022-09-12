#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    has_coins, to_binary, wasm_execute, wasm_instantiate, Addr, Binary, Coin, Deps, DepsMut, Empty,
    Env, MessageInfo, Order, Querier, Reply, Response, StdResult, Storage, SubMsg, Uint64,
};
use cw2::set_contract_version;
use cw721::{AllNftInfoResponse, TokensResponse};
use cw721_base::{
    ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg, MintMsg,
    QueryMsg as Cw721QueryMsg,
};
use cw_utils::parse_reply_instantiate_data;
use desmos_bindings::{
    msg::DesmosMsg,
    posts::querier::PostsQuerier,
    query::DesmosQuery,
    reactions::querier::ReactionsQuerier,
    types::{PageRequest, PageResponse},
};
use std::ops::Deref;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, QueryRaritiesResponse, Rarity,
};
use crate::state::{ConfigState, RarityState, CONFIG, CW721_ADDRESS, RARITY};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:remarkables";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

// actions for executing messages
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_INSTANTIATE_CW721_REPLY: &str = "instantiate_cw721_reply";
const ACTION_MINT_TO: &str = "mint_to";
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
    for rarity in msg.rarities {
        let state = RarityState {
            level: rarity.level,
            mint_fees: rarity.mint_fees,
            engagement_threshold: rarity.engagement_threshold,
        };
        RARITY.save(deps.storage, rarity.level, &state)?;
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
        ExecuteMsg::MintTo {
            post_id,
            remarkables_uri,
            rarity_level,
        } => execute_mint_to(deps, info, rarity_level, post_id.into(), remarkables_uri),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
        ExecuteMsg::UpdateRarityMintFee {
            rarity_level,
            new_fees,
        } => execute_update_rarity_mint_fees(deps, info, rarity_level, new_fees),
    }
}

fn execute_mint_to(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    rarity_level: u32,
    post_id: u64,
    remarkables_uri: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let rarity = RARITY.load(deps.storage, rarity_level)?;
    // Check if rarity mint fees is enough
    if !is_enough_fees(info.funds, rarity.mint_fees) {
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
    // Create the cw721 message to send to mint the remarkables
    let mint_msg = Cw721ExecuteMsg::<Empty, Empty>::Mint(MintMsg::<Empty> {
        token_id: post_id.to_string(),
        owner: info.sender.clone().into(),
        token_uri: Some(remarkables_uri.clone()),
        extension: Empty {},
    });
    let wasm_execute_mint_msg = wasm_execute(CW721_ADDRESS.load(deps.storage)?, &mint_msg, vec![])?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_MINT_TO)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_RARITY_LEVEL, rarity_level.to_string())
        .add_attribute(ATTRIBUTE_TOKEN_ID, post_id.to_string())
        .add_attribute(ATTRIBUTE_RECIPIENT, &info.sender)
        .add_attribute(ATTRIBUTE_TOKEN_URI, remarkables_uri)
        .add_message(wasm_execute_mint_msg))
}

fn is_enough_fees(funds: Vec<Coin>, requireds: Vec<Coin>) -> bool {
    if requireds.len() == 0 {
        return true;
    }
    // It takes O(n^2) time complexity but both list are extremely small
    for required in requireds.iter() {
        if has_coins(&funds, &required) {
            return true;
        }
    }
    false
}

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
        .query_post(subspace_id, post_id)?
        .post;
    if post.author != sender {
        return Err(ContractError::NoEligibilityError {});
    }
    // Check if the reactions of the post is larger than the threshold.
    let reactions = ReactionsQuerier::new(querier).query_reactions(
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
    )?;
    let reactions_pagination = reactions.pagination.unwrap_or(PageResponse::default());
    let reactions_count = reactions_pagination.total.unwrap_or(0u64.into());
    if engagement_threshold as u64 > reactions_count.into() {
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
        config.admin = new_admin_addr.clone();
        Ok(config)
    })?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_ADMIN)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_ADMIN, &info.sender)
        .add_attribute(ATTRIBUTE_NEW_ADMIN, new_admin_addr))
}

fn execute_update_rarity_mint_fees(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    level: u32,
    new_fees: Vec<Coin>,
) -> Result<Response<DesmosMsg>, ContractError> {
    check_admin(deps.storage, &info)?;
    RARITY.update(deps.storage, level, |rarity| -> Result<_, ContractError> {
        let mut new_rarity = rarity.ok_or(ContractError::RarityNotExists { level })?;
        new_rarity.mint_fees = new_fees;
        Ok(new_rarity)
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
    let res: StdResult<Vec<_>> = RARITY
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let rarities = res?
        .iter()
        .map(|(level, rarity)| Rarity {
            level: *level,
            engagement_threshold: rarity.engagement_threshold,
            mint_fees: rarity.mint_fees.clone(),
        })
        .collect();
    Ok(QueryRaritiesResponse { rarities })
}

fn query_all_nft_info(
    deps: Deps<DesmosQuery>,
    token_id: String,
    include_expired: Option<bool>,
) -> StdResult<AllNftInfoResponse<Empty>> {
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
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, StdError, SubMsgResponse, SubMsgResult};
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use desmos_bindings::mocks::mock_queriers::mock_dependencies_with_custom_querier;

    const ADMIN: &str = "admin";
    const NEW_ADMIN: &str = "new_admin";
    const SUBSPACE_ID: u64 = 1;
    const DENOM: &str = "test";
    const CW721_CODE_ID: u64 = 1;

    fn get_instantiate_rarities() -> Vec<Rarity> {
        vec![Rarity {
            level: 1,
            engagement_threshold: 100,
            mint_fees: coins(100, DENOM),
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
            let mut deps = mock_dependencies_with_custom_querier(&[]);
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
        fn instatiate_properly() {
            let mut deps = mock_dependencies_with_custom_querier(&[]);
            do_instantiate(deps.as_mut());
            let config = CONFIG.load(&deps.storage).unwrap();
            let expected_config = ConfigState {
                admin: Addr::unchecked(ADMIN),
                cw721_code_id: CW721_CODE_ID,
                subspace_id: SUBSPACE_ID,
            };
            assert_eq!(config, expected_config);

            let res: StdResult<Vec<_>> = RARITY
                .range(&deps.storage, None, None, Order::Ascending)
                .collect();
            let rarities: Vec<Rarity> = res
                .unwrap()
                .iter()
                .map(|(level, rarity)| Rarity {
                    level: *level,
                    engagement_threshold: rarity.engagement_threshold,
                    mint_fees: rarity.mint_fees.clone(),
                })
                .collect();
            let expected_rarities = get_instantiate_rarities();
            assert_eq!(expected_rarities, rarities)
        }
    }

    mod reply {
        use super::*;
        #[test]
        fn cw721_instantiate_with_invalid_reply_id_error() {
            let mut deps = mock_dependencies_with_custom_querier(&[]);
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
            let mut deps = mock_dependencies_with_custom_querier(&[]);
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

    mod mint_to {
        use super::*;
        #[test]
        fn mint_to_with_not_existing_rarity_error() {
            let mut deps = mock_dependencies_with_custom_querier(&[]);
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(NEW_ADMIN, &vec![]);
            let msg = ExecuteMsg::MintTo {
                post_id: 1u64.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: 2,
            };
            assert!(execute(deps.as_mut(), env, info, msg).is_err())
        }
        #[test]
        fn mint_to_without_enough_fees_error() {
            let mut deps = mock_dependencies_with_custom_querier(&[]);
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(NEW_ADMIN, &vec![]);
            let msg = ExecuteMsg::MintTo {
                post_id: 1u64.into(),
                remarkables_uri: "ipfs://test.com".into(),
                rarity_level: 1,
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::MintFeesNotEnough {},
            )
        }
    }

    mod update_admin {
        use super::*;
        #[test]
        fn update_admin_without_permissions_error() {
            let mut deps = mock_dependencies_with_custom_querier(&[]);
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(NEW_ADMIN, &vec![]);
            let msg = ExecuteMsg::UpdateAdmin {
                new_admin: NEW_ADMIN.into(),
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::NotAdmin {
                    caller: Addr::unchecked(NEW_ADMIN)
                }
            )
        }
        #[test]
        fn update_admin_with_invalid_address_error() {
            let mut deps = mock_dependencies_with_custom_querier(&[]);
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
            let mut deps = mock_dependencies_with_custom_querier(&[]);
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
            let mut deps = mock_dependencies_with_custom_querier(&[]);
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(NEW_ADMIN, &vec![]);
            let msg = ExecuteMsg::UpdateRarityMintFee {
                rarity_level: 1,
                new_fees: coins(50, DENOM),
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::NotAdmin {
                    caller: Addr::unchecked(NEW_ADMIN)
                }
            )
        }
        #[test]
        fn update_no_existing_rarity_mint_fees_error() {
            let mut deps = mock_dependencies_with_custom_querier(&[]);
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(ADMIN, &vec![]);
            let msg = ExecuteMsg::UpdateRarityMintFee {
                rarity_level: 2,
                new_fees: coins(50, DENOM),
            };
            assert_eq!(
                execute(deps.as_mut(), env, info, msg).unwrap_err(),
                ContractError::RarityNotExists { level: 2 },
            )
        }
        #[test]
        fn update_rarity_mint_fees_properly() {
            let mut deps = mock_dependencies_with_custom_querier(&[]);
            do_instantiate(deps.as_mut());
            let env = mock_env();
            let info = mock_info(ADMIN, &vec![]);
            let msg = ExecuteMsg::UpdateRarityMintFee {
                rarity_level: 1,
                new_fees: coins(50, DENOM),
            };
            execute(deps.as_mut(), env, info, msg).unwrap();
            let new_rarity = RARITY.load(&deps.storage, 1).unwrap();
            let expected = RarityState {
                level: 1,
                engagement_threshold: 100,
                mint_fees: coins(50, DENOM),
            };
            assert_eq!(expected, new_rarity);
        }
    }
}
