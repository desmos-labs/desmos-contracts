use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, ServiceFee, Target, Tip,
    TipsResponse,
};
use crate::state::{
    Config, StateServiceFee, CONFIG, POST_TIPS_HISTORY, RECEIVED_TIPS_HISTORY, SENT_TIPS_HISTORY,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Storage,
};
use cw2::set_contract_version;
use cw_storage_plus::{KeyDeserialize, Map, PrimaryKey};
use desmos_bindings::posts::querier::PostsQuerier;
use desmos_bindings::subspaces::querier::SubspacesQuerier;
use desmos_bindings::{msg::DesmosMsg, query::DesmosQuery};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::convert::TryFrom;
use std::ops::Deref;

// Contract constants
pub const MAX_TIPS_HISTORY_SIZE: u32 = 30;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:tips";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// actions for executing messages
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_SEND_TIP: &str = "send_tip";
const ACTION_UPDATE_SERVICE_FEE: &str = "update_service_fee";
const ACTION_UPDATE_ADMIN: &str = "update_admin";
const ACTION_UPDATE_SAVED_TIPS_RECORD_SIZE: &str = "update_saved_tips_record_size";
const ACTION_CLAIM_FEES: &str = "claim_fees";

// attributes for executing messages
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_SENDER: &str = "sender";
const ATTRIBUTE_ADMIN: &str = "admin";
const ATTRIBUTE_SUBSPACE_ID: &str = "subspace_id";
const ATTRIBUTE_TIPS_RECORD_SIZE: &str = "tips_record_size";
const ATTRIBUTE_NEW_ADMIN: &str = "new_admin";
const ATTRIBUTE_NEW_SIZE: &str = "new_size";
const ATTRIBUTE_RECEIVER: &str = "receiver";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    msg.validate()?;
    let admin = deps.api.addr_validate(&msg.admin)?;

    // Ensure that the subspace exists
    SubspacesQuerier::new(deps.querier.deref())
        .query_subspace(msg.subspace_id.u64())
        .map_err(|error| ContractError::SubspaceNotExist {
            id: msg.subspace_id.u64(),
            error,
        })?;

    let service_fee = if let Some(service_fee) = msg.service_fee {
        Some(StateServiceFee::try_from(service_fee)?)
    } else {
        None
    };

    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(
        deps.storage,
        &Config {
            admin,
            subspace_id: msg.subspace_id.u64(),
            service_fee,
            saved_tips_record_size: msg.saved_tips_record_size,
        },
    )?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE)
        .add_attribute(ATTRIBUTE_SENDER, info.sender)
        .add_attribute(ATTRIBUTE_ADMIN, msg.admin)
        .add_attribute(ATTRIBUTE_SUBSPACE_ID, msg.subspace_id)
        .add_attribute(
            ATTRIBUTE_TIPS_RECORD_SIZE,
            msg.saved_tips_record_size.to_string(),
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
        ExecuteMsg::SendTip { target, amount } => execute_send_tip(deps, env, info, target, amount),
        ExecuteMsg::UpdateServiceFee { new_fee } => execute_update_service_fee(deps, info, new_fee),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
        ExecuteMsg::UpdateSavedTipsRecordSize { new_size } => {
            execute_update_saved_tips_record_size(deps, info, new_size)
        }
        ExecuteMsg::ClaimFees { receiver } => execute_claim_fees(deps, env, info, receiver),
    }
}

