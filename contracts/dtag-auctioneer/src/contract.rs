use cosmwasm_std::{attr, entry_point, to_binary, Binary, Env, MessageInfo, Response, StdResult};

use desmos_std::{
    querier::DesmosQuerier,
    query_types::PostsResponse,
    types::{Deps, DepsMut, Post},
    msg,
    msg::DesmosMsgWrapper
};

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{state_read, state_store, State, dtag_requests_records_read,
            dtag_auction_records_store, DtagAuctionStatus},
};
use crate::msg::SudoMsg;
use crate::state::AuctionStatus;

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors and declare a custom Error variant for the ones where you will want to make use of it

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<DesmosMsgWrapper>> {
    let state = State {
        contract_dtag: msg.contract_dtag.clone(),
    };

    state_store(deps.storage).save(&state)?;

    let save_profile_msg = msg::save_profile(
        msg.contract_dtag.clone(),
        env.contract.address.to_string(),
    );

    let res: Response<DesmosMsgWrapper> = Response::new().
        add_message(save_profile_msg).
        add_attributes(vec![
            attr("action", "save_contract_dtag"),
            attr("dtag", msg.contract_dtag)
        ]);

    Ok(res)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::AskMeForDtagTransferRequest {} => handle_dtag_transfer_request_to_user(deps, env, info),
    }
}

pub fn handle_dtag_transfer_request_to_user(
    deps: DepsMut,
    env: Env,
    info: MessageInfo
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    let msg_sender = info.sender;
    let dtag_request_record = dtag_requests_records_read(deps.storage).load(
        msg_sender.as_bytes());

    // return error if the dtag request for the msg sender has already been made
    if dtag_request_record.is_ok() {
        return Err(ContractError::AlreadyStoredDtagRequest {})
    };

    let record = DtagAuctionStatus::new(msg_sender.to_string(), AuctionStatus::PendingTransferRequest);
    dtag_auction_records_store(deps.storage)
        .save(msg_sender.as_bytes(), &record)?;

    let dtag_transfer_req_msg = msg::request_dtag_transfer(
        env.contract.address.into_string(), msg_sender.to_string());

    let response = Response::new()
        .add_message(dtag_transfer_req_msg)
        .add_attributes(vec![
            attr("action", "dtag_transfer_request_to_user"),
            attr("user", msg_sender.clone()),
        ]);

    Ok(response)
}


#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::UpdateDtagAuctionStatus { user, status} =>
        update_dtag_auction_status(deps, user, status),
    }
}

pub fn update_dtag_auction_status(deps: DepsMut, user: String, status: AuctionStatus) -> Result<Response, ContractError> {
    let dtag_auction_record = dtag_auction_records_store(deps.storage)
        .update(user.as_bytes(), |opt_record: Option<DtagAuctionStatus> | -> Result<DtagAuctionStatus, ContractError> {
            let mut record = opt_record.ok_or_else(|| ContractError::DtagAuctionRecordNotFound {})?;
            record.status = status;
            Ok(record)
        })?;

    let response = Response::new()
        .add_attributes(vec![
            attr("action", "update_dtag_auction_status"),
            attr("status", dtag_auction_record.status.to_string()),
            attr("user", user.clone())
        ]);

    Ok(response)
}
