use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, ServiceFee, Target, Tip,
    TipsResponse,
};
use crate::state::{Config, TipsRecordKey, CONFIG, TIPS_KEY_LIST, TIPS_RECORD};
use crate::utils;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Uint64,
};
use cw2::set_contract_version;
use desmos_bindings::posts::querier::PostsQuerier;
use desmos_bindings::subspaces::querier::SubspacesQuerier;
use desmos_bindings::{msg::DesmosMsg, query::DesmosQuery};
use std::collections::vec_deque::VecDeque;
use std::convert::TryInto;
use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tips";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// actions for executing messages
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_SEND_TIP: &str = "send_tip";
const ACTION_UPDATE_SERVICE_FEE: &str = "update_service_fee";
const ACTION_UPDATE_ADMIN: &str = "update_admin";
const ACTION_UPDATE_SAVED_TIPS_RECORD_THRESHOLD: &str = "update_saved_tips_record_threshold";
const ACTION_CLAIM_FEES: &str = "claim_fees";

// attributes for executing messages
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_SENDER: &str = "sender";
const ATTRIBUTE_ADMIN: &str = "admin";
const ATTRIBUTE_SUBSPACE_ID: &str = "subspace_id";
const ATTRIBUTE_TIPS_RECORD_THRESHOLD: &str = "tips_record_threshold";
const ATTRIBUTE_NEW_ADMIN: &str = "new_admin";
const ATTRIBUTE_NEW_THRESHOLD: &str = "new_threshold";
const ATTRIBUTE_RECEIVER: &str = "receiver";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    msg.validate()?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = deps.api.addr_validate(&msg.admin)?;
    // Ensure that the subspace exists
    SubspacesQuerier::new(deps.querier.deref())
        .query_subspace(msg.subspace_id.u64())
        .map_err(|error| ContractError::SubspaceNotExist {
            id: msg.subspace_id.u64(),
            error,
        })?;

    CONFIG.save(
        deps.storage,
        &Config {
            admin,
            subspace_id: msg.subspace_id.u64(),
            service_fee: msg.service_fee.try_into()?,
            tips_record_threshold: msg.saved_tips_threshold,
        },
    )?;
    TIPS_KEY_LIST.save(deps.storage, &VecDeque::new())?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE)
        .add_attribute(ATTRIBUTE_SENDER, info.sender)
        .add_attribute(ATTRIBUTE_ADMIN, msg.admin)
        .add_attribute(ATTRIBUTE_SUBSPACE_ID, msg.subspace_id)
        .add_attribute(
            ATTRIBUTE_TIPS_RECORD_THRESHOLD,
            msg.saved_tips_threshold.to_string(),
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    msg.validate()?;

    match msg {
        ExecuteMsg::SendTip { target } => execute_send_tip(deps, info, target),
        ExecuteMsg::UpdateServiceFee { new_fee } => execute_update_service_fee(deps, info, new_fee),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
        ExecuteMsg::UpdateSavedTipsRecordThreshold { new_threshold } => {
            execute_update_saved_tips_record_threshold(deps, info, new_threshold)
        }
        ExecuteMsg::ClaimFees { receiver } => execute_claim_fees(deps, env, info, receiver),
    }
}

