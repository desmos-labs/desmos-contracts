use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryPendingTipsResponse};
use crate::state::{PendingTip, PendingTips, PENDING_TIPS};
use crate::utils::{serialize_coins, sum_coins_sorted};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use desmos_bindings::msg::DesmosMsg;
use desmos_bindings::profiles::models_app_links::ApplicationLinkState;
use desmos_bindings::profiles::querier::ProfilesQuerier;
use desmos_bindings::query::DesmosQuery;
use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:social-tips";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// consts
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_TIP_COLLECTED: &str = "tip_collected";
const ATTRIBUTE_TIP_RECEIVER: &str = "tip_receiver";
const ATTRIBUTE_TIP_AMOUNT: &str = "tip_amount";
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_SEND_TIPS: &str = "send_tips";
const ACTION_CLAIM_PENDING_TIPS: &str = "claim_pending_tips";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

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
            handler,
        } => send_tip(deps, env, info, application, handler),
        ExecuteMsg::ClaimTips {} => claim_tips(deps, info),
    }
}

pub fn send_tip(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    application: String,
    handler: String,
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
        Some(handler.clone()),
        None,
    )?;

    // Fail if there are more then 1 users with the same application link.
    if response.owners.len() > 1 {
        return Err(ContractError::ToManyOwners {});
    }

    let serialized_coins = serialize_coins(&funds);

    return if !response.owners.is_empty() {
        let owner = response.owners.first().unwrap().user.to_string();

        Ok(Response::new()
            .add_attribute(ATTRIBUTE_ACTION, ACTION_SEND_TIPS)
            .add_attribute(ATTRIBUTE_TIP_COLLECTED, "false")
            .add_attribute(ATTRIBUTE_TIP_RECEIVER, &owner)
            .add_attribute(ATTRIBUTE_TIP_AMOUNT, serialized_coins)
            .add_message(BankMsg::Send {
                amount: funds,
                to_address: owner,
            }))
    } else {
        // No one have linked the provided username and application.
        PENDING_TIPS.update::<_, ContractError>(deps.storage, (application, handler), |tips| {
            let mut tips = tips.unwrap_or_default();
            tips.push(PendingTip {
                sender,
                amount: funds,
                block_height: env.block.height,
            });
            Ok(tips)
        })?;

        Ok(Response::new()
            .add_attribute(ATTRIBUTE_ACTION, ACTION_SEND_TIPS)
            .add_attribute(ATTRIBUTE_TIP_COLLECTED, "true")
            .add_attribute(ATTRIBUTE_TIP_AMOUNT, serialized_coins))
    };
}

