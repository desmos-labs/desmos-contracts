use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, QueryPendingTipsResponse,
    QueryUnclaimedSentTipsResponse,
};
use crate::state::{pending_tips, Config, PendingTip, CONFIG};
use crate::utils::{serialize_coins, sum_coins_sorted};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Order, Response,
    StdResult, Storage, Uint64,
};
use cw2::set_contract_version;
use desmos_bindings::msg::DesmosMsg;
use desmos_bindings::profiles::models_app_links::ApplicationLinkState;
use desmos_bindings::profiles::querier::ProfilesQuerier;
use desmos_bindings::query::DesmosQuery;
use desmos_bindings::types::PageRequest;
use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:social-tips";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// consts
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_TIP_COLLECTED: &str = "tip_collected";
const ATTRIBUTE_TIP_CLAIMER: &str = "tip_claimer";
const ATTRIBUTE_TIP_AMOUNT: &str = "tip_amount";
const ATTRIBUTE_REMOVED_TIP_AMOUNT: &str = "removed_tip_amount";
const ATTRIBUTE_NEW_MAX_PENDING_TIPS_VALUE: &str = "new_max_pending_tips_value";
const ATTRIBUTE_NEW_MAX_SENT_PENDING_TIPS_VALUE: &str = "new_max_sent_pending_tips_value";
const ATTRIBUTE_NEW_ADMIN: &str = "new_admin";
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_SEND_TIPS: &str = "send_tips";
const ACTION_UPDATE_ADMIN: &str = "update_admin";
const ACTION_CLAIM_PENDING_TIPS: &str = "claim_pending_tips";
const ACTION_UPDATE_MAX_PENDING_TIPS: &str = "update_max_pending_tips";
const ACTION_UPDATE_MAX_SENT_PENDING_TIPS: &str = "update_max_sent_pending_tips";
const ACTION_REMOVE_PENDING_TIP: &str = "remove_pending_tip";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    msg.validate()?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = if let Some(address) = msg.admin {
        deps.api.addr_validate(&address)?
    } else {
        info.sender
    };

    CONFIG.save(
        deps.storage,
        &Config {
            admin,
            max_pending_tips: msg.max_pending_tips,
            max_sent_pending_tips: msg.max_sent_pending_tips,
        },
    )?;

    Ok(Response::new().add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE))
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
        ExecuteMsg::SendTip {
            application,
            handle,
            owner_index,
        } => send_tip(deps, env, info, application, handle, owner_index),
        ExecuteMsg::ClaimTips {} => claim_tips(deps, info),
        ExecuteMsg::UpdateAdmin { new_admin } => update_admin(deps, info, new_admin),
        ExecuteMsg::UpdateMaxPendingTips { value } => update_max_pending_tips(deps, info, value),
        ExecuteMsg::UpdateMaxSentPendingTips { value } => {
            update_max_sent_pending_tips(deps, info, value)
        }
        ExecuteMsg::RemovePendingTip {
            application,
            handle,
        } => remove_pending_tip(deps, info, application, handle),
    }
}

fn check_admin(store: &dyn Storage, sender: &Addr) -> Result<(), ContractError> {
    let config = CONFIG.load(store)?;
    if config.admin.eq(sender) {
        Ok(())
    } else {
        Err(ContractError::Unauthorized {})
    }
}