fn execute_send_tip(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    target: Target,
) -> Result<Response<DesmosMsg>, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // Computes the fee and the coins to be sent to the user
    let (_, coin_to_send) = config.service_fee.compute_fees(info.funds)?;

    // Resolve the receiver and the optional post id
    let (post_id, receiver) = match target {
        Target::ContentTarget { post_id } => {
            let post = PostsQuerier::new(deps.querier.deref())
                .query_post(config.subspace_id, post_id.u64())?
                .post;
            (post.id.u64(), post.author)
        }
        Target::UserTarget { receiver } => {
            // Set the post id to 0 (invalid id) to signal that this tip is referencing an user
            (0_u64, deps.api.addr_validate(&receiver)?)
        }
    };

    if config.tips_record_threshold > 0 {
        let tip_key: TipsRecordKey = (info.sender.clone(), receiver.clone(), post_id);
        let tip_record_key = TIPS_RECORD.key(tip_key.clone());
        let tip_record_coins = if let Some(mut coins) = tip_record_key.may_load(deps.storage)? {
            // Append the new coins
            coins.extend(coin_to_send.clone());
            utils::merge_coins(coins)
        } else {
            // Load the key list
            let mut tips_key_list = TIPS_KEY_LIST.load(deps.storage)?;
            // If we have reached the threshold remove the oldest key
            if tips_key_list.len() == config.tips_record_threshold as usize {
                TIPS_RECORD.remove(deps.storage, tips_key_list.pop_front().unwrap());
            }
            // Add the new key to the end
            tips_key_list.push_back(tip_key);
            TIPS_KEY_LIST.save(deps.storage, &tips_key_list)?;
            // Return the new coins
            coin_to_send.clone()
        };
        tip_record_key.save(deps.storage, &tip_record_coins)?;
    }

    if coin_to_send.is_empty() {
        return Err(ContractError::FoundAmountTooSmall {});
    }

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_SEND_TIP)
        .add_attribute(ATTRIBUTE_SENDER, info.sender)
        .add_message(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: coin_to_send,
        }))
}

fn execute_update_service_fee(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    service_fee: ServiceFee,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    config.service_fee = service_fee.try_into()?;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_SERVICE_FEE)
        .add_attribute(ATTRIBUTE_SENDER, info.sender))
}

fn execute_update_admin(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let new_admin = deps.api.addr_validate(&new_admin)?;
    CONFIG.update(deps.storage, |mut config| {
        if config.admin != info.sender {
            return Err(ContractError::Unauthorized {});
        }
        config.admin = new_admin.clone();
        Ok(config)
    })?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_ADMIN)
        .add_attribute(ATTRIBUTE_SENDER, info.sender)
        .add_attribute(ATTRIBUTE_NEW_ADMIN, new_admin))
}

fn execute_update_saved_tips_record_threshold(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    new_threshold: u32,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // The new size is smaller of the current one maybe we have to shrink the tips record
    if config.tips_record_threshold > new_threshold {
        let mut keys = TIPS_KEY_LIST.load(deps.storage)?;
        // If we have more tips that the allowed threshold shrink the tips record
        if keys.len() > new_threshold as usize {
            let to_remove = keys.len() - new_threshold as usize;
            for _ in 0..to_remove {
                if let Some(key) = keys.pop_front() {
                    TIPS_RECORD.remove(deps.storage, key);
                }
            }
            TIPS_KEY_LIST.save(deps.storage, &keys)?;
        }
    }

    config.tips_record_threshold = new_threshold;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_SAVED_TIPS_RECORD_THRESHOLD)
        .add_attribute(ATTRIBUTE_SENDER, info.sender)
        .add_attribute(ATTRIBUTE_NEW_THRESHOLD, new_threshold.to_string()))
}

fn execute_claim_fees(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    receiver: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let receiver = deps.api.addr_validate(&receiver)?;
    let contract_balance = deps
        .querier
        .query_all_balances(env.contract.address.as_str())?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_CLAIM_FEES)
        .add_attribute(ATTRIBUTE_SENDER, info.sender)
        .add_attribute(ATTRIBUTE_RECEIVER, receiver.as_str())
        .add_message(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: contract_balance,
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> Result<Binary, ContractError> {
    match msg {
        QueryMsg::Config { .. } => to_binary(&query_config(deps)?),
        QueryMsg::UserReceivedTips { user } => to_binary(&query_tips(
            deps,
            None,
            Some(deps.api.addr_validate(&user)?),
            None,
        )?),
        QueryMsg::UserSentTips { user } => to_binary(&query_tips(
            deps,
            Some(deps.api.addr_validate(&user)?),
            None,
            None,
        )?),
        QueryMsg::PostReceivedTips { post_id } => {
            to_binary(&query_tips(deps, None, None, Some(post_id))?)
        }
    }
    .map_err(ContractError::from)
}

pub fn query_config(deps: Deps<DesmosQuery>) -> StdResult<QueryConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(QueryConfigResponse {
        admin: config.admin,
        subspace_id: config.subspace_id.into(),
        service_fee: config.service_fee.into(),
        saved_tips_record_threshold: config.tips_record_threshold,
    })
}

