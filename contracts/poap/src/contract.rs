use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryEventInfoResponse,
    QueryMintedAmountResponse, QueryMsg,
};
use crate::state::{
    Config, EventInfo, TokenExtInfo, CONFIG, CW721_ADDRESS, EVENT_INFO, MINTER_ADDRESS,
    NEXT_POAP_ID,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_execute, wasm_instantiate, Addr, Binary, Deps, DepsMut, Empty, Env,
    MessageInfo, Reply, Response, StdResult, SubMsg, Timestamp,
};
use cw2::set_contract_version;
use cw721_base::{
    msg::ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg, MintMsg,
};
use cw_utils::parse_reply_instantiate_data;
use desmos_bindings::{msg::DesmosMsg, query::DesmosQuery};
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:poap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
// actions consts
const ACTION_ENABLE_MINT: &str = "enable_mint";
const ACTION_DISABLE_MINT: &str = "disable_mint";
const ACTION_MINT: &str = "mint";
const ACTION_MINT_TO: &str = "mint_to";
const ACTION_UPDATE_EVENT_INFO: &str = "update_event_info";
const ACTION_UPDATE_ADMIN: &str = "update_admin";
const ACTION_UPDATE_MINTER: &str = "update_minter";
// response attributes
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_SENDER: &str = "sender";
const ATTRIBUTE_CREATOR: &str = "creator";

const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    msg.validate()?;

    // Validate the admin address
    let admin = deps.api.addr_validate(&msg.admin)?;

    // Validate the minter address
    let minter = deps.api.addr_validate(&msg.minter)?;

    // Validate the creator address
    let creator = deps.api.addr_validate(&msg.event_info.creator)?;

    // Check that the end time is in the future
    if !msg.event_info.end_time.gt(&env.block.time) {
        return Err(ContractError::EndTimeBeforeCurrentTime {
            current_time: env.block.time,
            end_time: msg.event_info.end_time,
        });
    }

    // Check that the start time is in the future
    if !msg.event_info.start_time.gt(&env.block.time) {
        return Err(ContractError::StartTimeBeforeCurrentTime {
            current_time: env.block.time,
            start_time: msg.event_info.start_time,
        });
    }

    let config = Config {
        admin: admin.clone(),
        minter: minter.clone(),
        per_address_limit: msg.event_info.per_address_limit,
        cw721_code_id: msg.cw721_code_id.u64(),
        mint_enabled: false,
    };
    // Save the received event info.
    CONFIG.save(deps.storage, &config)?;

    let event_info = EventInfo {
        creator: creator.clone(),
        start_time: msg.event_info.start_time,
        end_time: msg.event_info.end_time,
        poap_uri: msg.event_info.poap_uri.clone(),
    };
    // Save the event info
    EVENT_INFO.save(deps.storage, &event_info)?;

    // Submessage to instantiate cw721 contract
    let cw721_submessage = SubMsg::reply_on_success(
        wasm_instantiate(
            msg.cw721_code_id.into(),
            &Cw721InstantiateMsg {
                name: msg.cw721_initiate_msg.name,
                symbol: msg.cw721_initiate_msg.symbol,
                minter: env.contract.address.to_string(),
            },
            info.funds,
            "poap_cw721".to_string(),
        )?,
        INSTANTIATE_CW721_REPLY_ID,
    );

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, "instantiate")
        .add_attribute(ATTRIBUTE_CREATOR, creator)
        .add_attribute("admin", admin)
        .add_attribute("minter", minter)
        .add_attribute("start_time", msg.event_info.start_time.to_string())
        .add_attribute("end_time", msg.event_info.end_time.to_string())
        .add_attribute(
            "per_address_limit",
            msg.event_info.per_address_limit.to_string(),
        )
        .add_attribute("poap_uri", &msg.event_info.poap_uri)
        .add_attribute("cw721_code_id", &msg.cw721_code_id.to_string())
        .add_submessage(cw721_submessage))
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
        ExecuteMsg::EnableMint {} => execute_set_mint_enabled(deps, info, true),
        ExecuteMsg::DisableMint {} => execute_set_mint_enabled(deps, info, false),
        ExecuteMsg::Mint {} => {
            let recipient_addr = info.sender.clone();
            execute_mint(deps, env, info, ACTION_MINT, recipient_addr, false, false)
        }
        ExecuteMsg::MintTo { recipient } => {
            let recipient_addr = deps.api.addr_validate(&recipient)?;
            execute_mint(deps, env, info, ACTION_MINT_TO, recipient_addr, true, true)
        }
        ExecuteMsg::UpdateEventInfo {
            start_time,
            end_time,
        } => execute_update_event_info(deps, env, info, start_time, end_time),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
        ExecuteMsg::UpdateMinter { new_minter } => execute_update_minter(deps, info, new_minter),
    }
}

