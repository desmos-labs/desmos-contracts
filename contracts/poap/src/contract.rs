use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg};
use crate::state::{Config, EventInfo, CONFIG, CW721_ADDRESS, EVENT_INFO, NEXT_POAP_ID};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, CosmosMsg, Deps, DepsMut, Empty, Env, MessageInfo, Reply, ReplyOn,
    Response, StdError, StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw721_base::{
    msg::ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg, MintMsg,
};
use cw_utils::parse_reply_instantiate_data;
use url::Url;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:poap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    // Validate the admin address
    let admin = match msg.admin {
        // Fallback to sender if the admin is not defined
        None => info.sender.clone(),
        // Admin defined, make sure that is a valid address
        Some(admin_address) => deps.api.addr_validate(&admin_address)?,
    };

    // Validate the minter address
    let minter = match msg.minter {
        // Fallback to sender if the minter is not defined
        None => info.sender.clone(),
        // Minter defined, make sure that is a valid address
        Some(minter_address) => deps.api.addr_validate(&minter_address)?,
    };

    // Validate the creator address
    let creator = deps.api.addr_validate(&msg.event_info.creator)?;

    // Check that start time is lower then end time
    if msg.event_info.start_time.ge(&msg.event_info.end_time) {
        return Err(ContractError::StartTimeAfterEndTime {
            start: msg.event_info.start_time,
            end: msg.event_info.end_time,
        });
    }

    // Check that the end time is in the future
    if env.block.time.le(&msg.event_info.end_time) {
        return Err(ContractError::EndTimeAlreadyPassed {
            end: msg.event_info.end_time,
        });
    }

    // Check that the poap uri is valid IPFS url
    let ipfs_url = Url::parse(&msg.event_info.base_poap_uri)
        .map_err(|_err| ContractError::InvalidPoapUri {})?;
    if ipfs_url.scheme() != "ipfs" {
        return Err(ContractError::InvalidPoapUri {});
    }

    // Check event uri
    Url::parse(&msg.event_info.event_uri).map_err(|_err| ContractError::InvalidEventUri {})?;

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
    let sub_msgs: Vec<SubMsg> = vec![SubMsg {
        msg: WasmMsg::Instantiate {
            admin: Some(admin.to_string()),
            code_id: msg.cw721_code_id.u64(),
            msg: to_binary(&Cw721InstantiateMsg {
                name: msg.cw721_initiate_msg.name,
                symbol: msg.cw721_initiate_msg.symbol,
                minter: env.contract.address.to_string(),
            })?,
            funds: info.funds,
            label: "poap cw721".to_string(),
        }
        .into(),
        id: INSTANTIATE_CW721_REPLY_ID,
        gas_limit: None,
        reply_on: ReplyOn::Success,
    }];

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("creator", creator)
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
        .add_submessages(sub_msgs))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::EnableMint {} => execute_set_mint_enabled(deps, info, true),
        ExecuteMsg::DisableMint {} => execute_set_mint_enabled(deps, info, false),
        ExecuteMsg::Mint {} => {
            let recipient_addr = info.sender.clone();
            execute_mint(deps, env, info, "mint", recipient_addr, false, false)
        }
        ExecuteMsg::MintTo { recipient } => {
            let recipient_addr = deps.api.addr_validate(&recipient)?;
            execute_mint(deps, env, info, "mint to", recipient_addr, true, true)
        }
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
        "enable mint"
    } else {
        "disable mint"
    };

    Ok(Response::new()
        .add_attribute("action", action)
        .add_attribute("sender", info.sender))
}

fn execute_mint(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    action: &str,
    recipient_addr: Addr,
    bypass_mint_enable: bool,
    check_is_minter: bool,
) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let event_info = EVENT_INFO.load(deps.storage)?;

    // Check if the event is terminated
    if env.block.time.gt(&event_info.end_time) {
        return Err(ContractError::EventTerminated {});
    }

    // Check if the mint is enabled
    if bypass_mint_enable == false && config.mint_enabled == false {
        return Err(ContractError::MintDisabled {});
    }

    // Check if who is performing the action is the minter
    if check_is_minter == true && info.sender != config.minter {
        return Err(ContractError::Unauthorized {});
    }

    // Get the nex poap id
    let poap_id = NEXT_POAP_ID.may_load(deps.storage)?.unwrap_or(1);

    // Create the cw721 message to send to mint the poap
    let mint_msg = Cw721ExecuteMsg::Mint(MintMsg::<Empty> {
        token_id: poap_id.to_string(),
        owner: recipient_addr.to_string(),
        token_uri: Some(format!("{}/{}", event_info.base_poap_uri, poap_id)),
        extension: Empty {},
    });

    let cw721_address = CW721_ADDRESS.load(deps.storage)?;
    let response_msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: cw721_address.to_string(),
        msg: to_binary(&mint_msg)?,
        funds: vec![],
    });

    // Update the next poap id state
    let new_poap_id = poap_id + 1;
    NEXT_POAP_ID.save(deps.storage, &new_poap_id)?;

    Ok(Response::new()
        .add_attribute("action", action)
        .add_attribute("sender", info.sender)
        .add_attribute("recipient", recipient_addr.to_string())
        .add_attribute("poap_id", poap_id.to_string())
        .add_message(response_msg))
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
        .add_attribute("action", "update admin")
        .add_attribute("new admin", &admin_address))
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
        .add_attribute("action", "update minter")
        .add_attribute("new minter", &minter_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
    }
}

fn query_config(deps: Deps) -> StdResult<QueryConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let cw721_address = CW721_ADDRESS.load(deps.storage)?;

    Ok(QueryConfigResponse {
        admin: config.admin.to_string(),
        minter: config.minter.to_string(),
        mint_enabled: config.mint_enabled,
        per_address_limit: config.per_address_limit,
        cw721_contract_code: config.cw721_code_id.into(),
        cw721_contract: cw721_address.to_string(),
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
            Ok(Response::default().add_attribute("action", "instantiate_cw721_reply"))
        }
        Err(_) => Err(ContractError::InstantiateCw721Error {}),
    }
}
