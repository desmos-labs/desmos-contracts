use cosmwasm_std::{attr, entry_point, to_binary, Binary, Env, MessageInfo, Response, StdResult, Uint64, Timestamp, Addr, Order, Coin, BankMsg};

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
use crate::state::{ACTIVE_AUCTION, Auction, AUCTIONS_STORE, AuctionStatus, CONTRACT_DTAG_STORE, DTAG_TRANSFER_RECORD, Offers};
use crate::state::AuctionStatus::Active;

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors and declare a custom Error variant for the ones where you will want to make use of it

#[cfg_attr(not(feature = "library"), entry_point)]
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

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    match msg {
        ExecuteMsg::CreateAuction {
            dtag,
            starting_price,
            max_participants
        } => handle_create_auction(deps, env, info.sender, dtag, starting_price, max_participants),
        ExecuteMsg::MakeOffer { amount} => handle_make_offer(deps, info.sender, amount),
        ExecuteMsg::RetreatOffer { user } => handle_retreat_offer(deps, info.sender),
        ExecuteMsg::CloseAuctionAndSellDTag { user } => handle_close_auction_and_sell_dtag(deps, info.sender)
    }
}

/// handle_create_auction manage the creation of an auction from the given creator
pub fn handle_create_auction(
    deps: DepsMut,
    env: Env,
    creator: Addr,
    dtag: String,
    starting_price: Uint64,
    max_participants: Uint64,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {

    // check if an auction made by the msg sender already exist
    if AUCTIONS_STORE.has(deps.storage,&creator) {
        return Err(ContractError::AlreadyExistentAuction {});
    }

    // check if the contract already sent a transfer request to the user
    let dtag_request_record = DTAG_TRANSFER_RECORD.may_load(deps.storage)?;
    if dtag_request_record.is_some() {
        return Err(ContractError::AlreadyExistentDtagRequest {});
    }

    let mut new_auction = Auction::new(
        dtag,
        starting_price,
        max_participants,
        None,
        None,
        AuctionStatus::Inactive,
        creator.clone()
    );

    AUCTIONS_STORE.save(deps.storage, &creator, &new_auction)?;

    // prepare standard response
    let mut res = Response::new()
        .add_attribute("action", "create_auction")
        .add_attribute("creator", creator)
        .add_attribute("dtag", dtag.clone());

    // get the active auction, if it exists
    let active_auction = ACTIVE_AUCTION.may_load(deps.storage)?;
    // if an active auction doesn't exist, add to the response a request dtag transfer message for the
    // current auction
    if !active_auction.is_some() {
        // save the transfer request record made by the contract
        /// TODO is it necessary? what if the transfer tx fails? (It should be reverted by the contract)
        let record = DtagTransferRecord::new(creator.to_string());
        DTAG_TRANSFER_RECORD.save(deps.storage, &record);

        // create the Desmos native message to ask for a DTag transfer
        let dtag_transfer_req_msg = msg::request_dtag_transfer(
            env.contract.address.into_string(), creator.to_string());

        // add the message to the response, it will be triggered at the end of the execution
        res.add_message(dtag_transfer_req_msg);
    }

    Ok(res)
}

/// handle_make_offer manage the creation and insertion of a new auction offer from a user
pub fn handle_make_offer(deps: DepsMut, user: Addr, amount: Vec<Coin>)
    -> Result<Response, ContractError> {
    let auction = ACTIVE_AUCTION.may_load(deps.storage)?;
    let mut auction = auction.ok_or(ContractError::AuctionNotFound {})?;
    auction.add_offer(user, amount);

    let res = Response::new()
        .add_attribute("action", "make_offer")
        .add_attribute("user", user.clone())
        .add_attribute("amount", amount.clone())
        .add_attribute("dtag", auction.dtag);

    Ok(res)
}

/// handle_retreat_offer manage the removal of an existent auction offer from a user
pub fn handle_retreat_offer(deps: DepsMut, user: Addr) -> Result<Response, ContractError> {
    let auction = ACTIVE_AUCTION.may_load(deps.storage)?;
    let mut auction = auction.ok_or(ContractError::AuctionNotFound {})?;

    auction.remove_offer(user).ok_or(ContractError::OfferNotFound {});

    let res = Response::new()
        .add_attribute("action", "retreat_offer")
        .add_attribute("user", user.clone())
        .add_attribute("dtag", auction.dtag);

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::ActivateAuctionForUser { creator } =>
        activate_auction_for_user(deps, env, creator),
        SudoMsg::CompleteAuction { .. } => {}
    }
}

pub fn complete_auction_and_activate_next_one(deps: DepsMut, env: Env, user: Addr) -> Result<Response, ContractError> {
    // Get the current active auction if exists, otherwise return error
    let auction = ACTIVE_AUCTION.may_load(deps.storage)?;
    let auction = auction.ok_or(ContractError::AuctionNotFound {})?;

    // Fetch the best offer
    let (_, best_offer) = auction.get_best_offer().ok_or(ContractError::OfferNotFound {})?;

    // Prepare the bank msg with the funds to be sent to the auction creator
    let deliver_offer_msg = BankMsg::Send {
        to_address: auction.user.into_string(),
        amount: best_offer.clone()
    };

}

pub fn activate_auction_for_user(deps: DepsMut, env: Env, user: Addr) -> Result<Response, ContractError> {
    let auction = AUCTIONS_STORE.may_load(deps.storage, &user)?;
    let mut auction = auction.ok_or(ContractError::AuctionNotFound {})?;

    auction.start_time = Some(env.block.time);
    // the actual end time is 2 days later the auction start. 2 days = 172800
    auction.end_time = Some(env.block.time.plus_seconds(172800));
    auction.status = AuctionStatus::Active;

    // save the auction inside the active auction store
    ACTIVE_AUCTION.save(deps.storage, &auction);

    // remove it from the inactive store
    AUCTIONS_STORE.remove(deps.storage, &user);

    let response = Response::new()
        .add_attributes(vec![
            attr("action", "update_dtag_auction_status"),
            attr("status", "Activate"),
            attr("user", user)
        ]);

    Ok(response)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetActiveAuction { } => {
            to_binary(&query_active_auction(deps)?)
        }
        QueryMsg::GetAuctionByUser { user } => {}
    }
}

pub fn query_active_auction(deps: Deps) -> StdResult<Auction> {
    let auction = ACTIVE_AUCTION.load(deps.storage);
    if auction.is_err() {
        Err(ContractError::AuctionNotFound {})
    }

    Ok(auction?)
}

pub fn query_auction_by_user(deps: Deps, user: Addr) -> StdResult<Auction> {
    let auction = AUCTIONS_STORE.load(deps.storage, &user);
    if auction.is_err() {
        Err(ContractError::AuctionNotFound {})
    }

    Ok(auction?)
}
