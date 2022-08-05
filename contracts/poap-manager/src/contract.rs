#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_execute, wasm_instantiate, Addr, Deps, DepsMut, Env, MessageInfo,
    QueryResponse, Reply, Response, StdResult, SubMsg,
};
use cw2::set_contract_version;
use cw_utils::parse_reply_instantiate_data;

use desmos_bindings::{msg::DesmosMsg, query::DesmosQuery, profiles::querier::ProfilesQuerier};
use poap::msg::ExecuteMsg as POAPExecuteMsg;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg};
use crate::state::{Config, CONFIG, POAP_CONTRACT_ADDRESS};

use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:poap-manager";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// actions for executing messages
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_INSTANTIATE_POAP_REPLY: &str = "instantiate_poap_reply";
const ACTION_CLAIM: &str = "claim";
const ACTION_MINT_TO: &str = "mint_to";
const ACTION_UPDATE_ADMIN: &str = "update_admin";

// attributes for executing messages
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_ADMIN: &str = "admin";
const ATTRIBUTE_POAP_CODE_ID: &str = "poap_code_id";
const ATTRIBUTE_SENDER: &str = "sender";
const ATTRIBUTE_NEW_ADMIN: &str = "new_admin";

// reply ids for handling reply tasks
const INSTANTIATE_POAP_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    msg.validate()?;
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = deps.api.addr_validate(&msg.admin)?;
    let config = Config {
        admin: admin.clone(),
        poap_code_id: msg.poap_code_id.u64(),
    };
    CONFIG.save(deps.storage, &config)?;

    // assign the minter of poap contract to the manager's contract address
    let mut poap_instantiate_msg = msg.poap_instantiate_msg;
    poap_instantiate_msg.minter = env.contract.address.into();

    Ok(Response::new()
        .add_attribute("action", ACTION_INSTANTIATE)
        .add_attribute(ATTRIBUTE_ADMIN, admin)
        .add_attribute(ATTRIBUTE_POAP_CODE_ID, msg.poap_code_id)
        .add_submessage(SubMsg::reply_on_success(
            wasm_instantiate(
                msg.poap_code_id.u64(),
                &poap_instantiate_msg,
                info.funds,
                "poap".into(),
            )?,
            INSTANTIATE_POAP_REPLY_ID,
        )))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    msg: Reply,
) -> Result<Response<DesmosMsg>, ContractError> {
    match msg.id {
        INSTANTIATE_POAP_REPLY_ID => resolve_instantiate_poap_reply(deps, msg),
        _ => Err(ContractError::InvalidReplyID {}),
    }
}

fn resolve_instantiate_poap_reply(
    deps: DepsMut<DesmosQuery>,
    msg: Reply,
) -> Result<Response<DesmosMsg>, ContractError> {
    let res = parse_reply_instantiate_data(msg)?;
    let address = deps.api.addr_validate(&res.contract_address)?;
    POAP_CONTRACT_ADDRESS.save(deps.storage, &address)?;
    Ok(Response::new().add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE_POAP_REPLY))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    match msg {
        ExecuteMsg::Claim {} => execute_claim(deps, info),
        ExecuteMsg::MintTo { recipient } => execute_mint_to(deps, info, recipient),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
    }
}

fn execute_claim(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
) -> Result<Response<DesmosMsg>, ContractError> {
    let poap_contract_address = POAP_CONTRACT_ADDRESS.load(deps.storage)?;
    if !check_eligibility(deps, info.sender.clone())? {
        return Err(ContractError::NoEligibilityError {});
    }
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_CLAIM)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_message(wasm_execute(
            poap_contract_address,
            &POAPExecuteMsg::MintTo {
                recipient: info.sender.into(),
            },
            info.funds,
        )?))
}

fn check_eligibility(deps: DepsMut<DesmosQuery>, user: Addr) -> Result<bool, ContractError> {
    ProfilesQuerier::new(deps.querier.deref()).query_profile(user)?;
    Ok(true)
}

fn execute_mint_to(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    recipient: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let poap_contract_address = POAP_CONTRACT_ADDRESS.load(deps.storage)?;
    deps.api.addr_validate(&recipient)?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_MINT_TO)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_message(wasm_execute(
            poap_contract_address,
            &POAPExecuteMsg::MintTo { recipient },
            info.funds,
        )?))
}