pub fn send_tip(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    application: String,
    handle: String,
    owner_index: Option<Uint64>,
) -> Result<Response<DesmosMsg>, ContractError> {
    let querier = ProfilesQuerier::new(deps.querier.deref());
    let sender = info.sender;
    let funds = sum_coins_sorted(info.funds)?;

    if funds.is_empty() {
        return Err(ContractError::EmptyTipAmount {});
    }

    // Query users that have that application linked to their accounts.
    let response = querier.query_application_link_owners(
        Some(application.clone()),
        Some(handle.clone()),
        Some(PageRequest {
            key: None,
            limit: Uint64::new(1),
            offset: owner_index,
            reverse: false,
            count_total: false,
        }),
    )?;

    let serialized_coins = serialize_coins(&funds);

    return if !response.owners.is_empty() {
        let owner = response.owners.first().unwrap().user.to_string();

        Ok(Response::new()
            .add_attribute(ATTRIBUTE_ACTION, ACTION_SEND_TIPS)
            .add_attribute(ATTRIBUTE_TIP_COLLECTED, "false")
            .add_attribute(ATTRIBUTE_TIP_CLAIMER, &owner)
            .add_attribute(ATTRIBUTE_TIP_AMOUNT, serialized_coins)
            .add_message(BankMsg::Send {
                amount: funds,
                to_address: owner,
            }))
    } else {
        let config = CONFIG.load(deps.storage)?;
        let tips = pending_tips();

        let user_sent_pending_tips_count = tips
            .idx
            .sender
            .prefix(sender.clone())
            .range_raw(deps.storage, None, None, Order::Ascending)
            .count();

        // Ensure that the sender can not spam the chain with multiple tips to different users.
        if user_sent_pending_tips_count >= config.max_sent_pending_tips as usize {
            return Err(ContractError::ToManySentPendingTips {});
        }

        let user_pending_tips_count = tips
            .prefix((application.clone(), handle.clone()))
            .range_raw(deps.storage, None, None, Order::Ascending)
            .count();

        // Ensure that the user don't have to many pending tips.
        if user_pending_tips_count >= config.max_pending_tips as usize {
            return Err(ContractError::ToManyPendingTipsForUser {
                application,
                handle,
            });
        }

        let key = (application, handle, sender.clone());
        let replaced = tips.may_load(deps.storage, key.clone())?;

        tips.replace(
            deps.storage,
            key,
            Some(&PendingTip {
                sender,
                amount: funds,
                block_height: env.block.height,
            }),
            replaced.as_ref(),
        )?;

        let mut response = Response::new()
            .add_attribute(ATTRIBUTE_ACTION, ACTION_SEND_TIPS)
            .add_attribute(ATTRIBUTE_TIP_COLLECTED, "true")
            .add_attribute(ATTRIBUTE_TIP_AMOUNT, serialized_coins);

        // Send back the funds of the replaced tip.
        if let Some(replaced_tip) = replaced {
            response = response.add_message(BankMsg::Send {
                amount: replaced_tip.amount,
                to_address: replaced_tip.sender.to_string(),
            });
        }

        Ok(response)
    };
}

pub fn claim_tips(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut coins = Vec::<Coin>::new();
    let querier = ProfilesQuerier::new(deps.querier.deref());
    let pending_tips_map = pending_tips();

    for app_link_result in
        querier.iterate_application_links(Some(info.sender.clone()), None, None, 10)
    {
        let app_link = app_link_result?;
        if app_link.state == ApplicationLinkState::VerificationSuccess {
            let mut pending_tips = pending_tips_map
                .prefix((
                    app_link.data.application.clone(),
                    app_link.data.username.clone(),
                ))
                .range(deps.storage, None, None, Order::Ascending)
                .collect::<StdResult<Vec<_>>>()?;

            for (sender, mut pending_tip) in pending_tips.drain(0..) {
                pending_tip
                    .amount
                    .drain(0..)
                    .for_each(|coin| coins.push(coin));
                pending_tips_map.remove(
                    deps.storage,
                    (
                        app_link.data.application.clone(),
                        app_link.data.username.clone(),
                        sender,
                    ),
                )?;
            }
        }
    }

    let merged_coins = sum_coins_sorted(coins)?;

    if merged_coins.is_empty() {
        return Err(ContractError::NoTipsAvailable {
            user: info.sender.to_string(),
        });
    }

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_CLAIM_PENDING_TIPS)
        .add_attribute(ATTRIBUTE_TIP_CLAIMER, &info.sender)
        .add_attribute(ATTRIBUTE_TIP_AMOUNT, serialize_coins(&merged_coins))
        .add_message(BankMsg::Send {
            amount: merged_coins,
            to_address: info.sender.to_string(),
        }))
}

fn update_admin(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let new_admin_addr = deps.api.addr_validate(&new_admin)?;
    check_admin(deps.storage, &info.sender)?;

    CONFIG.update::<_, ContractError>(deps.storage, |mut config| {
        config.admin = new_admin_addr;
        Ok(config)
    })?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_ADMIN)
        .add_attribute(ATTRIBUTE_NEW_ADMIN, &new_admin))
}