fn execute_set_mint_enabled(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    mint_enabled: bool,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Check that the sender is the admin
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    config.mint_enabled = mint_enabled;
    // Save the new configurations
    CONFIG.save(deps.storage, &config)?;

    let action = if mint_enabled {
        ACTION_ENABLE_MINT
    } else {
        ACTION_DISABLE_MINT
    };

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, action)
        .add_attribute(ATTRIBUTE_SENDER, info.sender))
}

fn execute_mint(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    action: &str,
    recipient_addr: Addr,
    bypass_mint_enable: bool,
    check_authorized_to_mint: bool,
) -> Result<Response<DesmosMsg>, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let event_info = EVENT_INFO.load(deps.storage)?;

    // Check if the event is started
    if !event_info.is_started(&env.block.time) {
        return Err(ContractError::EventNotStarted {
            current_time: env.block.time,
            start_time: event_info.start_time,
        });
    }

    // Check if the event is ended
    if event_info.is_ended(&env.block.time) {
        return Err(ContractError::EventTerminated {
            current_time: env.block.time,
            end_time: event_info.end_time,
        });
    }

    // Check if the mint is enabled
    if !bypass_mint_enable && !config.mint_enabled {
        return Err(ContractError::MintDisabled {});
    }

    // Check if who is performing the action is the minter
    if check_authorized_to_mint && info.sender != config.minter && info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    // Check per address limit
    let recipient_mint_count = (MINTER_ADDRESS
        .key(recipient_addr.clone())
        .may_load(deps.storage)?)
    .unwrap_or(0);

    if recipient_mint_count >= config.per_address_limit {
        return Err(ContractError::MaxPerAddressLimitExceeded {
            recipient_addr: recipient_addr.to_string(),
        });
    }

    // Get the next poap id
    let poap_id = NEXT_POAP_ID.may_load(deps.storage)?.unwrap_or(1);

    // Create the cw721 message to send to mint the poap
    let mint_msg = Cw721ExecuteMsg::<TokenExtInfo, Empty>::Mint(MintMsg::<TokenExtInfo> {
        token_id: poap_id.to_string(),
        owner: recipient_addr.to_string(),
        token_uri: Some(event_info.poap_uri),
        extension: TokenExtInfo {
            claimer: recipient_addr.clone(),
        },
    });

    let cw721_address = CW721_ADDRESS.load(deps.storage)?;
    let wasm_execute_mint_msg = wasm_execute(cw721_address, &mint_msg, vec![])?;

    // Update the next poap id state
    let new_poap_id = poap_id + 1;
    NEXT_POAP_ID.save(deps.storage, &new_poap_id)?;
    // Save the new mint count for the sender's address
    let new_recipient_mint_count = recipient_mint_count + 1;
    MINTER_ADDRESS.save(
        deps.storage,
        recipient_addr.clone(),
        &new_recipient_mint_count,
    )?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, action)
        .add_attribute(ATTRIBUTE_SENDER, info.sender)
        .add_attribute("recipient", recipient_addr.to_string())
        .add_attribute("poap_id", poap_id.to_string())
        .add_message(wasm_execute_mint_msg))
}