fn execute_send_tip(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    info: MessageInfo,
    target: Target,
    tip_amount: Vec<Coin>,
) -> Result<Response<DesmosMsg>, ContractError> {
    if info.funds.is_empty() {
        return Err(ContractError::EmptyFunds {});
    }

    let config = CONFIG.load(deps.storage)?;

    if let Some(service_fee) = &config.service_fee {
        service_fee.check_fees(&info.funds, &tip_amount)?;
    }

    // Resolve the receiver and the optional post id
    let (post_id, receiver) = match target {
        Target::ContentTarget { post_id } => {
            let post = PostsQuerier::new(deps.querier.deref())
                .query_post(config.subspace_id, post_id.u64())
                .map_err(|_| ContractError::PostNotFound { id: post_id.u64() })?
                .post;
            (post.id.u64(), post.author)
        }
        Target::UserTarget { receiver } => {
            // Set the post id to 0 (invalid id) to signal that this tip is referencing an user
            (0_u64, deps.api.addr_validate(&receiver)?)
        }
    };

    if info.sender == receiver {
        return Err(ContractError::SenderEqReceiver {});
    }

    if config.saved_tips_record_size > 0 {
        SENT_TIPS_HISTORY.update::<_, ContractError>(
            deps.storage,
            info.sender.clone(),
            |history| {
                let mut history = history.unwrap_or_default();
                history.push_back((receiver.clone(), tip_amount.clone(), post_id));

                while history.len() > config.saved_tips_record_size as usize {
                    history.pop_front();
                }
                Ok(history)
            },
        )?;

        RECEIVED_TIPS_HISTORY.update::<_, ContractError>(
            deps.storage,
            receiver.clone(),
            |history| {
                let mut history = history.unwrap_or_default();
                history.push_back((info.sender.clone(), tip_amount.clone(), post_id));

                while history.len() > config.saved_tips_record_size as usize {
                    history.pop_front();
                }
                Ok(history)
            },
        )?;

        if post_id > 0 {
            POST_TIPS_HISTORY.update::<_, ContractError>(deps.storage, post_id, |history| {
                let mut history = history.unwrap_or_default();
                history.push_back((info.sender.clone(), tip_amount.clone()));

                while history.len() > config.saved_tips_record_size as usize {
                    history.pop_front();
                }
                Ok(history)
            })?;
        }
    }

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_SEND_TIP)
        .add_attribute(ATTRIBUTE_SENDER, info.sender)
        .add_attribute(ATTRIBUTE_RECEIVER, receiver.as_str())
        .add_message(BankMsg::Send {
            to_address: receiver.to_string(),
            amount: tip_amount,
        }))
}

fn execute_update_service_fee(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    service_fee: Option<ServiceFee>,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    let new_service_fee = if let Some(service_fee) = service_fee {
        Some(StateServiceFee::try_from(service_fee)?)
    } else {
        None
    };
    config.service_fee = new_service_fee;
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

fn execute_update_saved_tips_record_size(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    new_size: u32,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    if config.admin != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // Wipe the tips history otherwise leave to SendTip to shrink the tips record
    if new_size == 0 {
        wipe_map(&SENT_TIPS_HISTORY, deps.storage)?;
        wipe_map(&RECEIVED_TIPS_HISTORY, deps.storage)?;
        wipe_map(&POST_TIPS_HISTORY, deps.storage)?;
    }

    config.saved_tips_record_size = new_size;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_SAVED_TIPS_RECORD_SIZE)
        .add_attribute(ATTRIBUTE_SENDER, info.sender)
        .add_attribute(ATTRIBUTE_NEW_SIZE, new_size.to_string()))
}

fn wipe_map<'a, K, V>(map: &Map<'a, K, V>, storage: &mut dyn Storage) -> StdResult<()>
where
    K: PrimaryKey<'a> + KeyDeserialize + KeyDeserialize<Output = K> + 'static,
    V: Serialize + DeserializeOwned,
{
    let mut wiped = false;
    while !wiped {
        // Read the data paginated since all the tips_record may not fit inside the VM heap.
        let mut keys: Vec<K::Output> = map
            .keys(storage, None, None, Order::Ascending)
            .take(20)
            .collect::<StdResult<Vec<_>>>()?;

        wiped = keys.len() < 20;

        keys.drain(0..).for_each(|key| map.remove(storage, key));
    }
    Ok(())
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
        QueryMsg::UserReceivedTips { user } => {
            to_binary(&query_received_tips(deps, deps.api.addr_validate(&user)?)?)
        }
        QueryMsg::UserSentTips { user } => {
            to_binary(&query_sent_tips(deps, deps.api.addr_validate(&user)?)?)
        }
        QueryMsg::PostReceivedTips { post_id } => to_binary(&query_post_tips(deps, post_id.u64())?),
    }
    .map_err(ContractError::from)
}

pub fn query_config(deps: Deps<DesmosQuery>) -> StdResult<QueryConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(QueryConfigResponse {
        admin: config.admin,
        subspace_id: config.subspace_id.into(),
        service_fee: config.service_fee.map(StateServiceFee::into),
        saved_tips_record_size: config.saved_tips_record_size,
    })
}

fn query_sent_tips(deps: Deps<DesmosQuery>, sender: Addr) -> Result<TipsResponse, ContractError> {
    let tips = SENT_TIPS_HISTORY
        .may_load(deps.storage, sender.clone())?
        .unwrap_or_default()
        .drain(0..)
        .map(|tip| Tip::from_sent_tip(sender.clone(), tip))
        .collect();
    Ok(TipsResponse { tips })
}

