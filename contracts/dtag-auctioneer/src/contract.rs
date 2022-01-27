use cosmwasm_std::{
    attr, entry_point, to_binary, Addr, BankMsg, Binary, CosmosMsg, Env, MessageInfo, Order,
    Response, StdResult, Uint128, Uint64,
};
use std::str::FromStr;
use desmos_std::{
    msg,
    msg::DesmosMsgWrapper,
    types::{Deps, DepsMut},
};
use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg},
    state::{
        Auction, AuctionStatus, ContractDTag, DtagTransferRecord, DtagTransferStatus,
        ACTIVE_AUCTION, AUCTIONS_STORE, CONTRACT_DTAG_STORE, DTAG_TRANSFER_RECORD,
    },
};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors and declare a custom Error variant for the ones where you will want to make use of it

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response<DesmosMsgWrapper>> {
    let dtag = ContractDTag(msg.contract_dtag.clone());

    CONTRACT_DTAG_STORE.save(deps.storage, &dtag)?;

    let save_profile_msg = msg::save_profile(
        msg.contract_dtag.clone(),
        env.contract.address.to_string(),
        Some("Dtag auctioneer contract".to_string()),
        None,
        None,
        None,
    );

    let response: Response<DesmosMsgWrapper> = Response::new()
        .add_message(save_profile_msg)
        .add_attribute("action", "save_contract_profile")
        .add_attribute("dtag", msg.contract_dtag);

    Ok(response)
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
            max_participants,
        } => handle_create_auction(
            deps,
            env,
            info.sender,
            dtag,
            starting_price,
            max_participants,
        ),
        ExecuteMsg::MakeOffer {} => handle_make_offer(deps, info),
        ExecuteMsg::RetreatOffer {} => handle_retreat_offer(deps, info.sender),
    }
}

/// handle_create_auction manage the creation of an auction from the given creator
pub fn handle_create_auction(
    deps: DepsMut,
    env: Env,
    creator: Addr,
    dtag: String,
    starting_price: Uint128,
    max_participants: Uint64,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    // check if an auction made by the msg sender already exist
    if AUCTIONS_STORE.has(deps.storage, &creator) {
        return Err(ContractError::AlreadyExistentAuction {});
    }

    // check if the contract already sent a transfer request to the user
    let dtag_request_record = DTAG_TRANSFER_RECORD.may_load(deps.storage);
    if dtag_request_record.is_err() {
        return Err(ContractError::AlreadyExistentDtagRequest {});
    }

    // create the new auction and set it as inactive
    let new_auction = Auction::new(
        dtag.clone(),
        starting_price,
        max_participants,
        None,
        None,
        AuctionStatus::Inactive,
        creator.clone(),
    );

    AUCTIONS_STORE.save(deps.storage, &creator, &new_auction)?;

    // prepare the standard response
    let mut response = Response::new()
        .add_attribute("action", "create_auction")
        .add_attribute("creator", creator.clone())
        .add_attribute("dtag", dtag);

    // get the active auction, if it exists
    let active_auction = ACTIVE_AUCTION.may_load(deps.storage)?;
    // if an active auction doesn't exist, add to the response a request dtag transfer message for the
    // current auction
    if active_auction.is_none() {
        // save the transfer request record made by the contract
        // TODO is it necessary? what if the transfer tx fails? (It should be reverted by the contract)
        let record = DtagTransferRecord::new(creator.to_string());
        DTAG_TRANSFER_RECORD.save(deps.storage, &record)?;

        // create the Desmos native message to ask for a DTag transfer to the auction creator
        let dtag_transfer_req_msg =
            msg::request_dtag_transfer(env.contract.address.into_string(), creator.to_string());

        // add the message to the response, it will be triggered at the end of the execution
        response = response.add_message(dtag_transfer_req_msg);
    }

    Ok(response)
}

/// handle_make_offer manage the creation and insertion of a new auction offer from a user
pub fn handle_make_offer(
    deps: DepsMut,
    info: MessageInfo,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    let auction = ACTIVE_AUCTION.may_load(deps.storage)?;
    let mut auction = auction.ok_or(ContractError::AuctionNotFound {})?;
    let offer = info.funds;

    if offer[0].amount < auction.starting_price {
        return Err(ContractError::MinimumPriceNotReached {});
    }

    auction.add_offer(info.sender.clone(), offer.clone());

    let res = Response::new()
        .add_attribute("action", "make_offer")
        .add_attribute("user", info.sender)
        .add_attribute("amount", offer[0].amount)
        .add_attribute("dtag", auction.dtag);

    Ok(res)
}