fn execute_update_event_info(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    start_time: Timestamp,
    end_time: Timestamp,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut event_info = EVENT_INFO.load(deps.storage)?;

    // Check that is the event creator that is changing the event time frame
    if event_info.creator != info.sender {
        return Err(ContractError::Unauthorized {});
    }

    // Check that the event is not ended
    if event_info.is_ended(&env.block.time) {
        return Err(ContractError::EventTerminated {
            current_time: env.block.time,
            end_time: event_info.end_time,
        });
    }

    // Check that the event is not started
    if event_info.is_started(&env.block.time) {
        return Err(ContractError::EventStarted {
            current_time: env.block.time,
            start_time: event_info.start_time,
        });
    }

    // Check that the start time is not already passed
    if env.block.time.ge(&start_time) {
        return Err(ContractError::StartTimeBeforeCurrentTime {
            current_time: env.block.time,
            start_time,
        });
    }

    // Update the event info
    event_info.start_time = start_time;
    event_info.end_time = end_time;
    EVENT_INFO.save(deps.storage, &event_info)?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_EVENT_INFO)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute("new_start_time", start_time.to_string())
        .add_attribute("new_end_time", end_time.to_string()))
}

fn execute_update_admin(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    admin_address: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Check that the sender is the admin
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    // Update the admin address.
    let new_admin = deps.api.addr_validate(&admin_address)?;
    config.admin = new_admin;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_ADMIN)
        .add_attribute("new_admin", &admin_address))
}

fn execute_update_minter(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    minter_address: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut config = CONFIG.load(deps.storage)?;

    // Check that the sender is the admin
    if info.sender != config.admin {
        return Err(ContractError::Unauthorized {});
    }

    // Update the minter address.
    let new_minter = deps.api.addr_validate(&minter_address)?;
    config.minter = new_minter;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_MINTER)
        .add_attribute("new_minter", &minter_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::EventInfo {} => to_binary(&query_event_info(deps)?),
        QueryMsg::MintedAmount { user } => to_binary(&query_minted_amount(deps, user)?),
    }
}

fn query_config(deps: Deps<DesmosQuery>) -> StdResult<QueryConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let cw721_address = CW721_ADDRESS.load(deps.storage)?;

    Ok(QueryConfigResponse {
        admin: config.admin,
        minter: config.minter,
        mint_enabled: config.mint_enabled,
        per_address_limit: config.per_address_limit,
        cw721_contract_code: config.cw721_code_id.into(),
        cw721_contract: cw721_address,
    })
}

fn query_event_info(deps: Deps<DesmosQuery>) -> StdResult<QueryEventInfoResponse> {
    let event_info = EVENT_INFO.load(deps.storage)?;

    Ok(QueryEventInfoResponse {
        creator: event_info.creator,
        start_time: event_info.start_time,
        end_time: event_info.end_time,
        poap_uri: event_info.poap_uri,
    })
}

