use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{Config, EventInfo, CONFIG, CW721_ADDRESS, EVENT_INFO};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, ReplyOn, Response, StdError,
    StdResult, SubMsg, WasmMsg,
};
use cw2::set_contract_version;
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
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
        creator: creator.clone(),
        admin: admin.clone(),
        minter: minter.clone(),
        per_address_limit: msg.event_info.per_address_limit,
        cw721_code_id: msg.cw721_code_id.u64(),
        mint_enabled: false,
    };
    // Save the received event info.
    CONFIG.save(deps.storage, &config)?;

    let event_info = EventInfo {
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::EnableMint {} => execute_set_mint_enabled(deps, info, true),
        ExecuteMsg::DisableMint {} => execute_set_mint_enabled(deps, info, false),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
        ExecuteMsg::UpdateMinter { new_minter } => execute_update_minter(deps, info, new_admin),
        _ => Err(ContractError::Unauthorized {}),
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
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        _ => Err(StdError::generic_err("Query operation not supported")),
    }
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