fn update_max_pending_tips(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    value: u16,
) -> Result<Response<DesmosMsg>, ContractError> {
    check_admin(deps.storage, &info.sender)?;

    CONFIG.update::<_, ContractError>(deps.storage, |mut config| {
        config.max_pending_tips = value;
        Ok(config)
    })?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_MAX_PENDING_TIPS)
        .add_attribute(ATTRIBUTE_NEW_MAX_PENDING_TIPS_VALUE, value.to_string()))
}

fn update_max_sent_pending_tips(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    value: u16,
) -> Result<Response<DesmosMsg>, ContractError> {
    check_admin(deps.storage, &info.sender)?;

    CONFIG.update::<_, ContractError>(deps.storage, |mut config| {
        config.max_sent_pending_tips = value;
        Ok(config)
    })?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_MAX_SENT_PENDING_TIPS)
        .add_attribute(ATTRIBUTE_NEW_MAX_SENT_PENDING_TIPS_VALUE, value.to_string()))
}

fn remove_pending_tip(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    application: String,
    handle: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let pending_tips_map = pending_tips();
    let key = (application.clone(), handle.clone(), info.sender);
    let pending_tip_option = pending_tips_map.may_load(deps.storage, key.clone())?;

    if let Some(to_remove_tip) = pending_tip_option {
        let refund_address = key.2.to_string();
        pending_tips_map.replace(deps.storage, key, None, Some(&to_remove_tip))?;

        Ok(Response::new()
            .add_attribute(ATTRIBUTE_ACTION, ACTION_REMOVE_PENDING_TIP)
            .add_attribute(
                ATTRIBUTE_REMOVED_TIP_AMOUNT,
                serialize_coins(&to_remove_tip.amount),
            )
            .add_message(BankMsg::Send {
                amount: to_remove_tip.amount,
                to_address: refund_address,
            }))
    } else {
        Err(ContractError::NoPendingTip {
            application,
            handle,
        })
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::UserPendingTips { user } => to_binary(&query_user_pending_tips(deps, user)?),
        QueryMsg::UnclaimedSentTips { user } => to_binary(&query_unclaimed_sent_tips(deps, user)?),
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_user_pending_tips(
    deps: Deps<DesmosQuery>,
    user: String,
) -> StdResult<QueryPendingTipsResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let querier = ProfilesQuerier::new(deps.querier.deref());
    let mut tips = Vec::<PendingTip>::new();

    for app_link_result in querier.iterate_application_links(Some(user_addr), None, None, 10) {
        let app_link = app_link_result?;
        if app_link.state == ApplicationLinkState::VerificationSuccess {
            let key_prefix = (app_link.data.application, app_link.data.username);
            pending_tips()
                .prefix(key_prefix)
                .range(deps.storage, None, None, Order::Ascending)
                .try_for_each(|item| {
                    if let Ok((sender, pending_tip)) = item {
                        tips.push(PendingTip {
                            sender,
                            amount: pending_tip.amount,
                            block_height: pending_tip.block_height,
                        });
                        Ok(())
                    } else {
                        Err(item.unwrap_err())
                    }
                })?;
        }
    }

    Ok(QueryPendingTipsResponse { tips })
}

fn query_unclaimed_sent_tips(
    deps: Deps<DesmosQuery>,
    sender: String,
) -> StdResult<QueryUnclaimedSentTipsResponse> {
    let sender = deps.api.addr_validate(&sender)?;

    let tips = pending_tips()
        .idx
        .sender
        .prefix(sender)
        .range(deps.storage, None, None, Order::Ascending)
        .map(|item| item.map(|item| item.1))
        .collect::<StdResult<_>>()?;

    Ok(QueryUnclaimedSentTipsResponse { tips })
}

fn query_config(deps: Deps<DesmosQuery>) -> StdResult<QueryConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    Ok(QueryConfigResponse {
        admin: config.admin,
        max_pending_tips: config.max_pending_tips,
        max_sent_pending_tips: config.max_sent_pending_tips,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::{
        ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, QueryPendingTipsResponse,
        QueryUnclaimedSentTipsResponse,
    };
    use crate::state::{
        pending_tips, PendingTip, CONFIG, MAX_CONFIGURABLE_PENDING_TIPS,
        MAX_CONFIGURABLE_SENT_PENDING_TIPS,
    };
    use crate::ContractError;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{
        from_binary, to_binary, Addr, BankMsg, Coin, ContractResult, DepsMut, Order, Response,
        StdResult, SubMsg, Uint64,
    };
    use desmos_bindings::mocks::mock_queriers::{
        mock_desmos_dependencies, mock_desmos_dependencies_with_custom_querier, MockDesmosQuerier,
    };
    use desmos_bindings::msg::DesmosMsg;
    use desmos_bindings::profiles::mocks::mock_profiles_query_response;
    use desmos_bindings::profiles::models_app_links::{
        ApplicationLink, ApplicationLinkOwnerDetails, ApplicationLinkState, CallData, Data,
        OracleRequest,
    };
    use desmos_bindings::profiles::models_query::{
        QueryApplicationLinkOwnersResponse, QueryApplicationLinksResponse,
    };
    use desmos_bindings::profiles::query::ProfilesQuery;
    use desmos_bindings::query::DesmosQuery;

    const ADMIN: &str = "admin";
    const SENDER: &str = "user1";
    const CLAIMER: &str = "user2";

    fn init_contract(
        deps: DepsMut<DesmosQuery>,
        max_pending_tips: u16,
        max_sent_pending_tips: u16,
    ) -> Result<Response<DesmosMsg>, ContractError> {
        let info = mock_info(ADMIN, &[]);
        let env = mock_env();
        instantiate(
            deps,
            env,
            info,
            InstantiateMsg {
                admin: None,
                max_pending_tips,
                max_sent_pending_tips,
            },
        )
    }

    fn querier_with_no_app_links() -> MockDesmosQuerier {
        MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
            match profiler_query {
                ProfilesQuery::ApplicationLinkOwners { .. } => {
                    let response = QueryApplicationLinkOwnersResponse {
                        owners: vec![],
                        pagination: None,
                    };
                    to_binary(&response).into()
                }
                _ => mock_profiles_query_response(profiler_query),
            }
        })
    }

    fn get_pending_tips(
        deps: DepsMut<DesmosQuery>,
        application: &str,
        handle: &str,
    ) -> Vec<PendingTip> {
        pending_tips()
            .prefix((application.to_string(), handle.to_string()))
            .range(deps.storage, None, None, Order::Ascending)
            .map(|item| item.map(|(_, pending_tip)| pending_tip))
            .collect::<StdResult<_>>()
            .unwrap()
    }

    #[test]
    fn instantiate_properly() {
        let mut deps = mock_desmos_dependencies();
        init_contract(deps.as_mut(), 10, 10).unwrap();
    }

    #[test]
    fn instantiate_with_invalid_max_pending_tips_error() {
        let mut deps = mock_desmos_dependencies();

        let error = init_contract(deps.as_mut(), 0, 10).unwrap_err();
        assert_eq!(
            ContractError::InvalidMaxPendingTipsValue {
                max: MAX_CONFIGURABLE_PENDING_TIPS,
                value: 0,
            },
            error
        );

        let error =
            init_contract(deps.as_mut(), MAX_CONFIGURABLE_PENDING_TIPS + 1, 10).unwrap_err();
        assert_eq!(
            ContractError::InvalidMaxPendingTipsValue {
                max: MAX_CONFIGURABLE_PENDING_TIPS,
                value: MAX_CONFIGURABLE_PENDING_TIPS + 1,
            },
            error
        );
    }

    #[test]
    fn instantiate_with_invalid_max_sent_pending_tips_error() {
        let mut deps = mock_desmos_dependencies();

        let error = init_contract(deps.as_mut(), 10, 0).unwrap_err();
        assert_eq!(
            ContractError::InvalidMaxSentPendingTipsValue {
                max: MAX_CONFIGURABLE_SENT_PENDING_TIPS,
                value: 0,
            },
            error
        );

        let error =
            init_contract(deps.as_mut(), 10, MAX_CONFIGURABLE_SENT_PENDING_TIPS + 1).unwrap_err();
        assert_eq!(
            ContractError::InvalidMaxSentPendingTipsValue {
                max: MAX_CONFIGURABLE_SENT_PENDING_TIPS,
                value: MAX_CONFIGURABLE_SENT_PENDING_TIPS + 1,
            },
            error
        );
    }

    #[test]
    fn tip_with_empty_application_error() {
        let mut deps = mock_desmos_dependencies();
        let env = mock_env();
        let info = mock_info(SENDER, &[Coin::new(10_000, "udsm")]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "".to_string(),
                handle: "user".to_string(),
                owner_index: None,
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::InvalidApplication {}, error);
    }

    #[test]
    fn tip_with_empty_handle_error() {
        let mut deps = mock_desmos_dependencies();
        let env = mock_env();
        let info = mock_info(SENDER, &[Coin::new(10_000, "udsm")]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "".to_string(),
                owner_index: None,
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::InvalidUserHandle {}, error);
    }

    #[test]
    fn tip_with_empty_funds_error() {
        let mut deps = mock_desmos_dependencies();
        let env = mock_env();
        let info = mock_info(SENDER, &[]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
                owner_index: None,
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::EmptyTipAmount {}, error);
    }

    #[test]
    fn tip_user_linked_to_multiple_addresses_properly() {
        const OWNER_INDEX: u64 = 1u64;

        let querier = MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
            match profiler_query {
                ProfilesQuery::ApplicationLinkOwners { pagination, .. } => {
                    if pagination.is_none()
                        || pagination.as_ref().unwrap().offset.is_none()
                        || pagination.as_ref().unwrap().offset.unwrap() != Uint64::new(OWNER_INDEX)
                    {
                        return ContractResult::Err("invalid pagination offset".to_string());
                    }
                    let response = QueryApplicationLinkOwnersResponse {
                        owners: vec![ApplicationLinkOwnerDetails {
                            user: Addr::unchecked(CLAIMER),
                            application: "mocked_app".to_string(),
                            username: "mocked_username".to_string(),
                        }],
                        pagination: None,
                    };
                    to_binary(&response).into()
                }
                _ => mock_profiles_query_response(profiler_query),
            }
        });

        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);
        let env = mock_env();
        let info = mock_info(SENDER, &[Coin::new(10_000, "udsm")]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let response = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
                owner_index: Some(Uint64::new(OWNER_INDEX)),
            },
        )
        .unwrap();

        assert_eq!(
            vec![SubMsg::new(BankMsg::Send {
                amount: vec![Coin::new(10_000, "udsm")],
                to_address: CLAIMER.to_string()
            })],
            response.messages
        );
    }

    #[test]
    fn tip_collected_properly() {
        let querier = querier_with_no_app_links();
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);
        let env = mock_env();
        let info = mock_info(SENDER, &[Coin::new(10_000, "udsm")]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
                owner_index: None,
            },
        )
        .unwrap();

        let pending_tips = get_pending_tips(deps.as_mut(), "application", "handle");

        assert_eq!(
            vec![PendingTip {
                sender: Addr::unchecked(SENDER),
                amount: vec![Coin::new(10_000, "udsm")],
                block_height: 12345
            }],
            pending_tips
        )
    }

    #[test]
    fn reach_max_pending_tips_error() {
        let querier = querier_with_no_app_links();
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);

        init_contract(deps.as_mut(), 3, 10).unwrap();

        for i in 0..3 {
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(&format!("user{}", i), &[Coin::new(10_000, "udsm")]),
                ExecuteMsg::SendTip {
                    application: "application".to_string(),
                    handle: "handle".to_string(),
                    owner_index: None,
                },
            )
            .unwrap();
        }

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[Coin::new(10_000, "udsm")]),
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
                owner_index: None,
            },
        )
        .unwrap_err();

        assert_eq!(
            ContractError::ToManyPendingTipsForUser {
                handle: "handle".to_string(),
                application: "application".to_string()
            },
            error
        );
    }

    #[test]
    fn reach_max_sent_pending_tips_error() {
        let querier = querier_with_no_app_links();
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);

        init_contract(deps.as_mut(), 10, 3).unwrap();

        for i in 0..3 {
            execute(
                deps.as_mut(),
                mock_env(),
                mock_info(SENDER, &[Coin::new(10_000, "udsm")]),
                ExecuteMsg::SendTip {
                    application: "application".to_string(),
                    handle: format!("handle{}", i),
                    owner_index: None,
                },
            )
            .unwrap();
        }

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[Coin::new(10_000, "udsm")]),
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handle3".to_string(),
                owner_index: None,
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::ToManySentPendingTips {}, error);
    }

    #[test]
    fn replaced_tip_sends_refund_properly() {
        let querier = querier_with_no_app_links();

        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[Coin::new(10_000, "udsm")]),
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
                owner_index: None,
            },
        )
        .unwrap();

        let response = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[Coin::new(20_000, "udsm")]),
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
                owner_index: None,
            },
        )
        .unwrap();

        assert_eq!(
            vec![SubMsg::new(BankMsg::Send {
                amount: vec![Coin::new(10_000, "udsm")],
                to_address: SENDER.to_string()
            })],
            response.messages
        );

        let pending_tips = get_pending_tips(deps.as_mut(), "application", "handle");
        assert_eq!(
            vec![PendingTip {
                sender: Addr::unchecked(SENDER),
                amount: vec![Coin::new(20_000, "udsm")],
                block_height: 12345,
            }],
            pending_tips
        );
    }

    #[test]
    fn tip_sent_properly() {
        let querier = MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
            match profiler_query {
                ProfilesQuery::ApplicationLinkOwners { .. } => {
                    let response = QueryApplicationLinkOwnersResponse {
                        owners: vec![ApplicationLinkOwnerDetails {
                            user: Addr::unchecked(CLAIMER),
                            application: "application".to_string(),
                            username: "handle".to_string(),
                        }],
                        pagination: None,
                    };
                    to_binary(&response).into()
                }
                _ => mock_profiles_query_response(profiler_query),
            }
        });

        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);
        let env = mock_env();
        let info = mock_info(SENDER, &[Coin::new(10_000, "udsm")]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let response = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
                owner_index: None,
            },
        )
        .unwrap();

        assert_eq!(
            &SubMsg::<DesmosMsg>::new(BankMsg::Send {
                to_address: CLAIMER.to_string(),
                amount: vec![Coin::new(10_000, "udsm")],
            }),
            response.messages.first().unwrap()
        )
    }

    #[test]
    fn claim_no_pending_tips_error() {
        let mut deps = mock_desmos_dependencies();
        let env = mock_env();
        let info = mock_info(CLAIMER, &[]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(deps.as_mut(), env, info, ExecuteMsg::ClaimTips {}).unwrap_err();

        assert_eq!(
            error,
            ContractError::NoTipsAvailable {
                user: CLAIMER.to_string()
            }
        )
    }

    #[test]
    fn claim_pending_tip_properly() {
        let querier = querier_with_no_app_links();
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);
        let env = mock_env();
        let info = mock_info(SENDER, &[Coin::new(10_000, "udsm")]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handler".to_string(),
                owner_index: None,
            },
        )
        .unwrap();

        // Mock USER_2 app link
        deps.querier =
            MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
                match profiler_query {
                    ProfilesQuery::ApplicationLinks { .. } => {
                        let response = QueryApplicationLinksResponse {
                            links: vec![ApplicationLink {
                                user: Addr::unchecked(CLAIMER),
                                data: Data {
                                    username: "handler".to_string(),
                                    application: "application".to_string(),
                                },
                                state: ApplicationLinkState::VerificationSuccess,
                                oracle_request: OracleRequest {
                                    id: Uint64::new(0),
                                    oracle_script_id: Uint64::new(0),
                                    call_data: CallData {
                                        application: "".to_string(),
                                        call_data: "".to_string(),
                                    },
                                    client_id: "".to_string(),
                                },
                                result: None,
                                creation_time: "".to_string(),
                                expiration_time: "".to_string(),
                            }],
                            pagination: None,
                        };
                        to_binary(&response).into()
                    }
                    _ => mock_profiles_query_response(profiler_query),
                }
            });
        let env = mock_env();
        let info = mock_info(CLAIMER, &[Coin::new(10_000, "udsm")]);

        let response = execute(deps.as_mut(), env, info, ExecuteMsg::ClaimTips {}).unwrap();
        assert_eq!(
            &SubMsg::<DesmosMsg>::new(BankMsg::Send {
                to_address: CLAIMER.to_string(),
                amount: vec![Coin::new(10_000, "udsm")],
            }),
            response.messages.first().unwrap()
        );

        // Ensure that the claimed tips have been deleted from the contract state
        let pending_tips = get_pending_tips(deps.as_mut(), "application", "handle");
        assert_eq!(Vec::<PendingTip>::new(), pending_tips);
    }

    #[test]
    fn update_admin_from_non_admin_error() {
        let mut deps = mock_desmos_dependencies();

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[]),
            ExecuteMsg::UpdateAdmin {
                new_admin: SENDER.to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::Unauthorized {}, error)
    }

    #[test]
    fn update_admin_properly() {
        let mut deps = mock_desmos_dependencies();

        init_contract(deps.as_mut(), 10, 10).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateAdmin {
                new_admin: SENDER.to_string(),
            },
        )
        .unwrap();
    }

    #[test]
    fn update_max_pending_tip_from_non_admin_error() {
        let mut deps = mock_desmos_dependencies();

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[]),
            ExecuteMsg::UpdateMaxPendingTips { value: 2 },
        )
        .unwrap_err();

        assert_eq!(ContractError::Unauthorized {}, error)
    }

    #[test]
    fn update_max_pending_tip_with_invalid_value_error() {
        let mut deps = mock_desmos_dependencies();

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateMaxPendingTips { value: 0 },
        )
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxPendingTipsValue {
                value: 0,
                max: MAX_CONFIGURABLE_PENDING_TIPS
            },
            error
        );

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateMaxPendingTips {
                value: MAX_CONFIGURABLE_PENDING_TIPS + 1,
            },
        )
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxPendingTipsValue {
                value: MAX_CONFIGURABLE_PENDING_TIPS + 1,
                max: MAX_CONFIGURABLE_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn update_max_pending_tip_properly() {
        let mut deps = mock_desmos_dependencies();

        init_contract(deps.as_mut(), 10, 10).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateMaxPendingTips { value: 5 },
        )
        .unwrap();

        assert_eq!(
            5,
            CONFIG.load(deps.as_mut().storage).unwrap().max_pending_tips
        )
    }

    #[test]
    fn update_max_sent_pending_tip_from_non_admin_error() {
        let mut deps = mock_desmos_dependencies();

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[]),
            ExecuteMsg::UpdateMaxSentPendingTips { value: 2 },
        )
        .unwrap_err();

        assert_eq!(ContractError::Unauthorized {}, error)
    }

    #[test]
    fn update_max_sent_pending_tip_with_invalid_value_error() {
        let mut deps = mock_desmos_dependencies();

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateMaxSentPendingTips { value: 0 },
        )
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxSentPendingTipsValue {
                value: 0,
                max: MAX_CONFIGURABLE_SENT_PENDING_TIPS
            },
            error
        );

        let error = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateMaxSentPendingTips {
                value: MAX_CONFIGURABLE_SENT_PENDING_TIPS + 1,
            },
        )
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxSentPendingTipsValue {
                value: MAX_CONFIGURABLE_SENT_PENDING_TIPS + 1,
                max: MAX_CONFIGURABLE_SENT_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn update_max_sent_pending_tip_properly() {
        let mut deps = mock_desmos_dependencies();

        init_contract(deps.as_mut(), 10, 10).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(ADMIN, &[]),
            ExecuteMsg::UpdateMaxSentPendingTips { value: 5 },
        )
        .unwrap();

        assert_eq!(
            5,
            CONFIG
                .load(deps.as_mut().storage)
                .unwrap()
                .max_sent_pending_tips
        )
    }

    #[test]
    fn remove_non_existing_pending_tip_error() {
        let mut deps = mock_desmos_dependencies();
        let env = mock_env();
        let info = mock_info(SENDER, &[]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        let error = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::RemovePendingTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(
            error,
            ContractError::NoPendingTip {
                application: "application".to_string(),
                handle: "handle".to_string()
            }
        )
    }

    #[test]
    fn remove_pending_tip_properly() {
        let querier = querier_with_no_app_links();
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[Coin::new(10_000, "udsm")]),
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
                owner_index: None,
            },
        )
        .unwrap();

        let response = execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[]),
            ExecuteMsg::RemovePendingTip {
                application: "application".to_string(),
                handle: "handle".to_string(),
            },
        )
        .unwrap();

        assert_eq!(
            vec![SubMsg::new(BankMsg::Send {
                amount: vec![Coin::new(10_000, "udsm")],
                to_address: SENDER.to_string()
            })],
            response.messages,
        );

        let pending_tips = get_pending_tips(deps.as_mut(), "application", "handle");
        assert_eq!(Vec::<PendingTip>::new(), pending_tips);
    }

    #[test]
    fn query_tips_properly() {
        let querier = querier_with_no_app_links();
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);
        let env = mock_env();
        let info = mock_info(SENDER, &[Coin::new(10_000, "udsm")]);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handler".to_string(),
                owner_index: None,
            },
        )
        .unwrap();

        // Mock USER_2 app link
        deps.querier =
            MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
                match profiler_query {
                    ProfilesQuery::ApplicationLinks { .. } => {
                        let response = QueryApplicationLinksResponse {
                            links: vec![ApplicationLink {
                                user: Addr::unchecked(CLAIMER),
                                data: Data {
                                    username: "handler".to_string(),
                                    application: "application".to_string(),
                                },
                                state: ApplicationLinkState::VerificationSuccess,
                                oracle_request: OracleRequest {
                                    id: Uint64::new(0),
                                    oracle_script_id: Uint64::new(0),
                                    call_data: CallData {
                                        application: "".to_string(),
                                        call_data: "".to_string(),
                                    },
                                    client_id: "".to_string(),
                                },
                                result: None,
                                creation_time: "".to_string(),
                                expiration_time: "".to_string(),
                            }],
                            pagination: None,
                        };
                        to_binary(&response).into()
                    }
                    _ => mock_profiles_query_response(profiler_query),
                }
            });

        let response = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::UserPendingTips {
                user: CLAIMER.to_string(),
            },
        )
        .unwrap();

        let response: QueryPendingTipsResponse = from_binary(&response).unwrap();
        assert_eq!(
            response.tips,
            vec![PendingTip {
                sender: Addr::unchecked(SENDER),
                amount: vec![Coin::new(10_000, "udsm")],
                block_height: 12345
            }]
        )
    }

    #[test]
    fn query_unclaimed_sent_tips_properly() {
        let querier = querier_with_no_app_links();
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);

        init_contract(deps.as_mut(), 10, 10).unwrap();

        execute(
            deps.as_mut(),
            mock_env(),
            mock_info(SENDER, &[Coin::new(10_000, "udsm")]),
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handle: "handler".to_string(),
                owner_index: None,
            },
        )
        .unwrap();

        let response = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::UnclaimedSentTips {
                user: SENDER.to_string(),
            },
        )
        .unwrap();

        let response: QueryUnclaimedSentTipsResponse = from_binary(&response).unwrap();
        assert_eq!(
            response.tips,
            vec![PendingTip {
                sender: Addr::unchecked(SENDER),
                amount: vec![Coin::new(10_000, "udsm")],
                block_height: 12345
            }]
        )
    }

    #[test]
    fn query_config_properly() {
        let querier = querier_with_no_app_links();
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);

        init_contract(deps.as_mut(), 5, 10).unwrap();

        let response = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();

        let response: QueryConfigResponse = from_binary(&response).unwrap();
        assert_eq!(
            response,
            QueryConfigResponse {
                admin: Addr::unchecked(ADMIN),
                max_pending_tips: 5u16,
                max_sent_pending_tips: 10u16
            }
        )
    }
}