fn query_minted_amount(
    deps: Deps<DesmosQuery>,
    user: String,
) -> StdResult<QueryMintedAmountResponse> {
    let user_addr = deps.api.addr_validate(&user)?;

    let minted_amount = MINTER_ADDRESS
        .may_load(deps.storage, user_addr.clone())?
        .unwrap_or(0);

    Ok(QueryMintedAmountResponse {
        user: user_addr,
        amount: minted_amount,
    })
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
            Ok(Response::default().add_attribute(ATTRIBUTE_ACTION, "instantiate_cw721_reply"))
        }
        Err(_) => Err(ContractError::InstantiateCw721Error {}),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::{
        get_valid_init_msg, EVENT_END_SECONDS, EVENT_START_SECONDS, INITIAL_BLOCK_TIME_SECONDS,
    };
    use crate::ContractError::Unauthorized;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{DepsMut, Timestamp};
    use desmos_bindings::mocks::mock_queriers::mock_dependencies_with_custom_querier;

    const CREATOR: &str = "creator";
    const ADMIN: &str = "admin";
    const MINTER: &str = "minter";
    const USER: &str = "user";
    const FAKE_CW721_ADDRESS: &str = "cw721-contract";

    fn do_instantiate(deps: DepsMut<DesmosQuery>) {
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        // Change block time to event start.
        env.block.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS);

        // Since replay is not called, fake the stored cw721 contract address.
        CW721_ADDRESS
            .save(deps.storage, &Addr::unchecked(FAKE_CW721_ADDRESS))
            .unwrap();

        let msg = get_valid_init_msg(1);
        assert!(instantiate(deps, env, info, msg).is_ok());
    }

    #[test]
    fn instantiate_with_invalid_admin_addr_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        let mut init_msg = get_valid_init_msg(1);
        init_msg.admin = "a".to_string();

        let result = instantiate(deps.as_mut(), env, info, init_msg);
        assert!(result.is_err())
    }

    #[test]
    fn instantiate_with_invalid_minter_addr_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        let mut init_msg = get_valid_init_msg(1);
        init_msg.minter = "a".to_string();

        let result = instantiate(deps.as_mut(), env, info, init_msg);
        assert!(result.is_err())
    }

    #[test]
    fn instantiate_with_invalid_creator_addr_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        let mut init_msg = get_valid_init_msg(1);
        init_msg.event_info.creator = "a".to_string();

        let result = instantiate(deps.as_mut(), env, info, init_msg);
        assert!(result.is_err())
    }

    #[test]
    fn instantiate_with_event_start_before_current_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        let mut init_msg = get_valid_init_msg(1);
        // Create a start time 200 seconds before the current block time
        let start = &env.block.time.seconds() - 200;
        init_msg.event_info.start_time = Timestamp::from_seconds(start);
        init_msg.event_info.end_time = Timestamp::from_seconds(start + 600);

        let init_result = instantiate(deps.as_mut(), env.clone(), info, init_msg);
        assert_eq!(
            ContractError::StartTimeBeforeCurrentTime {
                current_time: env.block.time,
                start_time: Timestamp::from_seconds(start)
            },
            init_result.unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_event_start_equal_current_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);
        let mut init_msg = get_valid_init_msg(1);

        let start = env.block.time.nanos();
        init_msg.event_info.start_time = Timestamp::from_nanos(start);
        init_msg.event_info.end_time = Timestamp::from_nanos(start + 600);

        let init_result = instantiate(deps.as_mut(), env, info, init_msg);
        assert_eq!(
            ContractError::StartTimeBeforeCurrentTime {
                current_time: Timestamp::from_nanos(start),
                start_time: Timestamp::from_nanos(start)
            },
            init_result.unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_event_end_before_current_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);
        let mut init_msg = get_valid_init_msg(1);

        // Create a start time 200 seconds before the current block time
        let start = env.block.time.seconds() - 200;
        let end = env.block.time.seconds() - 100;
        init_msg.event_info.start_time = Timestamp::from_seconds(start);
        // Start time 100 seconds before the current block time
        init_msg.event_info.end_time = Timestamp::from_seconds(end);

        let init_result = instantiate(deps.as_mut(), env.clone(), info, init_msg);
        assert_eq!(
            ContractError::EndTimeBeforeCurrentTime {
                current_time: env.block.time,
                end_time: Timestamp::from_seconds(end)
            },
            init_result.unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_event_end_equal_current_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);
        let mut init_msg = get_valid_init_msg(1);

        let start = env.block.time.seconds() - 200;
        let end = env.block.time.nanos();
        init_msg.event_info.start_time = Timestamp::from_seconds(start);
        init_msg.event_info.end_time = Timestamp::from_nanos(end);

        let init_result = instantiate(deps.as_mut(), env, info, init_msg);
        assert_eq!(
            ContractError::EndTimeBeforeCurrentTime {
                current_time: Timestamp::from_nanos(end),
                end_time: Timestamp::from_nanos(end),
            },
            init_result.unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_event_start_after_end_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);
        let mut init_msg = get_valid_init_msg(1);

        // Create a start time 200 seconds after the current block time
        let start = env.block.time.seconds() + 200;
        let end = env.block.time.seconds() + 100;
        init_msg.event_info.start_time = Timestamp::from_seconds(start);
        // Start time 100 seconds before the event start time
        init_msg.event_info.end_time = Timestamp::from_seconds(end);

        let init_result = instantiate(deps.as_mut(), env, info, init_msg);
        assert_eq!(
            ContractError::StartTimeAfterEndTime {
                start: Timestamp::from_seconds(start),
                end: Timestamp::from_seconds(end),
            },
            init_result.unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_event_start_equal_end_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);
        let mut init_msg = get_valid_init_msg(1);

        // Create a start time 200 seconds after the current block time
        let start = env.block.time.seconds() + 200;
        init_msg.event_info.start_time = Timestamp::from_seconds(start);
        init_msg.event_info.end_time = Timestamp::from_seconds(start);

        let init_result = instantiate(deps.as_mut(), env, info, init_msg);
        assert_eq!(
            ContractError::StartTimeAfterEndTime {
                start: Timestamp::from_seconds(start),
                end: Timestamp::from_seconds(start),
            },
            init_result.unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_invalid_poap_uri_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);
        let mut init_msg = get_valid_init_msg(1);

        // Invalid uri
        init_msg.event_info.poap_uri = "invalid_uri".to_string();

        let init_result = instantiate(deps.as_mut(), env, info, init_msg);
        assert!(init_result.is_err());
    }

    #[test]
    fn instantiate_with_non_ipfs_poap_uri_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);
        let mut init_msg = get_valid_init_msg(1);

        init_msg.event_info.poap_uri = "https://random_domain.com".to_string();

        let init_result = instantiate(deps.as_mut(), env, info, init_msg);
        assert_eq!(ContractError::InvalidPoapUri {}, init_result.unwrap_err());
    }

    #[test]
    fn enable_mint_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::EnableMint {};
        execute(deps.as_mut(), env, info, msg).unwrap();

        let config = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(true, config.mint_enabled);
    }

    #[test]
    fn enable_mint_without_permission_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(USER, &vec![]);

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::EnableMint {};
        let execute_result = execute(deps.as_mut(), env, info, msg);
        assert_eq!(Unauthorized {}, execute_result.unwrap_err());
    }

    #[test]
    fn disable_mint_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::DisableMint {};
        execute(deps.as_mut(), env, info, msg).unwrap();

        let config = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(false, config.mint_enabled);
    }

    #[test]
    fn normal_user_can_not_disable_mint_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(USER, &vec![]);

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::DisableMint {};
        let execute_result = execute(deps.as_mut(), env, info, msg);
        assert_eq!(Unauthorized {}, execute_result.unwrap_err());
    }

    #[test]
    fn creator_change_event_info_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);
        let new_start_time = Timestamp::from_seconds(env.block.time.seconds() + 100);
        let new_end_time = Timestamp::from_seconds(env.block.time.seconds() + 400);

        do_instantiate(deps.as_mut());

        env.block.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS);

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: new_start_time.clone(),
            end_time: new_end_time.clone(),
        };

        execute(deps.as_mut(), env, info, msg).unwrap();

        let event_info = EVENT_INFO.load(&deps.storage).unwrap();
        assert_eq!(new_start_time, event_info.start_time);
        assert_eq!(new_end_time, event_info.end_time)
    }

    #[test]
    fn non_creator_change_event_info_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        let info = mock_info(USER, &vec![]);
        let new_start_time = Timestamp::from_seconds(env.block.time.seconds() + 100);
        let new_end_time = Timestamp::from_seconds(env.block.time.seconds() + 400);
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: new_start_time.clone(),
            end_time: new_end_time.clone(),
        };

        do_instantiate(deps.as_mut());

        let result = execute(deps.as_mut(), env, info, msg.clone());
        // User should not be authorized to update the event info
        assert_eq!(Unauthorized {}, result.unwrap_err());

        let env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        let result = execute(deps.as_mut(), env, info, msg);
        // Admin should not be authorized to update the event info
        assert_eq!(Unauthorized {}, result.unwrap_err());
    }

    #[test]
    fn event_info_update_after_event_started_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(EVENT_START_SECONDS),
            end_time: Timestamp::from_seconds(EVENT_END_SECONDS),
        };

        // Fake current time to event in progress
        env.block.time = Timestamp::from_seconds(EVENT_START_SECONDS + 100);

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
        assert_eq!(
            ContractError::EventStarted {
                start_time: Timestamp::from_seconds(EVENT_START_SECONDS),
                current_time: env.block.time.clone(),
            },
            result.unwrap_err()
        );

        // Edge case current time is event start
        env.block.time = Timestamp::from_seconds(EVENT_START_SECONDS);

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
        assert_eq!(
            ContractError::EventStarted {
                start_time: Timestamp::from_seconds(EVENT_START_SECONDS),
                current_time: env.block.time.clone(),
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn event_info_update_after_event_terminated_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(EVENT_START_SECONDS),
            // Add 300 seconds to prevent end time to be already passed
            end_time: Timestamp::from_seconds(EVENT_END_SECONDS + 300),
        };

        // Fake current time to event ended
        env.block.time = Timestamp::from_seconds(EVENT_END_SECONDS + 100);

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
        assert_eq!(
            ContractError::EventTerminated {
                end_time: Timestamp::from_seconds(EVENT_END_SECONDS),
                current_time: env.block.time.clone(),
            },
            result.unwrap_err()
        );

        // Edge case current time is event end
        env.block.time = Timestamp::from_seconds(EVENT_END_SECONDS);

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
        assert_eq!(
            ContractError::EventTerminated {
                end_time: Timestamp::from_seconds(EVENT_END_SECONDS),
                current_time: env.block.time.clone(),
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn event_info_update_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(EVENT_START_SECONDS),
            end_time: Timestamp::from_seconds(EVENT_END_SECONDS + 300),
        };

        // Current time is before event started
        env.block.time = Timestamp::from_seconds(EVENT_START_SECONDS - 100);

        execute(deps.as_mut(), env, info, msg).unwrap();
    }

    #[test]
    fn event_info_start_time_equal_end_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        do_instantiate(deps.as_mut());
        env.block.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS);

        // Start time eq end time
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(EVENT_START_SECONDS),
            end_time: Timestamp::from_seconds(EVENT_START_SECONDS),
        };

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        assert_eq!(
            ContractError::StartTimeAfterEndTime {
                start: Timestamp::from_seconds(EVENT_START_SECONDS),
                end: Timestamp::from_seconds(EVENT_START_SECONDS)
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn event_info_start_time_after_end_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        do_instantiate(deps.as_mut());
        env.block.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS);

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(EVENT_START_SECONDS + 100),
            end_time: Timestamp::from_seconds(EVENT_START_SECONDS),
        };

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        assert_eq!(
            ContractError::StartTimeAfterEndTime {
                start: Timestamp::from_seconds(EVENT_START_SECONDS + 100),
                end: Timestamp::from_seconds(EVENT_START_SECONDS)
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn event_info_start_time_before_current_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        do_instantiate(deps.as_mut());
        env.block.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS);

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS - 1),
            end_time: Timestamp::from_seconds(EVENT_END_SECONDS),
        };

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        assert_eq!(
            ContractError::StartTimeBeforeCurrentTime {
                start_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS - 1),
                current_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS),
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn event_info_start_time_equal_current_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        do_instantiate(deps.as_mut());
        env.block.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS);

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS),
            end_time: Timestamp::from_seconds(EVENT_END_SECONDS),
        };

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        assert_eq!(
            ContractError::StartTimeBeforeCurrentTime {
                start_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS),
                current_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS),
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn event_info_end_time_before_current_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        do_instantiate(deps.as_mut());
        env.block.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS);

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS + 2),
            end_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS - 1),
        };

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        assert_eq!(
            ContractError::StartTimeAfterEndTime {
                start: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS + 2),
                end: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS - 1),
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn event_info_end_time_equal_current_time_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(CREATOR, &vec![]);

        do_instantiate(deps.as_mut());
        env.block.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS);

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS + 2),
            end_time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS),
        };

        let result = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        assert_eq!(
            ContractError::StartTimeAfterEndTime {
                start: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS + 2),
                end: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS),
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn update_admin_without_permission_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        const NEW_ADMIN: &str = "admin2";

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::UpdateAdmin {
            new_admin: NEW_ADMIN.to_string(),
        };

        let result = execute(
            deps.as_mut(),
            env.clone(),
            mock_info(USER, &vec![]),
            msg.clone(),
        );
        assert_eq!(ContractError::Unauthorized {}, result.unwrap_err());

        let result = execute(
            deps.as_mut(),
            env.clone(),
            mock_info(CREATOR, &vec![]),
            msg.clone(),
        );
        assert_eq!(ContractError::Unauthorized {}, result.unwrap_err());
    }

    #[test]
    fn update_admin_with_permission_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        const NEW_ADMIN: &str = "admin2";

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::UpdateAdmin {
            new_admin: NEW_ADMIN.to_string(),
        };

        execute(deps.as_mut(), env.clone(), mock_info(ADMIN, &vec![]), msg).unwrap();

        let config = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(NEW_ADMIN, config.admin.as_str());
    }

    #[test]
    fn update_minter_permission_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        const NEW_MINTER: &str = "minter2";

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::UpdateMinter {
            new_minter: NEW_MINTER.to_string(),
        };

        let result = execute(
            deps.as_mut(),
            env.clone(),
            mock_info(USER, &vec![]),
            msg.clone(),
        );
        assert_eq!(ContractError::Unauthorized {}, result.unwrap_err());

        let result = execute(
            deps.as_mut(),
            env.clone(),
            mock_info(CREATOR, &vec![]),
            msg.clone(),
        );
        assert_eq!(ContractError::Unauthorized {}, result.unwrap_err());

        let result = execute(deps.as_mut(), env.clone(), mock_info(MINTER, &vec![]), msg);
        assert_eq!(ContractError::Unauthorized {}, result.unwrap_err());
    }

    #[test]
    fn update_minter_with_permission_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let env = mock_env();
        const NEW_MINTER: &str = "minter2";

        do_instantiate(deps.as_mut());

        let msg = ExecuteMsg::UpdateMinter {
            new_minter: NEW_MINTER.to_string(),
        };

        execute(deps.as_mut(), env.clone(), mock_info(ADMIN, &vec![]), msg).unwrap();

        let config = CONFIG.load(&deps.storage).unwrap();
        assert_eq!(NEW_MINTER, config.minter.as_str());
    }

    #[test]
    fn mint_with_event_not_started_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        do_instantiate(deps.as_mut());

        env.block.time = Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS);

        // Enable mint since is disable by default.
        let msg = ExecuteMsg::EnableMint {};
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        let msg = ExecuteMsg::Mint {};
        let info = mock_info(USER, &vec![]);
        let result = execute(deps.as_mut(), env.clone(), info, msg);

        // Event is not started
        assert_eq!(
            ContractError::EventNotStarted {
                current_time: env.block.time,
                start_time: Timestamp::from_seconds(EVENT_START_SECONDS),
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn mint_with_event_terminated_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        do_instantiate(deps.as_mut());

        // Enable mint since is disable by default.
        let msg = ExecuteMsg::EnableMint {};
        execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        env.block.time = Timestamp::from_seconds(EVENT_END_SECONDS);

        let msg = ExecuteMsg::Mint {};
        let info = mock_info(USER, &vec![]);
        let result = execute(deps.as_mut(), env.clone(), info, msg);

        // Event is not started
        assert_eq!(
            ContractError::EventTerminated {
                current_time: env.block.time,
                end_time: Timestamp::from_seconds(EVENT_END_SECONDS),
            },
            result.unwrap_err()
        );
    }

    #[test]
    fn mint_without_permissions_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(USER, &vec![]);

        do_instantiate(deps.as_mut());

        // Change current time to event start
        env.block.time = Timestamp::from_seconds(EVENT_START_SECONDS);

        let msg = ExecuteMsg::Mint {};
        let result = execute(deps.as_mut(), env.clone(), info, msg);

        // Event is not started
        assert_eq!(ContractError::MintDisabled {}, result.unwrap_err());
    }

    #[test]
    fn mint_out_of_max_amount_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        do_instantiate(deps.as_mut());

        // Change current time to event start
        env.block.time = Timestamp::from_seconds(EVENT_START_SECONDS);

        execute(deps.as_mut(), env.clone(), info, ExecuteMsg::EnableMint {}).unwrap();

        let info = mock_info(USER, &vec![]);
        // Mint the first poap
        execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::Mint {},
        )
        .unwrap();
        // Mint the second poap
        execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::Mint {},
        )
        .unwrap();

        let response = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::Mint {},
        );
        assert_eq!(
            ContractError::MaxPerAddressLimitExceeded {
                recipient_addr: USER.to_string()
            },
            response.unwrap_err()
        );

        // Ensure that mint to also fails when minting for the user
        let info = mock_info(ADMIN, &vec![]);
        let response = execute(
            deps.as_mut(),
            env.clone(),
            info,
            ExecuteMsg::MintTo {
                recipient: USER.to_string(),
            },
        );

        assert_eq!(
            ContractError::MaxPerAddressLimitExceeded {
                recipient_addr: USER.to_string()
            },
            response.unwrap_err()
        );
    }

    #[test]
    fn mint_to_out_of_max_amount_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();
        let info = mock_info(ADMIN, &vec![]);

        do_instantiate(deps.as_mut());

        // Change current time to event start
        env.block.time = Timestamp::from_seconds(EVENT_START_SECONDS);

        execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::EnableMint {},
        )
        .unwrap();

        // Mint the first poap
        execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::MintTo {
                recipient: USER.to_string(),
            },
        )
        .unwrap();
        // Mint the second and last allowed poap
        execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::MintTo {
                recipient: USER.to_string(),
            },
        )
        .unwrap();

        let response = execute(
            deps.as_mut(),
            env.clone(),
            info.clone(),
            ExecuteMsg::MintTo {
                recipient: USER.to_string(),
            },
        );
        // Should fail since the user have already received the max allowed poaps.
        assert_eq!(
            ContractError::MaxPerAddressLimitExceeded {
                recipient_addr: USER.to_string()
            },
            response.unwrap_err()
        );

        // Test also with Mint from use
        let info = mock_info(USER, &vec![]);
        let response = execute(deps.as_mut(), env.clone(), info, ExecuteMsg::Mint {});
        assert_eq!(
            ContractError::MaxPerAddressLimitExceeded {
                recipient_addr: USER.to_string()
            },
            response.unwrap_err()
        );
    }

    #[test]
    fn mint_to_without_permission_error() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();

        do_instantiate(deps.as_mut());

        // Change current time to event start
        env.block.time = Timestamp::from_seconds(EVENT_START_SECONDS);

        let response = execute(
            deps.as_mut(),
            env,
            mock_info(USER, &vec![]),
            ExecuteMsg::MintTo {
                recipient: USER.to_string(),
            },
        );
        // User should not be authorized to use the mint to action
        assert_eq!(ContractError::Unauthorized {}, response.unwrap_err());
    }

    #[test]
    fn mint_to_from_minter_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();

        do_instantiate(deps.as_mut());

        // Change current time to event start
        env.block.time = Timestamp::from_seconds(EVENT_START_SECONDS);

        // Test that minter can call mint to
        execute(
            deps.as_mut(),
            env.clone(),
            mock_info(MINTER, &vec![]),
            ExecuteMsg::MintTo {
                recipient: USER.to_string(),
            },
        )
        .unwrap();
    }

    #[test]
    fn mint_to_from_admin_properly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let mut env = mock_env();

        do_instantiate(deps.as_mut());

        // Change current time to event start
        env.block.time = Timestamp::from_seconds(EVENT_START_SECONDS);

        // Test that minter can call mint to
        execute(
            deps.as_mut(),
            env.clone(),
            mock_info(ADMIN, &vec![]),
            ExecuteMsg::MintTo {
                recipient: USER.to_string(),
            },
        )
        .unwrap();
    }
}