pub fn claim_tips(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut coins = Vec::<Coin>::new();
    let querier = ProfilesQuerier::new(deps.querier.deref());

    for app_link_result in
        querier.iterate_application_links(Some(info.sender.clone()), None, None, 10)
    {
        let app_link = app_link_result?;
        if app_link.state == ApplicationLinkState::VerificationSuccess {
            let key = (app_link.data.application, app_link.data.username);
            let pending_tips = PENDING_TIPS.may_load(deps.storage, key.clone())?;

            if let Some(mut tips) = pending_tips {
                tips.drain(0..)
                    .for_each(|mut tip| tip.amount.drain(0..).for_each(|coin| coins.push(coin)));

                PENDING_TIPS.remove(deps.storage, key);
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
        .add_attribute(ATTRIBUTE_TIP_RECEIVER, &info.sender)
        .add_attribute(ATTRIBUTE_TIP_AMOUNT, serialize_coins(&merged_coins))
        .add_message(BankMsg::Send {
            amount: merged_coins,
            to_address: info.sender.to_string(),
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::UserPendingTips { user } => to_binary(&query_user_pending_tips(deps, user)?),
    }
}

fn query_user_pending_tips(
    deps: Deps<DesmosQuery>,
    user: String,
) -> StdResult<QueryPendingTipsResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let querier = ProfilesQuerier::new(deps.querier.deref());
    let mut tips = PendingTips::new();

    for app_link_result in querier.iterate_application_links(Some(user_addr), None, None, 10) {
        let app_link = app_link_result?;

        if app_link.state == ApplicationLinkState::VerificationSuccess {
            let key = (app_link.data.application, app_link.data.username);
            let pending_tips = PENDING_TIPS.may_load(deps.storage, key.clone())?;

            if let Some(mut pending_tips) = pending_tips {
                pending_tips.drain(0..).for_each(|tip| tips.push(tip));
            }
        }
    }

    Ok(QueryPendingTipsResponse { tips })
}

#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, query};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryPendingTipsResponse};
    use crate::state::{PendingTip, PENDING_TIPS};
    use crate::ContractError;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{
        from_binary, to_binary, Addr, BankMsg, Coin, DepsMut, Response, SubMsg, Uint64,
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
    const USER_1: &str = "user1";
    const USER_2: &str = "user2";

    fn init_contract(deps: DepsMut<DesmosQuery>) -> Result<Response<DesmosMsg>, ContractError> {
        let info = mock_info(ADMIN, &[]);
        let env = mock_env();
        instantiate(deps, env, info, InstantiateMsg {})
    }

    #[test]
    fn instantiate_properly() {
        let mut deps = mock_desmos_dependencies();
        init_contract(deps.as_mut()).unwrap();
    }

    #[test]
    fn tip_with_empty_application_error() {
        let mut deps = mock_desmos_dependencies();
        let env = mock_env();
        let info = mock_info(USER_1, &[Coin::new(10000, "udsm")]);

        init_contract(deps.as_mut()).unwrap();

        let error = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "".to_string(),
                handler: "user".to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::InvalidApplication {}, error);
    }

    #[test]
    fn tip_with_empty_handle_error() {
        let mut deps = mock_desmos_dependencies();
        let env = mock_env();
        let info = mock_info(USER_1, &[Coin::new(10000, "udsm")]);

        init_contract(deps.as_mut()).unwrap();

        let error = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handler: "".to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::InvalidUserHandler {}, error);
    }

    #[test]
    fn tip_with_empty_funds_error() {
        let mut deps = mock_desmos_dependencies();
        let env = mock_env();
        let info = mock_info(USER_1, &[]);

        init_contract(deps.as_mut()).unwrap();

        let error = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handler: "handle".to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::EmptyTipAmount {}, error);
    }

    #[test]
    fn tip_user_linked_to_multiple_addresses_error() {
        let querier = MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
            match profiler_query {
                ProfilesQuery::ApplicationLinkOwners { .. } => {
                    let response = QueryApplicationLinkOwnersResponse {
                        owners: vec![
                            ApplicationLinkOwnerDetails {
                                user: Addr::unchecked(USER_1),
                                application: "mocked_app".to_string(),
                                username: "mocked_username".to_string(),
                            },
                            ApplicationLinkOwnerDetails {
                                user: Addr::unchecked(USER_2),
                                application: "mocked_app".to_string(),
                                username: "mocked_username".to_string(),
                            },
                        ],
                        pagination: None,
                    };
                    to_binary(&response).into()
                }
                _ => mock_profiles_query_response(profiler_query),
            }
        });

        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);
        let env = mock_env();
        let info = mock_info(USER_1, &[Coin::new(10000, "udsm")]);

        init_contract(deps.as_mut()).unwrap();

        let error = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handler: "handle".to_string(),
            },
        )
        .unwrap_err();

        assert_eq!(ContractError::ToManyOwners {}, error);
    }

    #[test]
    fn tip_collected_properly() {
        let querier = MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
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
        });

        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);
        let env = mock_env();
        let info = mock_info(USER_1, &[Coin::new(10000, "udsm")]);

        init_contract(deps.as_mut()).unwrap();

        execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handler: "handle".to_string(),
            },
        )
        .unwrap();

        let pending_tips = PENDING_TIPS
            .load(
                &deps.storage,
                ("application".to_string(), "handle".to_string()),
            )
            .unwrap();

        assert_eq!(
            vec![PendingTip {
                sender: Addr::unchecked(USER_1),
                amount: vec![Coin::new(10000, "udsm")],
                block_height: 12345
            }],
            pending_tips
        )
    }

    #[test]
    fn tip_sent_properly() {
        let querier = MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
            match profiler_query {
                ProfilesQuery::ApplicationLinkOwners { .. } => {
                    let response = QueryApplicationLinkOwnersResponse {
                        owners: vec![ApplicationLinkOwnerDetails {
                            user: Addr::unchecked(USER_2),
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
        let info = mock_info(USER_1, &[Coin::new(10000, "udsm")]);

        init_contract(deps.as_mut()).unwrap();

        let response = execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handler: "handle".to_string(),
            },
        )
        .unwrap();

        assert_eq!(
            &SubMsg::<DesmosMsg>::new(BankMsg::Send {
                to_address: USER_2.to_string(),
                amount: vec![Coin::new(10000, "udsm")],
            }),
            response.messages.first().unwrap()
        )
    }

    #[test]
    fn claim_no_pending_tips_error() {
        let mut deps = mock_desmos_dependencies();
        let env = mock_env();
        let info = mock_info(USER_2, &[]);

        init_contract(deps.as_mut()).unwrap();

        let error = execute(deps.as_mut(), env, info, ExecuteMsg::ClaimTips {}).unwrap_err();

        assert_eq!(
            error,
            ContractError::NoTipsAvailable {
                user: USER_2.to_string()
            }
        )
    }

    #[test]
    fn claim_pending_tip_properly() {
        let querier = MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
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
        });
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);
        let env = mock_env();
        let info = mock_info(USER_1, &[Coin::new(10000, "udsm")]);

        init_contract(deps.as_mut()).unwrap();

        execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handler: "handler".to_string(),
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
                                user: Addr::unchecked(USER_2),
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
        let info = mock_info(USER_2, &[Coin::new(10000, "udsm")]);

        let response = execute(deps.as_mut(), env, info, ExecuteMsg::ClaimTips {}).unwrap();
        assert_eq!(
            &SubMsg::<DesmosMsg>::new(BankMsg::Send {
                to_address: USER_2.to_string(),
                amount: vec![Coin::new(10000, "udsm")],
            }),
            response.messages.first().unwrap()
        );

        // Ensure that the claimed tips have been deleted from the contract state
        let pending_tips = PENDING_TIPS
            .may_load(
                &deps.storage,
                ("application".to_string(), "handle".to_string()),
            )
            .unwrap();
        assert_eq!(None, pending_tips);
    }

    #[test]
    fn query_tips_properly() {
        let querier = MockDesmosQuerier::default().with_custom_profiles_handler(|profiler_query| {
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
        });
        let mut deps = mock_desmos_dependencies_with_custom_querier(querier);
        let env = mock_env();
        let info = mock_info(USER_1, &[Coin::new(10000, "udsm")]);

        init_contract(deps.as_mut()).unwrap();

        execute(
            deps.as_mut(),
            env,
            info,
            ExecuteMsg::SendTip {
                application: "application".to_string(),
                handler: "handler".to_string(),
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
                                user: Addr::unchecked(USER_2),
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
                user: USER_2.to_string(),
            },
        )
        .unwrap();

        let response: QueryPendingTipsResponse = from_binary(&response).unwrap();
        assert_eq!(
            response.tips,
            vec![PendingTip {
                sender: Addr::unchecked(USER_1),
                amount: vec![Coin::new(10000, "udsm")],
                block_height: 12345
            }]
        )
    }
}