fn query_received_tips(
    deps: Deps<DesmosQuery>,
    receiver: Addr,
) -> Result<TipsResponse, ContractError> {
    let tips = RECEIVED_TIPS_HISTORY
        .may_load(deps.storage, receiver.clone())?
        .unwrap_or_default()
        .drain(0..)
        .map(|tip| Tip::from_received_tip(receiver.clone(), tip))
        .collect();
    Ok(TipsResponse { tips })
}

fn query_post_tips(deps: Deps<DesmosQuery>, post_id: u64) -> Result<TipsResponse, ContractError> {
    if post_id == 0 {
        return Err(ContractError::InvalidPostId {});
    }
    // Get the subspace id where is deployed the smart contract
    let subspace_id = CONFIG.load(deps.storage)?.subspace_id;
    // Get the post author
    let post_author = PostsQuerier::new(deps.querier.deref())
        .query_post(subspace_id, post_id)
        .map_err(|_| ContractError::PostNotFound { id: post_id })?
        .post
        .author;

    let tips = POST_TIPS_HISTORY
        .may_load(deps.storage, post_id)?
        .unwrap_or_default()
        .drain(0..)
        .map(|tip| Tip::from_post_tip(post_id, post_author.clone(), tip))
        .collect();

    Ok(TipsResponse { tips })
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query, MAX_TIPS_HISTORY_SIZE};
    use crate::error::ContractError;
    use crate::msg::{
        ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, ServiceFee, Target, Tip,
        TipsResponse,
    };
    use crate::state::{
        StateServiceFee, CONFIG, POST_TIPS_HISTORY, RECEIVED_TIPS_HISTORY, SENT_TIPS_HISTORY,
    };
    use cosmwasm_std::testing::{
        mock_env, mock_info, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR,
    };
    use cosmwasm_std::{
        from_binary, Addr, BankMsg, Coin, Decimal, DepsMut, OwnedDeps, Response, StdError, SubMsg,
        SystemError, SystemResult, Uint128, Uint64,
    };
    use desmos_bindings::mocks::mock_queriers::mock_dependencies_with_custom_querier;
    use desmos_bindings::msg::DesmosMsg;
    use desmos_bindings::posts::query::PostsQuery;
    use desmos_bindings::query::DesmosQuery;
    use desmos_bindings::subspaces::mocks::mock_subspaces_query_response;
    use desmos_bindings::subspaces::query::SubspacesQuery;
    use std::marker::PhantomData;

    const ADMIN: &str = "admin";
    const USER_1: &str = "user1";
    const USER_2: &str = "user2";
    const USER_3: &str = "user3";
    const POST_AUTHOR: &str = "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc";

    fn init_contract(
        deps: DepsMut<DesmosQuery>,
        subspace_id: u64,
        service_fee: Option<ServiceFee>,
        saved_tips_record_size: u32,
    ) -> Result<Response<DesmosMsg>, ContractError> {
        instantiate(
            deps,
            mock_env(),
            mock_info(ADMIN, &[]),
            InstantiateMsg {
                admin: ADMIN.to_string(),
                subspace_id: subspace_id.into(),
                service_fee,
                saved_tips_record_size,
            },
        )
    }

    fn tip_user(
        deps: DepsMut<DesmosQuery>,
        from: &str,
        to: &str,
        funds: &[Coin],
        coins: &[Coin],
    ) -> Result<Response<DesmosMsg>, ContractError> {
        let info = mock_info(from, funds);
        execute(
            deps,
            mock_env(),
            info,
            ExecuteMsg::SendTip {
                target: Target::UserTarget {
                    receiver: to.to_string(),
                },
                amount: coins.to_vec(),
            },
        )
    }

    fn tip_post(
        deps: DepsMut<DesmosQuery>,
        from: &str,
        post_id: u64,
        funds: &[Coin],
        coins: &[Coin],
    ) -> Result<Response<DesmosMsg>, ContractError> {
        let info = mock_info(from, funds);
        execute(
            deps,
            mock_env(),
            info,
            ExecuteMsg::SendTip {
                target: Target::ContentTarget {
                    post_id: post_id.into(),
                },
                amount: coins.to_vec(),
            },
        )
    }

    fn get_user_sent_tips(deps: DepsMut<DesmosQuery>, addr: &str) -> Vec<Tip> {
        SENT_TIPS_HISTORY
            .may_load(deps.storage, Addr::unchecked(addr))
            .unwrap()
            .unwrap_or_default()
            .drain(0..)
            .map(|tip| Tip::from_sent_tip(Addr::unchecked(addr), tip))
            .collect()
    }

    fn get_user_received_tips(deps: DepsMut<DesmosQuery>, addr: &str) -> Vec<Tip> {
        RECEIVED_TIPS_HISTORY
            .may_load(deps.storage, Addr::unchecked(addr))
            .unwrap()
            .unwrap_or_default()
            .drain(0..)
            .map(|tip| Tip::from_received_tip(Addr::unchecked(addr), tip))
            .collect()
    }

    fn get_post_tips(deps: DepsMut<DesmosQuery>, post_id: u64, post_author: &str) -> Vec<Tip> {
        POST_TIPS_HISTORY
            .may_load(deps.storage, post_id)
            .unwrap()
            .unwrap_or_default()
            .drain(0..)
            .map(|tip| Tip::from_post_tip(post_id, Addr::unchecked(post_author), tip))
            .collect()
    }

    #[test]
    fn init_contract_with_invalid_subspace_id() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        let init_err = init_contract(
            deps.as_mut(),
            0,
            Some(ServiceFee::Fixed { amount: vec![] }),
            1,
        )
        .unwrap_err();
        assert_eq!(ContractError::InvalidSubspaceId {}, init_err);
    }

    #[test]
    fn init_contract_with_invalid_tips_history_size() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        let init_err =
            init_contract(deps.as_mut(), 1, None, MAX_TIPS_HISTORY_SIZE + 1).unwrap_err();

        assert_eq!(
            ContractError::InvalidTipsHistorySize {
                value: MAX_TIPS_HISTORY_SIZE + 1,
                max: MAX_TIPS_HISTORY_SIZE
            },
            init_err
        );
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

        let init_err = init_contract(
            deps.as_mut(),
            2,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(42, "dsm")],
            }),
            1,
        )
        .unwrap_err();

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
    fn init_contract_with_empty_fixed_service_fees() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        // Simulate init with 100% of service fees
        let init_err = init_contract(
            deps.as_mut(),
            2,
            Some(ServiceFee::Fixed { amount: vec![] }),
            1,
        )
        .unwrap_err();

        assert_eq!(ContractError::EmptyFixedFee {}, init_err);
    }

    #[test]
    fn init_contract_with_invalid_zero_fee_coin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        // Simulate init with 100% of service fees
        let init_err = init_contract(
            deps.as_mut(),
            2,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(0, "udsm")],
            }),
            1,
        )
        .unwrap_err();

        assert_eq!(
            ContractError::ZeroFeeCoin {
                denom: "udsm".to_string()
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
            Some(ServiceFee::Percentage {
                value: Decimal::from_atomics(100u32, 0).unwrap(),
            }),
            1,
        )
        .unwrap_err();

        assert_eq!(ContractError::InvalidPercentageFee {}, init_err);
    }

    #[test]
    fn init_contract_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(42, "udsm")],
            }),
            1,
        )
        .unwrap();
    }

    #[test]
    fn init_contract_without_service_fee_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, None, 1).unwrap();
    }

    #[test]
    fn tip_user_with_invalid_address() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, None, 5).unwrap();

        let tip_error = tip_user(
            deps.as_mut(),
            USER_1,
            "a",
            &[Coin::new(5000, "udsm")],
            &[Coin::new(5000, "udsm")],
        )
        .unwrap_err();

        assert_eq!(
            ContractError::Std(StdError::generic_err(
                "Invalid input: human address too short"
            )),
            tip_error
        );
    }

    #[test]
    fn tip_with_empty_funds() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, None, 5).unwrap();

        let tip_error = tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[],
            &[Coin::new(5000, "udsm")],
        )
        .unwrap_err();

        assert_eq!(ContractError::EmptyFunds {}, tip_error);
    }

    #[test]
    fn tip_to_yourself() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm")],
            }),
            5,
        )
        .unwrap();

        let tip_error = tip_user(
            deps.as_mut(),
            USER_1,
            USER_1,
            &[Coin::new(1100, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap_err();

        assert_eq!(ContractError::SenderEqReceiver {}, tip_error);
    }

    #[test]
    fn tip_user_with_missing_fee_coin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm"), Coin::new(100, "uatom")],
            }),
            5,
        )
        .unwrap();

        let tip_error = tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(5100, "udsm")],
            &[Coin::new(5000, "udsm")],
        )
        .unwrap_err();

        assert_eq!(
            ContractError::InsufficientAmount {
                denom: "uatom".to_string(),
                requested: Uint128::new(100),
                provided: Uint128::zero(),
            },
            tip_error
        );
    }

    #[test]
    fn tip_user_with_insufficient_amount() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(5000, "udsm")],
            }),
            5,
        )
        .unwrap();

        let tip_error = tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(5999, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap_err();

        assert_eq!(
            ContractError::InsufficientAmount {
                denom: "udsm".to_string(),
                requested: Uint128::new(6000),
                provided: Uint128::new(5999),
            },
            tip_error
        );
    }

    #[test]
    fn tip_user_with_zero_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, None, 5).unwrap();

        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(5000, "udsm")],
        )
        .unwrap();

        let sent_tip = Tip {
            sender: Addr::unchecked(USER_1),
            receiver: Addr::unchecked(USER_2),
            amount: vec![Coin::new(5000, "udsm")],
            post_id: None,
        };

        assert_eq!(
            vec![sent_tip.clone()],
            get_user_sent_tips(deps.as_mut(), USER_1)
        );

        assert_eq!(
            vec![sent_tip],
            get_user_received_tips(deps.as_mut(), USER_2)
        );
    }

    #[test]
    fn tip_user_with_fixed_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            5,
        )
        .unwrap();

        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4000, "udsm")],
        )
        .unwrap();

        let sent_tip = Tip {
            sender: Addr::unchecked(USER_1),
            receiver: Addr::unchecked(USER_2),
            amount: vec![Coin::new(4000, "udsm")],
            post_id: None,
        };

        assert_eq!(
            vec![sent_tip.clone()],
            get_user_sent_tips(deps.as_mut(), USER_1)
        );

        assert_eq!(
            vec![sent_tip],
            get_user_received_tips(deps.as_mut(), USER_2)
        );
    }

    #[test]
    fn tip_user_with_percentage_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Percentage {
                value: Decimal::one(),
            }),
            5,
        )
        .unwrap();

        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4950, "udsm")],
        )
        .unwrap();

        let sent_tip = Tip {
            sender: Addr::unchecked(USER_1),
            receiver: Addr::unchecked(USER_2),
            amount: vec![Coin::new(4950, "udsm")],
            post_id: None,
        };

        assert_eq!(
            vec![sent_tip.clone()],
            get_user_sent_tips(deps.as_mut(), USER_1)
        );

        assert_eq!(
            vec![sent_tip],
            get_user_received_tips(deps.as_mut(), USER_2)
        );
    }

    #[test]
    fn tip_post_with_invalid_post_id() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm")],
            }),
            5,
        )
        .unwrap();

        let tip_error = tip_post(
            deps.as_mut(),
            USER_1,
            0,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4900, "udsm")],
        )
        .unwrap_err();

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
                _ => SystemResult::Err(SystemError::Unknown {}),
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
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm")],
            }),
            5,
        )
        .unwrap();

        let tip_error = tip_post(
            deps.as_mut(),
            USER_1,
            1,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4900, "udsm")],
        )
        .unwrap_err();

        assert_eq!(ContractError::PostNotFound { id: 1 }, tip_error);
    }

    #[test]
    fn tip_post_with_missing_fee_coin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm"), Coin::new(100, "uatom")],
            }),
            5,
        )
        .unwrap();

        let tip_error = tip_post(
            deps.as_mut(),
            USER_1,
            1,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4900, "udsm")],
        )
        .unwrap_err();

        assert_eq!(
            ContractError::InsufficientAmount {
                denom: "uatom".to_string(),
                requested: Uint128::new(100),
                provided: Uint128::zero(),
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
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(5000, "udsm")],
            }),
            5,
        )
        .unwrap();

        let tip_error = tip_post(
            deps.as_mut(),
            USER_1,
            1,
            &[Coin::new(1000, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap_err();

        assert_eq!(
            ContractError::InsufficientAmount {
                denom: "udsm".to_string(),
                requested: Uint128::new(6000),
                provided: Uint128::new(1000),
            },
            tip_error
        );
    }

    #[test]
    fn tip_post_with_zero_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(deps.as_mut(), 1, None, 5).unwrap();

        tip_post(
            deps.as_mut(),
            USER_1,
            1,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(5000, "udsm")],
        )
        .unwrap();

        let sent_tips = vec![Tip {
            sender: Addr::unchecked(USER_1),
            receiver: Addr::unchecked(POST_AUTHOR),
            amount: vec![Coin::new(5000, "udsm")],
            post_id: Some(Uint64::new(1)),
        }];

        assert_eq!(sent_tips, get_user_sent_tips(deps.as_mut(), USER_1));
        assert_eq!(
            sent_tips,
            get_user_received_tips(deps.as_mut(), POST_AUTHOR)
        );
        assert_eq!(sent_tips, get_post_tips(deps.as_mut(), 1, POST_AUTHOR));
    }

    #[test]
    fn tip_post_with_fixed_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            5,
        )
        .unwrap();

        tip_post(
            deps.as_mut(),
            USER_1,
            1,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4000, "udsm")],
        )
        .unwrap();
        let sent_tips = vec![Tip {
            sender: Addr::unchecked(USER_1),
            receiver: Addr::unchecked(POST_AUTHOR),
            amount: vec![Coin::new(4000, "udsm")],
            post_id: Some(Uint64::new(1)),
        }];

        assert_eq!(sent_tips, get_user_sent_tips(deps.as_mut(), USER_1));
        assert_eq!(
            sent_tips,
            get_user_received_tips(deps.as_mut(), POST_AUTHOR)
        );
        assert_eq!(sent_tips, get_post_tips(deps.as_mut(), 1, POST_AUTHOR));
    }

    #[test]
    fn tip_post_with_percentage_fees_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Percentage {
                value: Decimal::one(),
            }),
            5,
        )
        .unwrap();

        tip_post(
            deps.as_mut(),
            USER_1,
            1,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4950, "udsm")],
        )
        .unwrap();
        let sent_tips = vec![Tip {
            sender: Addr::unchecked(USER_1),
            receiver: Addr::unchecked(POST_AUTHOR),
            amount: vec![Coin::new(4950, "udsm")],
            post_id: Some(Uint64::new(1)),
        }];

        assert_eq!(sent_tips, get_user_sent_tips(deps.as_mut(), USER_1));
        assert_eq!(
            sent_tips,
            get_user_received_tips(deps.as_mut(), POST_AUTHOR)
        );
        assert_eq!(sent_tips, get_post_tips(deps.as_mut(), 1, POST_AUTHOR));
    }

    #[test]
    fn tip_reach_tips_record_size() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(100, "udsm")],
            }),
            2,
        )
        .unwrap();

        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(3100, "udsm")],
            &[Coin::new(3000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(4100, "udsm")],
            &[Coin::new(4000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(5100, "udsm")],
            &[Coin::new(5000, "udsm")],
        )
        .unwrap();

        let sent_tips = vec![
            Tip {
                sender: Addr::unchecked(USER_1),
                receiver: Addr::unchecked(USER_2),
                amount: vec![Coin::new(4000, "udsm")],
                post_id: None,
            },
            Tip {
                sender: Addr::unchecked(USER_1),
                receiver: Addr::unchecked(USER_2),
                amount: vec![Coin::new(5000, "udsm")],
                post_id: None,
            },
        ];

        assert_eq!(sent_tips, get_user_sent_tips(deps.as_mut(), USER_1));
        assert_eq!(sent_tips, get_user_received_tips(deps.as_mut(), USER_2));
    }

    #[test]
    fn tip_without_tips_record_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            0,
        )
        .unwrap();

        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4000, "udsm")],
        )
        .unwrap();

        assert!(get_user_sent_tips(deps.as_mut(), USER_1).is_empty());
        assert!(get_user_received_tips(deps.as_mut(), USER_2).is_empty());
    }

    #[test]
    fn update_service_fees_with_invalid_admin_address() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(42, "udsm")],
            }),
            5,
        )
        .unwrap();

        let update_error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER_1, &[]),
            ExecuteMsg::UpdateServiceFee {
                new_fee: Some(ServiceFee::Fixed {
                    amount: vec![Coin::new(42, "udsm")],
                }),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::Unauthorized {}, update_error);
    }

    #[test]
    fn update_service_fee_with_empty_fixed_fee() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(42, "udsm")],
            }),
            5,
        )
        .unwrap();

        let update_error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateServiceFee {
                new_fee: Some(ServiceFee::Fixed { amount: vec![] }),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::EmptyFixedFee {}, update_error);
    }

    #[test]
    fn update_service_fee_with_zero_fee_coin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(42, "udsm")],
            }),
            5,
        )
        .unwrap();

        let update_error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateServiceFee {
                new_fee: Some(ServiceFee::Fixed {
                    amount: vec![Coin::new(42, "uatom"), Coin::new(0, "udsm")],
                }),
            },
        )
        .unwrap_err();

        assert_eq!(
            ContractError::ZeroFeeCoin {
                denom: "udsm".to_string()
            },
            update_error
        );
    }

    #[test]
    fn update_service_fee_with_invalid_percentage() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(42, "udsm")],
            }),
            5,
        )
        .unwrap();

        let update_error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateServiceFee {
                new_fee: Some(ServiceFee::Percentage {
                    value: Decimal::from_atomics(100u32, 0).unwrap(),
                }),
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
            Some(ServiceFee::Percentage {
                value: Decimal::one(),
            }),
            5,
        )
        .unwrap();

        let fee_coins = vec![Coin::new(42, "udsm")];

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateServiceFee {
                new_fee: Some(ServiceFee::Fixed {
                    amount: fee_coins.clone(),
                }),
            },
        )
        .unwrap();

        let config = CONFIG.load(deps.as_mut().storage).unwrap();
        assert_eq!(
            Some(StateServiceFee::Fixed {
                amount: fee_coins.clone()
            }),
            config.service_fee
        );
    }

    #[test]
    fn clear_service_fee_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Percentage {
                value: Decimal::one(),
            }),
            5,
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateServiceFee { new_fee: None },
        )
        .unwrap();

        let config = CONFIG.load(deps.as_mut().storage).unwrap();
        assert_eq!(None, config.service_fee);
    }

    #[test]
    fn update_admin_from_non_admin_user() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(42, "udsm")],
            }),
            0,
        )
        .unwrap();

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

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(42, "udsm")],
            }),
            0,
        )
        .unwrap();

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

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(42, "udsm")],
            }),
            0,
        )
        .unwrap();

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
    fn update_tips_record_size_from_non_admin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            10,
        )
        .unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(USER_1, &[]),
            ExecuteMsg::UpdateSavedTipsRecordSize { new_size: 3 },
        )
        .unwrap_err();
        assert_eq!(ContractError::Unauthorized {}, error);
    }

    #[test]
    fn update_tips_record_with_invalid_size() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            3,
        )
        .unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateSavedTipsRecordSize {
                new_size: MAX_TIPS_HISTORY_SIZE + 1,
            },
        )
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidTipsHistorySize {
                value: MAX_TIPS_HISTORY_SIZE + 1,
                max: MAX_TIPS_HISTORY_SIZE
            },
            error
        );
    }

    #[test]
    fn update_tips_record_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            3,
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateSavedTipsRecordSize { new_size: 10 },
        )
        .unwrap();
    }

    #[test]
    fn update_tips_record_size_records_wipe_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            10,
        )
        .unwrap();

        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(2000, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_3,
            USER_1,
            &[Coin::new(2000, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap();
        tip_post(
            deps.as_mut(),
            USER_3,
            1,
            &[Coin::new(2000, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateSavedTipsRecordSize { new_size: 0 },
        )
        .unwrap();

        assert!(get_user_sent_tips(deps.as_mut(), USER_1).is_empty());
        assert!(get_user_sent_tips(deps.as_mut(), USER_3).is_empty());
        assert!(get_user_received_tips(deps.as_mut(), USER_2).is_empty());
        assert!(get_user_received_tips(deps.as_mut(), USER_1).is_empty());
        assert!(get_post_tips(deps.as_mut(), 1, "").is_empty());
    }

    #[test]
    fn claim_fee_from_non_admin() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
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
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
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
    fn query_config_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            5,
        )
        .unwrap();

        let response = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let config_response = from_binary::<QueryConfigResponse>(&response).unwrap();

        assert_eq!(ADMIN, config_response.admin.as_str());
        assert_eq!(Uint64::new(1), config_response.subspace_id);
        assert_eq!(
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")]
            }),
            config_response.service_fee
        );
        assert_eq!(5, config_response.saved_tips_record_size)
    }

    #[test]
    fn query_user_received_tips_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            5,
        )
        .unwrap();

        tip_user(
            deps.as_mut(),
            USER_1,
            USER_3,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_2,
            USER_3,
            &[Coin::new(2000, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_2,
            USER_3,
            &[Coin::new(2000, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap();
        // Some more data not related to user 3
        tip_user(
            deps.as_mut(),
            USER_2,
            USER_1,
            &[Coin::new(100000, "udsm")],
            &[Coin::new(99000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(100000, "udsm")],
            &[Coin::new(99000, "udsm")],
        )
        .unwrap();

        let response = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::UserReceivedTips {
                user: USER_3.to_string(),
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
                        receiver: Addr::unchecked(USER_3),
                        amount: vec![Coin::new(4000, "udsm")],
                        post_id: None
                    },
                    Tip {
                        sender: Addr::unchecked(USER_2),
                        receiver: Addr::unchecked(USER_3),
                        amount: vec![Coin::new(1000, "udsm")],
                        post_id: None
                    },
                    Tip {
                        sender: Addr::unchecked(USER_2),
                        receiver: Addr::unchecked(USER_3),
                        amount: vec![Coin::new(1000, "udsm")],
                        post_id: None
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
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            5,
        )
        .unwrap();

        tip_user(
            deps.as_mut(),
            USER_1,
            USER_3,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_2,
            USER_3,
            &[Coin::new(2000, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_2,
            USER_3,
            &[Coin::new(2000, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_2,
            USER_1,
            &[Coin::new(100000, "udsm")],
            &[Coin::new(99000, "udsm")],
        )
        .unwrap();
        tip_user(
            deps.as_mut(),
            USER_1,
            USER_2,
            &[Coin::new(100000, "udsm")],
            &[Coin::new(99000, "udsm")],
        )
        .unwrap();

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
                        receiver: Addr::unchecked(USER_3),
                        amount: vec![Coin::new(4000, "udsm")],
                        post_id: None
                    },
                    Tip {
                        sender: Addr::unchecked(USER_1),
                        receiver: Addr::unchecked(USER_2),
                        amount: vec![Coin::new(99000, "udsm")],
                        post_id: None
                    },
                ]
            }
        )
    }

    #[test]
    fn query_tips_with_invalid_post_id() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            5,
        )
        .unwrap();

        let error = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::PostReceivedTips {
                post_id: Uint64::new(0),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::InvalidPostId {}, error);
    }

    #[test]
    fn query_tips_with_not_tipped_post_id() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            5,
        )
        .unwrap();

        let response = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::PostReceivedTips {
                post_id: Uint64::new(7),
            },
        )
        .unwrap();
        let tips: TipsResponse = from_binary(&response).unwrap();

        assert_eq!(Vec::<Tip>::new(), tips.tips);
    }

    #[test]
    fn query_post_received_tips_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        init_contract(
            deps.as_mut(),
            1,
            Some(ServiceFee::Fixed {
                amount: vec![Coin::new(1000, "udsm")],
            }),
            5,
        )
        .unwrap();

        tip_post(
            deps.as_mut(),
            USER_1,
            1,
            &[Coin::new(5000, "udsm")],
            &[Coin::new(4000, "udsm")],
        )
        .unwrap();
        tip_post(
            deps.as_mut(),
            USER_2,
            1,
            &[Coin::new(2000, "udsm")],
            &[Coin::new(1000, "udsm")],
        )
        .unwrap();
        tip_post(
            deps.as_mut(),
            USER_3,
            1,
            &[Coin::new(100000, "udsm")],
            &[Coin::new(99000, "udsm")],
        )
        .unwrap();
        tip_post(
            deps.as_mut(),
            USER_1,
            1,
            &[Coin::new(100000, "udsm")],
            &[Coin::new(99000, "udsm")],
        )
        .unwrap();

        let response = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::PostReceivedTips {
                post_id: Uint64::new(1),
            },
        )
        .unwrap();
        let tips: TipsResponse = from_binary(&response).unwrap();
        assert_eq!(4, tips.tips.len());
        assert_eq!(
            tips,
            TipsResponse {
                tips: vec![
                    Tip {
                        sender: Addr::unchecked(USER_1),
                        receiver: Addr::unchecked(POST_AUTHOR),
                        amount: vec![Coin::new(4000, "udsm")],
                        post_id: Some(Uint64::new(1))
                    },
                    Tip {
                        sender: Addr::unchecked(USER_2),
                        receiver: Addr::unchecked(POST_AUTHOR),
                        amount: vec![Coin::new(1000, "udsm")],
                        post_id: Some(Uint64::new(1))
                    },
                    Tip {
                        sender: Addr::unchecked(USER_3),
                        receiver: Addr::unchecked(POST_AUTHOR),
                        amount: vec![Coin::new(99000, "udsm")],
                        post_id: Some(Uint64::new(1))
                    },
                    Tip {
                        sender: Addr::unchecked(USER_1),
                        receiver: Addr::unchecked(POST_AUTHOR),
                        amount: vec![Coin::new(99000, "udsm")],
                        post_id: Some(Uint64::new(1))
                    },
                ]
            }
        )
    }
}