/// handle_retreat_offer manage the removal of an existent auction offer from a user
pub fn handle_retreat_offer(
    deps: DepsMut,
    user: Addr,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    ACTIVE_AUCTION.update(
        deps.storage,
        | auction | -> StdResult<_> {
            let mut auction= auction;
            auction.remove_offer(user.clone())
                .ok_or(ContractError::OfferNotFound {});
            Ok(auction)
        }
    )?;

    let res = Response::new()
        .add_attribute("action", "retreat_offer")
        .add_attribute("user", user);

    Ok(res)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(
    deps: DepsMut,
    env: Env,
    msg: SudoMsg,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    match msg {
        SudoMsg::UpdateDTagAuctionStatus {
            user,
            transfer_status,
        } => {
            let dtag_transfer_status = DtagTransferStatus::from_str(transfer_status.as_str())?;
            update_dtag_auction_status(deps, env, user, dtag_transfer_status)
        }

        SudoMsg::CompleteAuction {} => complete_auction(deps, env),
    }
}

/// update_dtag_auction_status updates the status of an inactive auction created by the user with the given dtag_transfer_status
/// if the status == Accepted, then the auction will be activated, otherwise it will be deleted.
pub fn update_dtag_auction_status(
    deps: DepsMut,
    env: Env,
    user: Addr,
    dtag_transfer_status: DtagTransferStatus,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    let auction = AUCTIONS_STORE.may_load(deps.storage, &user)?;
    let mut auction = auction.ok_or(ContractError::AuctionNotFound {})?;
    let mut status_attr: &str = "Deleted";

    // if the DTag transfer has been accepted by the user who created the auction
    if dtag_transfer_status == DtagTransferStatus::Accepted {
        // activate the auction
        auction.activate(env.block.time);
        // save the auction inside the active auction store
        ACTIVE_AUCTION.save(deps.storage, &auction)?;
        status_attr = "Activated"
    }

    // remove it from the inactive store (applies either if the auction has been activated or not)
    AUCTIONS_STORE.remove(deps.storage, &user);

    let response = Response::new().add_attributes(vec![
        attr("action", "update_dtag_auction_status"),
        attr("status", status_attr),
        attr("user", user),
    ]);

    Ok(response)
}

/// complete_auction takes care of closing the auction when it reaches the ending time
/// and consequently start the next one
pub fn complete_auction(
    deps: DepsMut,
    env: Env,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    // Get the current active auction if exists, otherwise return error
    let auction = ACTIVE_AUCTION.may_load(deps.storage)?;
    let auction = auction.ok_or(ContractError::AuctionNotFound {})?;

    // Fetch the best offer
    let (_, best_offer) = auction
        .get_best_offer()
        .ok_or(ContractError::OfferNotFound {})?;

    // Prepare the bank msg with the funds to be sent to the auction creator
    let deliver_offer_msg = BankMsg::Send {
        to_address: auction.user.clone().into_string(),
        amount: best_offer.clone(),
    };

    let transfer_request_msg = start_next_auction_activation_process(deps, env);

    let response = Response::new()
        .add_message(deliver_offer_msg)
        .add_message(transfer_request_msg)
        .add_attribute("action", "complete_auction")
        .add_attribute("user", auction.user.clone())
        .add_attribute("dtag", auction.dtag);

    Ok(response)
}

/// start_auction_activation_process manage to activate a new auction from the inactive ones
fn start_next_auction_activation_process(deps: DepsMut, env: Env) -> CosmosMsg<DesmosMsgWrapper> {
    // Get the next auction to process
    let inactive_auctions: StdResult<Vec<_>> = AUCTIONS_STORE
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let next_active_auction = inactive_auctions.unwrap()[0].clone().1;

    // Prepare the request for the next active auction
    msg::request_dtag_transfer(
        env.contract.address.into_string(),
        next_active_auction.user.to_string(),
    )
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetActiveAuction {} => to_binary(&query_active_auction(deps)?),
        QueryMsg::GetAuctionByUser { user } => to_binary(&query_auction_by_user(deps, user)?),
    }
}

/// query_active_auction return the current active auction
pub fn query_active_auction(deps: Deps) -> StdResult<Auction> {
    let auction = ACTIVE_AUCTION.load(deps.storage)?;
    Ok(auction)
}

/// query_auction_by_user return the auction associated with the given user, if exists
pub fn query_auction_by_user(deps: Deps, user: Addr) -> StdResult<Auction> {
    let auction = AUCTIONS_STORE.load(deps.storage, &user)?;
    Ok(auction)
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::Coin;
    use cosmwasm_vm::testing::{mock_env, mock_info};
    use desmos_std::mock::mock_dependencies_with_custom_querier;
    use super::*;

    /// setup_test is an helper func to instantiate the contract
    fn setup_test(deps: DepsMut, env: Env, info: MessageInfo, dtag: &str) {
        let instantiate_msg = InstantiateMsg {
            contract_dtag: dtag.to_string()
        };
        instantiate(deps, env, info, instantiate_msg).unwrap();
    }

    #[test]
    fn test_instantiate() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);

        let instantiate_msg = InstantiateMsg {
            contract_dtag: "auctioneer_contract".to_string()
        };

        let res = instantiate(
            deps.as_mut(),
            mock_env(),
            info,
            instantiate_msg
        ).unwrap();

        let exp_response = vec![
            attr("action", "save_contract_profile"),
            attr("dtag", "auctioneer_dtag")
        ];

        let dtag = CONTRACT_DTAG_STORE.load(&deps.storage).unwrap();
        assert_eq!("auctioneer_contract", dtag.0.as_str())
    }

    #[test]
    fn test_handle_dtag_auction() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);

        setup_test(deps.as_mut(), env, info, "contract_dtag");

        let res = handle_create_auction(
            deps.as_mut(),
            env,
            info.sender.clone(),
            "sender_dtag".to_string(),
            Uint128(100),
            Uint64(50),
        );

        let auction = AUCTIONS_STORE.load(&deps.storage, &info.sender).unwrap();

        let dtag_record = DTAG_TRANSFER_RECORD.load(&deps.storage);
    }



}
