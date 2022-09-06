use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, ServiceFee, Target};
use crate::state::{Config, TipsRecordKey, CONFIG, TIPS_KEY_LIST, TIPS_RECORD};
use crate::utils;
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    Addr, BankMsg, Deps, DepsMut, Env, MessageInfo, QueryResponse, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use desmos_bindings::posts::querier::PostsQuerier;
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
    // Load the config from the storage
    let config = CONFIG.load(deps.storage)?;
    // Computes the fee and the coins to be sent to the user
    let (_fees, coin_to_send) = config.service_fee.compute_fees(info.funds)?;
    let (post_id, receiver) = match target {
        Target::ContentTarget { post_id } => {
            let querier = PostsQuerier::new(deps.querier.deref());
            let post = querier.query_post(config.subspace_id, post_id.u64())?.post;
            (post.id.u64(), post.author)
        }
        Target::UserTarget { receiver } => (0_u64, deps.api.addr_validate(&receiver)?),
    };

    let tip_key: TipsRecordKey = (info.sender.clone(), receiver.clone(), post_id);
    let tip_record_key = TIPS_RECORD.key(tip_key.clone());
    let tip_record_coins = if let Some(mut coins) = tip_record_key.may_load(deps.storage)? {
        // Append the new coins
        coins.extend(coin_to_send.clone());
        utils::merge_coins(coins)
    } else {
        // Load the key list
        let mut item_keys = TIPS_KEY_LIST
            .load(deps.storage)
            .map_err(ContractError::from)?;
        // If we have reached the threshold remove the oldest key
        if item_keys.len() == config.tips_record_threshold as usize {
            TIPS_RECORD.remove(deps.storage, item_keys.pop_front().unwrap());
        }
        // Add the new key to the end
        item_keys.push_back(tip_key);
        TIPS_KEY_LIST.save(deps.storage, &item_keys)?;
        // Return the new coins
        coin_to_send.clone()
    };
    tip_record_key.save(deps.storage, &tip_record_coins)?;

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
pub fn query(_deps: Deps<DesmosQuery>, _env: Env, _msg: QueryMsg) -> StdResult<QueryResponse> {
    Err(StdError::generic_err("not implemented"))
}

#[cfg(test)]
mod tests {}
