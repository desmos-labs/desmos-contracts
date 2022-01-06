use cosmwasm_std::{attr, entry_point, to_binary, Binary, Env, MessageInfo, Response, StdResult, Uint64, Timestamp, Addr, Order};

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
    state::{ContractDTag, DtagTransferRecord},
};
use crate::msg::SudoMsg;
use crate::state::{Auction, AUCTIONS_STORE, AuctionStatus, CONTRACT_DTAG_STORE, DTAG_TRANSFER_RECORD};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors and declare a custom Error variant for the ones where you will want to make use of it

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<DesmosMsgWrapper>> {
    let dtag = ContractDTag(
        msg.contract_dtag.clone()
    );

    CONTRACT_DTAG_STORE.save(deps.storage, &dtag)?;

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
        ExecuteMsg::CreateAuction {
            dtag,
            starting_price,
            max_participants,
            end_time,
            user
        } => handle_create_auction(deps, env, info, dtag, starting_price, max_participants, end_time, user),
        ExecuteMsg::MakeOffer { amount, user} => {}
        ExecuteMsg::RetreatOffer { user } => {}
        ExecuteMsg::CloseAuctionAndSellDTag { user } => {}
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
        return Err(ContractError::AlreadyExistentDtagRequest {})
    };

    let record = DtagTransferRecord::new(msg_sender.to_string(), AuctionStatus::PendingTransferRequest);
    dtag_transfer_records_store(deps.storage)
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

/// does_dtag_request_exists checks the existence of a dtag transfer request made by the user
fn does_dtag_request_exists(deps: &DepsMut, user: &str) -> bool {
    let dtag_request_record = DTAG_TRANSFER_RECORD.load(deps.storage);

    if dtag_request_record.is_ok() {
        return true
    }

    return false
}

pub fn handle_create_auction(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    dtag: String,
    starting_price: Uint64,
    max_participants: Uint64,
    end_time: Timestamp,
    user: String,
) -> Result<Response, ContractError> {

    // check if an auction made by the msg sender already exist
    let auctions: StdResult<Vec<_, >> = AUCTIONS_STORE.prefix(&info.sender)
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    if auctions.is_ok() {
        return Err(ContractError::AlreadyExistentAuction {})
    }

    let auctions= AUCTIONS_STORE.sub_prefix(&AuctionStatus::Active);


    if does_dtag_request_exists(&deps, info.sender.as_str()) {
        return Err(ContractError::AlreadyExistentDtagRequest {})
    }

    // save the record of the transfer that the contract will make in order to get the user's DTag before starting the auction
    let record = DtagTransferRecord::new(info.sender.to_string());
    dtag_transfer_records_store(deps.storage)
        .save(msg_sender.as_bytes(), &record)?;

    // create the Desmos native message to ask for a DTag transfer
    let dtag_transfer_req_msg = msg::request_dtag_transfer(
        env.contract.address.into_string(), info.sender.to_string());

    let auction = Auction::new(dtag, starting_price, max_participants,
                               Option::None, Option::None, user);



}

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::UpdateDtagAuctionStatus { user, status} =>
        update_dtag_auction_status(deps, user, status),
    }
}

pub fn update_dtag_auction_status(deps: DepsMut, user: String, status: AuctionStatus) -> Result<Response, ContractError> {
    let dtag_auction_record = dtag_transfer_records_store(deps.storage)
        .update(user.as_bytes(), |opt_record: Option<DtagTransferRecord> | -> Result<DtagTransferRecord, ContractError> {
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
