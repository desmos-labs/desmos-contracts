use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryEventInfoResponse,
    QueryMintedAmountResponse, QueryMsg,
};
use crate::state::{
    Config, EventInfo, CONFIG, CW721_ADDRESS, EVENT_INFO, MINTER_ADDRESS, NEXT_POAP_ID,
};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_execute, wasm_instantiate, Addr, Binary, Deps, DepsMut, Env, MessageInfo,
    Reply, Response, StdResult, SubMsg, Timestamp,
};
use cw2::set_contract_version;
use cw721_base::{
    msg::ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg, MintMsg,
};
use cw_utils::parse_reply_instantiate_data;
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    msg.validate()?;

    // Validate the admin address
    let admin = deps.api.addr_validate(&msg.admin)?;

    // Validate the minter address
    let minter = deps.api.addr_validate(&msg.minter)?;

    // Validate the creator address
    let creator = deps.api.addr_validate(&msg.event_info.creator)?;

    // Check that the start time is in the future
    if !msg.event_info.start_time.gt(&env.block.time) {
        return Err(ContractError::StartTimeBeforeCurrentTime {
            current_time: env.block.time,
            start_time: msg.event_info.start_time,
        });
    }

    // Check that the end time is in the future
    if !msg.event_info.end_time.gt(&env.block.time) {
        return Err(ContractError::EndTimeBeforeCurrentTime {
            current_time: env.block.time,
            end_time: msg.event_info.end_time,
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
        base_poap_uri: msg.event_info.base_poap_uri.clone(),
        event_uri: msg.event_info.event_uri.clone(),
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
        .add_attribute("base_poap_uri", &msg.event_info.base_poap_uri)
        .add_attribute("event_uri", &msg.event_info.event_uri)
        .add_attribute("cw721_code_id", &msg.cw721_code_id.to_string())
        .add_submessage(cw721_submessage))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
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
    deps: DepsMut,
    info: MessageInfo,
    mint_enabled: bool,
) -> Result<Response, ContractError> {
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: &str,
    recipient_addr: Addr,
    bypass_mint_enable: bool,
    check_authorized_to_mint: bool,
) -> Result<Response, ContractError> {
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
    let mint_msg = Cw721ExecuteMsg::Mint(MintMsg::<String> {
        token_id: poap_id.to_string(),
        owner: recipient_addr.to_string(),
        token_uri: Some(event_info.base_poap_uri),
        extension: event_info.event_uri,
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    start_time: Timestamp,
    end_time: Timestamp,
) -> Result<Response, ContractError> {
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

    // Check that the end time is not already passed
    if env.block.time.ge(&end_time) {
        return Err(ContractError::EndTimeBeforeCurrentTime {
            current_time: env.block.time,
            end_time,
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
    deps: DepsMut,
    info: MessageInfo,
    admin_address: String,
) -> Result<Response, ContractError> {
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
    deps: DepsMut,
    info: MessageInfo,
    minter_address: String,
) -> Result<Response, ContractError> {
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
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::EventInfo {} => to_binary(&query_event_info(deps)?),
        QueryMsg::MintedAmount { user } => to_binary(&query_minted_amount(deps, user)?),
    }
}

fn query_config(deps: Deps) -> StdResult<QueryConfigResponse> {
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

fn query_event_info(deps: Deps) -> StdResult<QueryEventInfoResponse> {
    let event_info = EVENT_INFO.load(deps.storage)?;

    Ok(QueryEventInfoResponse {
        creator: event_info.creator,
        start_time: event_info.start_time,
        end_time: event_info.end_time,
        event_uri: event_info.event_uri,
    })
}

fn query_minted_amount(deps: Deps, user: String) -> StdResult<QueryMintedAmountResponse> {
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
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> Result<Response, ContractError> {
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