fn execute_update_admin(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    user: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let new_admin = deps.api.addr_validate(&user)?;
    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        if config.admin != info.sender {
            return Err(ContractError::NotAdmin {
                caller: info.sender.clone(),
            });
        }
        config.admin = new_admin.clone();
        Ok(config)
    })?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_ADMIN)
        .add_attribute(ATTRIBUTE_NEW_ADMIN, new_admin)
        .add_attribute(ATTRIBUTE_SENDER, info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> StdResult<QueryResponse> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps<DesmosQuery>) -> StdResult<QueryConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    Ok(QueryConfigResponse {
        admin: config.admin,
        poap_code_id: config.poap_code_id,
        poap_contract_address: POAP_CONTRACT_ADDRESS.load(deps.storage)?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{StdError, SubMsgResponse, SubMsgResult, Timestamp};
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use cw_utils::ParseReplyError;
    use desmos_bindings::mocks::mock_queriers::mock_dependencies_with_custom_querier;
    use poap::msg::{EventInfo, InstantiateMsg as POAPInstantiateMsg};
    use prost::Message;

    const CREATOR: &str = "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc";
    const NEW_ADMIN: &str = "desmos1fcrca0eyvj32yeqwyqgs245gjmq4ee9vjjdlnz";

    fn get_valid_instantiate() -> InstantiateMsg {
        InstantiateMsg {
            admin: "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc".into(),
            poap_code_id: 1u64.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: CREATOR.into(),
                minter: CREATOR.into(),
                cw721_code_id: 2u64.into(),
                cw721_initiate_msg: Cw721InstantiateMsg {
                    minter: CREATOR.into(),
                    name: CREATOR.into(),
                    symbol: CREATOR.into(),
                },
                event_info: EventInfo {
                    creator: CREATOR.into(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    base_poap_uri: "ipfs://popap-uri".to_string(),
                    event_uri: "ipfs://event-uri".to_string(),
                },
            },
        }
    }

    #[derive(Clone, PartialEq, Message)]
    struct MsgInstantiateContractResponse {
        #[prost(string, tag = "1")]
        pub contract_address: ::prost::alloc::string::String,
        #[prost(bytes, tag = "2")]
        pub data: ::prost::alloc::vec::Vec<u8>,
    }

    fn get_valid_instantiate_reply() -> Reply {
        let instantiate_reply = MsgInstantiateContractResponse {
            contract_address: "poap_contract_address".into(),
            data: vec![],
        };
        let mut encoded_instantiate_reply =
            Vec::<u8>::with_capacity(instantiate_reply.encoded_len());

        instantiate_reply
            .encode(&mut encoded_instantiate_reply)
            .unwrap();
        Reply {
            id: 1,
            result: SubMsgResult::Ok(SubMsgResponse {
                events: vec![],
                data: Some(encoded_instantiate_reply.into()),
            }),
        }
    }

    fn do_instantiate(deps: DepsMut<DesmosQuery>) {
        let env = mock_env();
        let info = mock_info(CREATOR, &vec![]);
        let valid_msg = get_valid_instantiate();
        instantiate(deps, env, info, valid_msg).unwrap();
    }

    #[test]
    fn instatiate_with_invalid_msg_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(CREATOR, &vec![]);
        let invalid_msg = InstantiateMsg {
            admin: "".into(),
            poap_code_id: 0u64.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: CREATOR.into(),
                minter: CREATOR.into(),
                cw721_code_id: 2u64.into(),
                cw721_initiate_msg: Cw721InstantiateMsg {
                    minter: CREATOR.into(),
                    name: CREATOR.into(),
                    symbol: CREATOR.into(),
                },
                event_info: EventInfo {
                    creator: CREATOR.into(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    base_poap_uri: "ipfs://popap-uri".to_string(),
                    event_uri: "ipfs://event-uri".to_string(),
                },
            },
        };
        assert_eq!(
            instantiate(deps.as_mut(), env, info, invalid_msg).unwrap_err(),
            ContractError::InvalidPOAPCodeID {},
        )
    }

    #[test]
    fn instatiate_with_invalid_admin_address_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(CREATOR, &vec![]);
        let invalid_msg = InstantiateMsg {
            admin: "a".into(),
            poap_code_id: 1u64.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: CREATOR.into(),
                minter: CREATOR.into(),
                cw721_code_id: 2u64.into(),
                cw721_initiate_msg: Cw721InstantiateMsg {
                    minter: CREATOR.into(),
                    name: CREATOR.into(),
                    symbol: CREATOR.into(),
                },
                event_info: EventInfo {
                    creator: CREATOR.into(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    base_poap_uri: "ipfs://popap-uri".to_string(),
                    event_uri: "ipfs://event-uri".to_string(),
                },
            },
        };
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
        let expected = Config {
            admin: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            poap_code_id: 1u64,
        };
        assert_eq!(config, expected)
    }

    #[test]
    fn poap_instantiate_with_invalid_reply_id_error() {
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
    fn poap_instantiate_with_invalid_instantiate_msg_error() {
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
        assert_eq!(
            result.unwrap_err(),
            ContractError::ParseReplyError(ParseReplyError::ParseFailure(
                "Missing reply data".into()
            ))
        )
    }

    #[test]
    fn poap_instantiate_reply_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        do_instantiate(deps.as_mut());
        let env = mock_env();
        let result = reply(deps.as_mut(), env, get_valid_instantiate_reply());
        assert!(result.is_ok());
        let address = POAP_CONTRACT_ADDRESS.load(&deps.storage).unwrap();
        let expected = Addr::unchecked("poap_contract_address");
        assert_eq!(address, expected);
    }

    #[test]
    fn claim_with_unsupported_desmos_deps_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        do_instantiate(deps.as_mut());
        let env = mock_env();
        let info = mock_info(CREATOR, &vec![]);
        let msg = ExecuteMsg::Claim {};
        assert_eq!(
            execute(deps.as_mut(), env, info, msg).unwrap_err(),
            ContractError::Std(StdError::not_found("cosmwasm_std::addresses::Addr"))
        )
    }

    #[test]
    fn claim_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&vec![]);
        do_instantiate(deps.as_mut());
        POAP_CONTRACT_ADDRESS
            .save(deps.as_mut().storage, &Addr::unchecked(""))
            .unwrap();
        let env = mock_env();
        let info = mock_info(CREATOR, &vec![]);
        let msg = ExecuteMsg::Claim {};
        execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn mint_to_with_invalid_recipient_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        do_instantiate(deps.as_mut());
        POAP_CONTRACT_ADDRESS
            .save(deps.as_mut().storage, &Addr::unchecked(""))
            .unwrap();
        let env = mock_env();
        let info = mock_info(CREATOR, &vec![]);
        let msg = ExecuteMsg::MintTo {
            recipient: "a".into(),
        };
        assert_eq!(
            execute(deps.as_mut(), env, info, msg).unwrap_err(),
            ContractError::Std(StdError::generic_err(
                "Invalid input: human address too short"
            ))
        )
    }

    #[test]
    fn mint_to_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        do_instantiate(deps.as_mut());
        POAP_CONTRACT_ADDRESS
            .save(deps.as_mut().storage, &Addr::unchecked(""))
            .unwrap();
        let env = mock_env();
        let info = mock_info(CREATOR, &vec![]);
        let msg = ExecuteMsg::MintTo {
            recipient: CREATOR.into(),
        };
        execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn update_admin_with_invalid_new_admin_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        do_instantiate(deps.as_mut());
        let env = mock_env();
        let info = mock_info(CREATOR, &vec![]);
        let msg = ExecuteMsg::UpdateAdmin {
            new_admin: "".into(),
        };
        assert_eq!(
            execute(deps.as_mut(), env, info, msg).unwrap_err(),
            ContractError::Std(StdError::generic_err(
                "Invalid input: human address too short"
            ))
        )
    }

    #[test]
    fn update_admin_without_permission_error() {
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
        let info = mock_info(CREATOR, &vec![]);
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
        let info = mock_info(CREATOR, &vec![]);
        let msg = ExecuteMsg::UpdateAdmin {
            new_admin: NEW_ADMIN.into(),
        };
        assert!(execute(deps.as_mut(), env, info, msg).is_ok());

        let config = CONFIG.load(&deps.storage).unwrap();
        let expected = Config {
            admin: Addr::unchecked(NEW_ADMIN),
            poap_code_id: 1u64,
        };
        assert_eq!(config, expected)
    }
}