pub fn query_tips(
    deps: Deps<DesmosQuery>,
    sender: Option<Addr>,
    receiver: Option<Addr>,
    post_id: Option<Uint64>,
) -> Result<TipsResponse, ContractError> {
    if post_id.is_some() && post_id.unwrap().is_zero() {
        return Err(ContractError::InvalidPostId {});
    }

    let tips_tuple: StdResult<Vec<(TipsRecordKey, Vec<Coin>)>> = TIPS_RECORD
        .range(deps.storage, None, None, Order::Ascending)
        .filter(|tuple| {
            if let Ok(((t_sender, t_receiver, t_post_id), _)) = tuple {
                if sender.is_some() && !sender.as_ref().unwrap().eq(t_sender) {
                    return false;
                }
                if receiver.is_some() && !receiver.as_ref().unwrap().eq(t_receiver) {
                    return false;
                }
                if post_id.is_some() && *t_post_id != post_id.as_ref().unwrap().u64() {
                    return false;
                }
                true
            } else {
                false
            }
        })
        .collect();

    Ok(TipsResponse {
        tips: tips_tuple?
            .iter()
            .map(|tuple| Tip {
                sender: tuple.0 .0.clone(),
                receiver: tuple.0 .1.clone(),
                amount: tuple.1.clone(),
            })
            .collect(),
    })
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::error::ContractError;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ServiceFee, Target, Tip, TipsResponse};
    use crate::state::{StateServiceFee, TipsRecordKey, CONFIG, TIPS_KEY_LIST, TIPS_RECORD};
    use cosmwasm_std::testing::{
        mock_env, mock_info, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{
        from_binary, Addr, BankMsg, Coin, DepsMut, Order, OwnedDeps, Response, StdError, StdResult,
        SubMsg, SystemError, SystemResult, Uint128, Uint64,
    };
    use desmos_bindings::mocks::mock_queriers::mock_dependencies_with_custom_querier;
    use desmos_bindings::msg::DesmosMsg;
    use desmos_bindings::posts::mocks::MockPostsQueries;
    use desmos_bindings::posts::query::PostsQuery;
    use desmos_bindings::query::DesmosQuery;
    use desmos_bindings::subspaces::mocks::mock_subspaces_query_response;
    use desmos_bindings::subspaces::query::SubspacesQuery;
    use std::collections::VecDeque;
    use std::marker::PhantomData;

    const ADMIN: &str = "admin";
    const USER_1: &str = "user1";
    const USER_2: &str = "user2";
    const USER_3: &str = "user3";

    fn init_contract(
        deps: DepsMut<DesmosQuery>,
        subspace_id: u64,
        service_fee: ServiceFee,
        saved_tips_threshold: u32,
    ) -> Result<Response<DesmosMsg>, ContractError> {
        instantiate(
            deps,
            mock_env(),
            mock_info(ADMIN, &[]),
            InstantiateMsg {
                admin: ADMIN.to_string(),
                subspace_id: subspace_id.into(),
                service_fee,
                saved_tips_threshold,
            },
        )
    }

    fn tip_user(
        deps: DepsMut<DesmosQuery>,
        from: &str,
        to: &str,
        coins: &[Coin],
    ) -> Result<Response<DesmosMsg>, ContractError> {
        let info = mock_info(from, coins);
        execute(
            deps,
            mock_env(),
            info,
            ExecuteMsg::SendTip {
                target: Target::UserTarget {
                    receiver: to.to_string(),
                },
            },
        )
    }

    fn tip_post(
        deps: DepsMut<DesmosQuery>,
        from: &str,
        post_id: u64,
        coins: &[Coin],
    ) -> Result<Response<DesmosMsg>, ContractError> {
        let info = mock_info(from, coins);
        execute(
            deps,
            mock_env(),
            info,
            ExecuteMsg::SendTip {
                target: Target::ContentTarget {
                    post_id: post_id.into(),
                },
            },
        )
    }

    fn get_tips_record_items(deps: DepsMut<DesmosQuery>) -> Vec<(TipsRecordKey, Vec<Coin>)> {
        TIPS_RECORD
            .range(deps.storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<(TipsRecordKey, Vec<Coin>)>>>()
            .unwrap()
    }

    #[test]
    fn init_contract_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, ServiceFee::Fixed { amount: vec![] }, 1).unwrap();
    }

    #[test]
    fn init_contract_with_invalid_subspace_id() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        let init_err =
            init_contract(deps.as_mut(), 0, ServiceFee::Fixed { amount: vec![] }, 1).unwrap_err();
        assert_eq!(ContractError::InvalidSubspaceId {}, init_err);
    }

    #[test]
    fn init_contract_with_non_existing_subspace() {
        let querier = MockQuerier::<DesmosQuery>::new(&[(MOCK_CONTRACT_ADDR, &[])])
            .with_custom_handler(|query| match query {
                DesmosQuery::Subspaces(subspaces_query) => match subspaces_query {
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

        let init_err =
            init_contract(deps.as_mut(), 2, ServiceFee::Fixed { amount: vec![] }, 1).unwrap_err();

        assert_eq!(
            ContractError::SubspaceNotExist {
                id: 2,
                error: StdError::generic_err(
                    "Querier system error: Cannot parse request: subspace not found in: "
                )
            },
            init_err
        );
    }

    #[test]
    fn init_contract_with_invalid_percentage_service_fees() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        // Simulate init with 100% of service fees
        let init_err = init_contract(
            deps.as_mut(),
            2,
            ServiceFee::Percentage {
                value: Uint128::new(100),
                decimals: 0,
            },
            1,
        )
        .unwrap_err();

        assert_eq!(ContractError::InvalidPercentageFee {}, init_err);
    }

    #[test]
    fn tip_user_with_invalid_address() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm"), Coin::new(100, "udsm")],
            },
            5,
        )
        .unwrap();

        let tip_error =
            tip_user(deps.as_mut(), USER_1, "a", &[Coin::new(5000, "udsm")]).unwrap_err();

        assert_eq!(
            ContractError::Std(StdError::generic_err(
                "Invalid input: human address too short"
            )),
            tip_error
        );
    }

    #[test]
    fn tip_user_with_missing_fee_coin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm"), Coin::new(100, "uatom")],
            },
            5,
        )
        .unwrap();

        let tip_error =
            tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(5000, "udsm")]).unwrap_err();

        assert_eq!(
            ContractError::FeeCoinNotProvided {
                denom: "uatom".to_string()
            },
            tip_error
        );
    }

    #[test]
    fn tip_user_with_insufficient_fees() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(5000, "udsm")],
            },
            5,
        )
        .unwrap();

        let tip_error =
            tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(1000, "udsm")]).unwrap_err();

        assert_eq!(
            ContractError::InsufficientFee {
                denom: "udsm".to_string(),
                requested: Uint128::new(5000),
                provided: Uint128::new(1000),
            },
            tip_error
        );
    }

    #[test]
    fn tip_user_with_amount_eq_fees() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(5000, "udsm")],
            },
            5,
        )
        .unwrap();

        let tip_error =
            tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(5000, "udsm")]).unwrap_err();

        assert_eq!(ContractError::FoundAmountTooSmall {}, tip_error);
    }

    #[test]
    fn tip_user_with_empty_fixed_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, ServiceFee::Fixed { amount: vec![] }, 5).unwrap();

        tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(5000, "udsm")]).unwrap();
        let tips = get_tips_record_items(deps.as_mut());

        assert_eq!(
            vec![(
                (Addr::unchecked(USER_1), Addr::unchecked(USER_2), 0),
                vec![Coin::new(5000, "udsm")]
            )],
            tips
        );
    }

    #[test]
    fn tip_user_with_fixed_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            5,
        )
        .unwrap();

        tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(5000, "udsm")]).unwrap();
        let tips = get_tips_record_items(deps.as_mut());

        assert_eq!(
            vec![(
                (Addr::unchecked(USER_1), Addr::unchecked(USER_2), 0),
                vec![Coin::new(4000, "udsm")]
            )],
            tips
        );
    }

    #[test]
    fn tip_user_with_zero_percentage_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Percentage {
                value: Uint128::new(0),
                decimals: 2,
            },
            5,
        )
        .unwrap();

        tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(5000, "udsm")]).unwrap();
        let tips = get_tips_record_items(deps.as_mut());

        assert_eq!(
            vec![(
                (Addr::unchecked(USER_1), Addr::unchecked(USER_2), 0),
                vec![Coin::new(5000, "udsm")]
            )],
            tips
        );
    }

    #[test]
    fn tip_user_with_percentage_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Percentage {
                value: Uint128::new(100),
                decimals: 2,
            },
            5,
        )
        .unwrap();

        tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(5000, "udsm")]).unwrap();
        let tips = get_tips_record_items(deps.as_mut());

        assert_eq!(
            vec![(
                (Addr::unchecked(USER_1), Addr::unchecked(USER_2), 0),
                vec![Coin::new(4950, "udsm")]
            )],
            tips
        );
    }

    #[test]
    fn tip_post_with_invalid_post_id() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm")],
            },
            5,
        )
        .unwrap();

        let tip_error = tip_post(deps.as_mut(), USER_1, 0, &[Coin::new(5000, "udsm")]).unwrap_err();

        assert_eq!(ContractError::InvalidPostId {}, tip_error);
    }

    #[test]
    fn tip_post_with_non_existing_post_id() {
        let querier = MockQuerier::<DesmosQuery>::new(&[(MOCK_CONTRACT_ADDR, &[])])
            .with_custom_handler(|query| match query {
                DesmosQuery::Posts(post_query) => match post_query {
                    PostsQuery::Post { .. } => SystemResult::Err(SystemError::InvalidRequest {
                        error: "post not found".to_string(),
                        request: Default::default(),
                    }),
                    _ => SystemResult::Err(SystemError::Unknown {}),
                },
                DesmosQuery::Subspaces(subspaces_query) => {
                    SystemResult::Ok(mock_subspaces_query_response(subspaces_query))
                }
            });
        let mut deps = OwnedDeps {
            storage: MockStorage::default(),
            querier,
            api: MockApi::default(),
            custom_query_type: PhantomData,
        };

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm")],
            },
            5,
        )
        .unwrap();

        let tip_error = tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(5000, "udsm")]).unwrap_err();

        assert_eq!(
            ContractError::Std(StdError::generic_err(
                "Querier system error: Cannot parse request: post not found in: "
            )),
            tip_error
        );
    }

    #[test]
    fn tip_post_with_missing_fee_coin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm"), Coin::new(100, "uatom")],
            },
            5,
        )
        .unwrap();

        let tip_error = tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(5000, "udsm")]).unwrap_err();

        assert_eq!(
            ContractError::FeeCoinNotProvided {
                denom: "uatom".to_string()
            },
            tip_error
        );
    }

    #[test]
    fn tip_post_with_insufficient_fees() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(5000, "udsm")],
            },
            5,
        )
        .unwrap();

        let tip_error = tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(1000, "udsm")]).unwrap_err();

        assert_eq!(
            ContractError::InsufficientFee {
                denom: "udsm".to_string(),
                requested: Uint128::new(5000),
                provided: Uint128::new(1000),
            },
            tip_error
        );
    }

    #[test]
    fn tip_post_with_amount_eq_fees() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(5000, "udsm")],
            },
            5,
        )
        .unwrap();

        let tip_error = tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(5000, "udsm")]).unwrap_err();

        assert_eq!(ContractError::FoundAmountTooSmall {}, tip_error);
    }

    #[test]
    fn tip_post_with_empty_fixed_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, ServiceFee::Fixed { amount: vec![] }, 5).unwrap();

        tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(5000, "udsm")]).unwrap();
        let tips = get_tips_record_items(deps.as_mut());

        let post_author = MockPostsQueries::get_mocked_post(Uint64::new(1), Uint64::new(1));

        assert_eq!(
            vec![(
                (Addr::unchecked(USER_1), post_author.author, 1),
                vec![Coin::new(5000, "udsm")]
            )],
            tips
        );
    }

    #[test]
    fn tip_post_with_fixed_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            5,
        )
        .unwrap();

        tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(5000, "udsm")]).unwrap();
        let tips = get_tips_record_items(deps.as_mut());

        let post_author = MockPostsQueries::get_mocked_post(Uint64::new(1), Uint64::new(1));

        assert_eq!(
            vec![(
                (Addr::unchecked(USER_1), post_author.author, 1),
                vec![Coin::new(4000, "udsm")]
            )],
            tips
        );
    }

    #[test]
    fn tip_post_with_zero_percentage_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Percentage {
                value: Uint128::new(0),
                decimals: 2,
            },
            5,
        )
        .unwrap();

        tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(5000, "udsm")]).unwrap();
        let tips = get_tips_record_items(deps.as_mut());

        let post_author = MockPostsQueries::get_mocked_post(Uint64::new(1), Uint64::new(1));

        assert_eq!(
            vec![(
                (Addr::unchecked(USER_1), post_author.author, 1),
                vec![Coin::new(5000, "udsm")]
            )],
            tips
        );
    }

    #[test]
    fn tip_post_with_percentage_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Percentage {
                value: Uint128::new(100),
                decimals: 2,
            },
            5,
        )
        .unwrap();

        tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(5000, "udsm")]).unwrap();
        let tips = get_tips_record_items(deps.as_mut());

        let post_author = MockPostsQueries::get_mocked_post(Uint64::new(1), Uint64::new(1));

        assert_eq!(
            vec![(
                (Addr::unchecked(USER_1), post_author.author, 1),
                vec![Coin::new(4950, "udsm")]
            )],
            tips
        );
    }

    #[test]
    fn tip_without_tips_record_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            0,
        )
        .unwrap();

        tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(5000, "udsm")]).unwrap();

        assert!(get_tips_record_items(deps.as_mut()).is_empty())
    }

    #[test]
    fn update_service_fees_with_invalid_admin_address() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, ServiceFee::Fixed { amount: vec![] }, 5).unwrap();

        let update_error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER_1, &[]),
            ExecuteMsg::UpdateServiceFee {
                new_fee: ServiceFee::Fixed { amount: vec![] },
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::Unauthorized {}, update_error);
    }

    #[test]
    fn update_service_fee_with_invalid_percentage() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, ServiceFee::Fixed { amount: vec![] }, 5).unwrap();

        let update_error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateServiceFee {
                new_fee: ServiceFee::Percentage {
                    value: Uint128::new(100),
                    decimals: 0,
                },
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::InvalidPercentageFee {}, update_error);
    }

    #[test]
    fn update_service_fee_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Percentage {
                value: Uint128::new(100),
                decimals: 2,
            },
            5,
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateServiceFee {
                new_fee: ServiceFee::Fixed { amount: vec![] },
            },
        )
        .unwrap();

        let config = CONFIG.load(deps.as_mut().storage).unwrap();
        assert_eq!(
            StateServiceFee::Fixed { amount: vec![] },
            config.service_fee
        );
    }

    #[test]
    fn update_admin_from_non_admin_user() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, ServiceFee::Fixed { amount: vec![] }, 0).unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER_1, &[]),
            ExecuteMsg::UpdateAdmin {
                new_admin: USER_1.to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::Unauthorized {}, error);
    }

    #[test]
    fn update_admin_with_invalid_admin_address() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, ServiceFee::Fixed { amount: vec![] }, 0).unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateAdmin {
                new_admin: "a".to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(
            ContractError::Std(StdError::generic_err(
                "Invalid input: human address too short"
            )),
            error
        );
    }

    #[test]
    fn update_admin_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, ServiceFee::Fixed { amount: vec![] }, 0).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateAdmin {
                new_admin: USER_1.to_string(),
            },
        )
        .unwrap();

        let config = CONFIG.load(deps.as_mut().storage).unwrap();
        assert_eq!(config.admin.as_str(), USER_1);
    }

    #[test]
    fn update_tips_record_threshold_from_non_admin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            10,
        )
        .unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER_1, &[]),
            ExecuteMsg::UpdateSavedTipsRecordThreshold { new_threshold: 3 },
        )
        .unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, error);
    }

    #[test]
    fn update_tips_record_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            3,
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateSavedTipsRecordThreshold { new_threshold: 10 },
        )
        .unwrap();
    }

    #[test]
    fn update_tips_record_threshold_shrink_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            10,
        )
        .unwrap();

        tip_user(deps.as_mut(), USER_3, USER_1, &[Coin::new(2000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(2000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_1, USER_3, &[Coin::new(2000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_3, USER_2, &[Coin::new(2000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_2, USER_1, &[Coin::new(2000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_2, USER_3, &[Coin::new(2000, "udsm")]).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateSavedTipsRecordThreshold { new_threshold: 3 },
        )
        .unwrap();

        let tips = get_tips_record_items(deps.as_mut());
        assert_eq!(
            vec![
                (
                    (Addr::unchecked(USER_2), Addr::unchecked(USER_1), 0),
                    vec![Coin::new(1000, "udsm")]
                ),
                (
                    (Addr::unchecked(USER_2), Addr::unchecked(USER_3), 0),
                    vec![Coin::new(1000, "udsm")]
                ),
                (
                    (Addr::unchecked(USER_3), Addr::unchecked(USER_2), 0),
                    vec![Coin::new(1000, "udsm")]
                ),
            ],
            tips
        );

        let keys = TIPS_KEY_LIST.load(deps.as_mut().storage).unwrap();
        assert_eq!(
            VecDeque::from([
                (Addr::unchecked(USER_3), Addr::unchecked(USER_2), 0),
                (Addr::unchecked(USER_2), Addr::unchecked(USER_1), 0),
                (Addr::unchecked(USER_2), Addr::unchecked(USER_3), 0),
            ]),
            keys
        )
    }

    #[test]
    fn update_tips_record_threshold_records_wipe_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            10,
        )
        .unwrap();

        tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(2000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_3, USER_1, &[Coin::new(2000, "udsm")]).unwrap();

        let tips = get_tips_record_items(deps.as_mut());
        assert_eq!(
            vec![
                (
                    (Addr::unchecked(USER_1), Addr::unchecked(USER_2), 0),
                    vec![Coin::new(1000, "udsm")]
                ),
                (
                    (Addr::unchecked(USER_3), Addr::unchecked(USER_1), 0),
                    vec![Coin::new(1000, "udsm")]
                ),
            ],
            tips
        );

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateSavedTipsRecordThreshold { new_threshold: 0 },
        )
        .unwrap();

        let tips = get_tips_record_items(deps.as_mut());
        assert!(tips.is_empty());

        let keys = TIPS_KEY_LIST.load(deps.as_mut().storage).unwrap();
        assert!(keys.is_empty());
    }

    #[test]
    fn claim_fee_from_non_admin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            10,
        )
        .unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER_1, &[]),
            ExecuteMsg::ClaimFees {
                receiver: USER_1.to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::Unauthorized {}, error);
    }

    #[test]
    fn claim_fee_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[Coin::new(2000, "udsm")]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            10,
        )
        .unwrap();

        let response = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::ClaimFees {
                receiver: USER_1.to_string(),
            },
        )
        .unwrap();

        assert_eq!(
            vec![SubMsg::new(BankMsg::Send {
                amount: vec![Coin::new(2000, "udsm")],
                to_address: USER_1.to_string()
            })],
            response.messages
        );
    }

    #[test]
    fn query_user_received_tips_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            5,
        )
        .unwrap();

        tip_user(deps.as_mut(), USER_1, USER_3, &[Coin::new(5000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_2, USER_3, &[Coin::new(2000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_2, USER_3, &[Coin::new(2000, "udsm")]).unwrap();
        // Some more data not related to user 3
        tip_user(deps.as_mut(), USER_2, USER_1, &[Coin::new(100000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(100000, "udsm")]).unwrap();

        let response = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::UserReceivedTips {
                user: USER_3.to_string(),
            },
        )
        .unwrap();
        let tips: TipsResponse = from_binary(&response).unwrap();
        // Must be 2 since the two tips made from USER_2 should be merged into one
        assert_eq!(2, tips.tips.len());
        assert_eq!(
            tips,
            TipsResponse {
                tips: vec![
                    Tip {
                        sender: Addr::unchecked(USER_1),
                        receiver: Addr::unchecked(USER_3),
                        amount: vec![Coin::new(4000, "udsm")]
                    },
                    Tip {
                        sender: Addr::unchecked(USER_2),
                        receiver: Addr::unchecked(USER_3),
                        amount: vec![Coin::new(2000, "udsm")]
                    },
                ]
            }
        )
    }

    #[test]
    fn query_user_sent_tips_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            5,
        )
        .unwrap();

        tip_user(deps.as_mut(), USER_1, USER_3, &[Coin::new(5000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_2, USER_3, &[Coin::new(2000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_2, USER_3, &[Coin::new(2000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_2, USER_1, &[Coin::new(100000, "udsm")]).unwrap();
        tip_user(deps.as_mut(), USER_1, USER_2, &[Coin::new(100000, "udsm")]).unwrap();

        let response = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::UserSentTips {
                user: USER_1.to_string(),
            },
        )
        .unwrap();
        let tips: TipsResponse = from_binary(&response).unwrap();
        assert_eq!(2, tips.tips.len());
        assert_eq!(
            tips,
            TipsResponse {
                tips: vec![
                    Tip {
                        sender: Addr::unchecked(USER_1),
                        receiver: Addr::unchecked(USER_2),
                        amount: vec![Coin::new(99000, "udsm")]
                    },
                    Tip {
                        sender: Addr::unchecked(USER_1),
                        receiver: Addr::unchecked(USER_3),
                        amount: vec![Coin::new(4000, "udsm")]
                    },
                ]
            }
        )
    }

    #[test]
    fn query_post_received_tips_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            },
            5,
        )
        .unwrap();

        tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(5000, "udsm")]).unwrap();
        tip_post(deps.as_mut(), USER_2, 1, &[Coin::new(2000, "udsm")]).unwrap();
        tip_post(deps.as_mut(), USER_3, 1, &[Coin::new(100000, "udsm")]).unwrap();
        tip_post(deps.as_mut(), USER_1, 1, &[Coin::new(100000, "udsm")]).unwrap();

        let response = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::PostReceivedTips {
                post_id: Uint64::new(1),
            },
        )
        .unwrap();
        let tips: TipsResponse = from_binary(&response).unwrap();
        assert_eq!(3, tips.tips.len());
        assert_eq!(
            tips,
            TipsResponse {
                tips: vec![
                    Tip {
                        sender: Addr::unchecked(USER_1),
                        receiver: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                        amount: vec![Coin::new(4000 + 99000, "udsm")]
                    },
                    Tip {
                        sender: Addr::unchecked(USER_2),
                        receiver: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                        amount: vec![Coin::new(1000, "udsm")]
                    },
                    Tip {
                        sender: Addr::unchecked(USER_3),
                        receiver: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
                        amount: vec![Coin::new(99000, "udsm")]
                    },
                ]
            }
        )
    }
}
