use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SudoMsg},
    state::{
        Auction, AuctionStatus, ContractDTag, DtagTransferStatus, ACTIVE_AUCTION, AUCTIONS_STORE,
        CONTRACT_DTAG_STORE,
    },
};
use cosmwasm_std::{
    attr, entry_point, to_binary, Addr, BankMsg, Binary, Env, MessageInfo, Order, Response,
    StdResult, Uint128, Uint64,
};
use desmos_std::{
    msg,
    msg::DesmosMsgWrapper,
    types::{Deps, DepsMut},
};
use std::str::FromStr;

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
        } => execute_create_auction(
            deps,
            env,
            info.sender,
            dtag,
            starting_price,
            max_participants,
        ),
        ExecuteMsg::MakeOffer {} => execute_make_offer(deps, env, info),
        ExecuteMsg::RetreatOffer {} => execute_retreat_offer(deps, info.sender),
        ExecuteMsg::CompleteAuction {} => execute_complete_auction(deps, env, info.sender),
    }
}

/// handle_create_auction manage the creation of an auction from the given creator
pub fn execute_create_auction(
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
        // create the Desmos native message to ask for a DTag transfer to the auction creator
        let dtag_transfer_req_msg =
            msg::request_dtag_transfer(env.contract.address.into_string(), creator.to_string());

        // append the message to the response, it will be triggered at the end of the contract execution
        response = response.add_message(dtag_transfer_req_msg);
    }

    Ok(response)
}

/// handle_make_offer manage the creation and insertion of a new auction offer from a user
/// TODO this should be enhanced
pub fn execute_make_offer(
    deps: DepsMut,
    env:  Env,
    info: MessageInfo,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    let auction = ACTIVE_AUCTION
        .may_load(deps.storage)?
        .ok_or(ContractError::AuctionNotFound {})?;

    if env.block.time > auction.end_time.unwrap() {
        return Err(ContractError::AlreadyClosedAuction {})
    }

    let offers = auction.count_offers(deps.storage);

    if offers > auction.max_participants.u64() {
        return Err(ContractError::MaxParticipantsNumberReached {});
    }

    let offer = info.funds;

    if offers == 0 && offer[0].amount < auction.starting_price {
        return Err(ContractError::MinimumPriceNotReached {});
    }

    if offers > 0 {
        let last_offer = auction.get_last_offer(deps.storage)?;
        if offer[0].amount < last_offer[0].amount {
            return Err(ContractError::MinimumPriceNotReached {});
        }
    }

    let _ = auction.add_offer(deps.storage, info.sender.clone(), offer.clone());

    let res = Response::new()
        .add_attribute("action", "make_offer")
        .add_attribute("user", info.sender)
        .add_attribute("amount", offer[0].amount)
        .add_attribute("dtag", auction.dtag);

    Ok(res)
}

/// handle_retreat_offer manage the removal of an existent auction offer from a user
pub fn execute_retreat_offer(
    deps: DepsMut,
    user: Addr,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    let auction = ACTIVE_AUCTION
        .may_load(deps.storage)?
        .ok_or(ContractError::AuctionNotFound {})?;

    auction.remove_offer(deps.storage, user.clone());

    let res = Response::new()
        .add_attribute("action", "retreat_offer")
        .add_attribute("user", user);

    Ok(res)
}

/// complete_auction takes care of closing the auction when it reaches the ending time
/// and consequently start the next one
pub fn execute_complete_auction(
    deps: DepsMut,
    env: Env,
    sender: Addr,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    // Get the current active auction if exists, otherwise return error
    let auction = ACTIVE_AUCTION
        .may_load(deps.storage)?
        .ok_or(ContractError::AuctionNotFound {})?;

    // Check if the sender is the actual auction creator
    if sender != auction.creator {
        return Err(ContractError::InvalidAuctionCreator {})
    }

    // Check if the auction is still active
    // TODO we need to think about this better.
    if env.block.time < auction.end_time.unwrap() {
        return Err(ContractError::StillActiveAuction {})
    }

    // Get the best offer
    let (offerer, best_offer) = auction.get_best_offer(deps.storage)?;

    // Prepare the bank msg with the funds to be sent to the auction creator
    let deliver_offer_msg = BankMsg::Send {
        to_address: auction.creator.clone().into_string(),
        amount: best_offer,
    };

    // Prepare the dtag transfer request message for the winner
    let dtag_transfer_request_msg =
        msg::request_dtag_transfer(offerer.to_string(), env.contract.address.to_string());

    // remove it from the inactive store (applies either if the auction has been activated or not)
    AUCTIONS_STORE.remove(deps.storage, &auction.creator.clone());

    let response = Response::new()
        .add_message(deliver_offer_msg)
        .add_message(dtag_transfer_request_msg)
        .add_attribute("action", "complete_auction")
        .add_attribute("user", auction.creator.clone())
        .add_attribute("dtag", auction.dtag)
        .add_attribute("winner", offerer);

    Ok(response)
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
        SudoMsg::StartNextAuction {} => start_next_auction_activation_process(deps, env),
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
    let mut auction = AUCTIONS_STORE
        .may_load(deps.storage, &user)?
        .ok_or(ContractError::AuctionNotFound {})?;
    let mut status_attr: &str = "Deleted";

    // if the DTag transfer has been accepted by the user who created the auction
    if dtag_transfer_status == DtagTransferStatus::Accepted {
        // activate the auction
        auction.activate(env.block.time);
        // save the auction inside the active auction store
        ACTIVE_AUCTION.save(deps.storage, &auction)?;
        status_attr = "Activated"
    }

    if dtag_transfer_status == DtagTransferStatus::Refused {
        // delete the auction
        AUCTIONS_STORE.remove(deps.storage, &user)
    }

    let response = Response::new().add_attributes(vec![
        attr("action", "update_dtag_auction_status"),
        attr("status", status_attr),
        attr("user", user),
    ]);

    Ok(response)
}

/// start_auction_activation_process manage to activate a new auction from the inactive ones
fn start_next_auction_activation_process(
    deps: DepsMut,
    env: Env,
) -> Result<Response<DesmosMsgWrapper>, ContractError> {
    // Get the next auction to activate
    let inactive_auctions_record: StdResult<Vec<_>> = AUCTIONS_STORE
        .range(deps.storage, None, None, Order::Ascending)
        .collect();

    let inactive_auctions = inactive_auctions_record.unwrap();

    // Check if there are any auction left to be processed
    if inactive_auctions.is_empty() {
        return Err(ContractError::NoPendingAuctionsLeft {});
    }

    // Get the first auction created
    let next_active_auction = inactive_auctions[0].clone().1;

    // Prepare the request for the next active auction
    let transfer_req_msg = msg::request_dtag_transfer(
        env.contract.address.into_string(),
        next_active_auction.creator.to_string(),
    );

    let response = Response::new()
        .add_message(transfer_req_msg)
        .add_attribute("action", "start_next_auction")
        .add_attribute("user", next_active_auction.creator.clone())
        .add_attribute("dtag", next_active_auction.dtag);

    Ok(response)
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
    use super::*;
    use crate::error::ContractError::AuctionNotFound;
    use crate::state::AUCTION_OFFERS_STORE;
    use cosmwasm_std::OverflowOperation::Add;
    use cosmwasm_std::WasmMsg::Execute;
    use cosmwasm_std::{from_binary, Coin, Timestamp};
    use cosmwasm_vm::testing::{mock_env, mock_info};
    use desmos_std::mock::mock_dependencies_with_custom_querier;

    /// setup_test is an helper func to instantiate the contract
    fn setup_test(deps: DepsMut, env: Env, info: MessageInfo, dtag: &str) {
        let instantiate_msg = InstantiateMsg {
            contract_dtag: dtag.to_string(),
        };
        instantiate(deps, env, info, instantiate_msg).unwrap();
    }

    fn save_auction(deps: DepsMut, creator: Addr, auction: Auction) {
        AUCTIONS_STORE.save(deps.storage, &creator, &auction);
    }

    fn save_active_auction(deps: DepsMut, auction: Auction) {
        ACTIVE_AUCTION.save(deps.storage, &auction);
    }

    fn add_auction_offer(deps: DepsMut, offerer_addr: Addr, offer: Vec<Coin>) {
        AUCTION_OFFERS_STORE.save(deps.storage, &offerer_addr, &offer);
    }

    #[test]
    fn test_instantiate() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);

        let instantiate_msg = InstantiateMsg {
            contract_dtag: "auctioneer_contract".to_string(),
        };

        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

        let exp_response = vec![
            attr("action", "save_contract_profile"),
            attr("dtag", "auctioneer_dtag"),
        ];

        let dtag = CONTRACT_DTAG_STORE.load(&deps.storage).unwrap();
        assert_eq!("auctioneer_contract", dtag.0.as_str())
    }

    #[test]
    fn test_execute_dtag_auction_creation_successful() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");

        let msg = ExecuteMsg::CreateAuction {
            dtag: "sender_dtag".to_string(),
            starting_price: Uint128::new(100),
            max_participants: Uint64::new(50),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg);

        let auction = AUCTIONS_STORE.load(&deps.storage, &info.sender).unwrap();

        let expected_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            None,
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        assert_eq!(expected_auction, auction);
    }

    #[test]
    fn test_execute_dtag_auction_creation_auction_already_present() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let present_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            None,
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_auction(deps.as_mut(), info.sender.clone(), present_auction);

        let msg = ExecuteMsg::CreateAuction {
            dtag: "sender_dtag".to_string(),
            starting_price: Uint128::new(100),
            max_participants: Uint64::new(50),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg);

        assert_eq!(ContractError::AlreadyExistentAuction {}, res.unwrap_err())
    }

    #[test]
    fn test_execute_dtag_auction_creation_auction_already_active() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let active_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            None,
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_active_auction(deps.as_mut(), active_auction);

        let msg = ExecuteMsg::CreateAuction {
            dtag: "another_dtag".to_string(),
            starting_price: Uint128::new(100),
            max_participants: Uint64::new(50),
        };

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let exp_response = Response::new()
            .add_attribute("action", "create_auction")
            .add_attribute("creator", info.sender.clone())
            .add_attribute("dtag", "another_dtag");

        assert_eq!(exp_response, res)
    }

    #[test]
    fn test_execute_make_offer() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let active_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            Some(Timestamp::from_seconds(10000000000)),
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_active_auction(deps.as_mut(), active_auction);

        let msg = ExecuteMsg::MakeOffer {};

        let res = execute(
            deps.as_mut(),
            env.clone(),
            mock_info("offerer_addr", &[Coin::new(1000, "udsm")]),
            msg,
        );

        let offer = AUCTION_OFFERS_STORE
            .load(&deps.storage, &Addr::unchecked("offerer_addr"))
            .unwrap();

        assert_eq!(vec![Coin::new(1000, "udsm")], offer)
    }

    #[test]
    fn test_execute_make_offer_no_active_auction() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let active_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            Some(Timestamp::from_seconds(10000000000)),
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");

        let msg = ExecuteMsg::MakeOffer {};

        let err = execute(
            deps.as_mut(),
            env.clone(),
            mock_info("offerer_addr", &[Coin::new(1000_000_000, "udsm")]),
            msg,
        )
        .unwrap_err();

        assert_eq!(ContractError::AuctionNotFound {}, err)
    }

    #[test]
    fn test_execute_make_offer_amount_offer_lower_than_minimum() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let active_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            Some(Timestamp::from_seconds(10000000000)),
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_active_auction(deps.as_mut(), active_auction);

        let msg = ExecuteMsg::MakeOffer {};
        let err = execute(
            deps.as_mut(),
            env.clone(),
            mock_info("offerer_addr", &[Coin::new(10, "udsm")]),
            msg,
        )
        .unwrap_err();

        assert_eq!(ContractError::MinimumPriceNotReached {}, err)
    }

    #[test]
    fn test_execute_make_offer_amount_offer_lower_than_the_last_offer() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let active_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            Some(Timestamp::from_seconds(10000000000)),
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_active_auction(deps.as_mut(), active_auction);
        add_auction_offer(
            deps.as_mut(),
            Addr::unchecked("offerer"),
            vec![Coin::new(200, "udsm")],
        );

        let msg = ExecuteMsg::MakeOffer {};
        let err = execute(
            deps.as_mut(),
            env.clone(),
            mock_info("offerer_addr", &[Coin::new(100, "udsm")]),
            msg,
        )
        .unwrap_err();

        assert_eq!(ContractError::MinimumPriceNotReached {}, err)
    }

    #[test]
    fn test_execute_make_offer_max_participants_number_reached() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let active_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(1),
            None,
            Some(Timestamp::from_seconds(10000000000)),
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        let offerers = vec![Addr::unchecked("addr1"), Addr::unchecked("addr2")];
        let mut amount = 10;
        for offerer in offerers {
            add_auction_offer(deps.as_mut(), offerer, vec![Coin::new(amount, "udsm")]);
            amount += 10
        }

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_active_auction(deps.as_mut(), active_auction);

        let msg = ExecuteMsg::MakeOffer {};
        let err = execute(
            deps.as_mut(),
            env.clone(),
            mock_info("offerer_addr", &[Coin::new(100, "udsm")]),
            msg,
        )
        .unwrap_err();

        assert_eq!(ContractError::MaxParticipantsNumberReached {}, err)
    }

    #[test]
    fn test_execute_retreat_offer() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();
        let offerer_addr = Addr::unchecked("offerer_addr");
        let exp_response = Response::new()
            .add_attribute("action", "retreat_offer")
            .add_attribute("user", offerer_addr.clone());

        let active_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            None,
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_active_auction(deps.as_mut(), active_auction);

        add_auction_offer(
            deps.as_mut(),
            offerer_addr.clone(),
            vec![Coin::new(1000, "udsm")],
        );

        let msg = ExecuteMsg::RetreatOffer {};
        let response = execute(
            deps.as_mut(),
            env.clone(),
            mock_info(offerer_addr.clone().as_str(), &[]),
            msg,
        )
        .unwrap();

        assert_eq!(exp_response, response);

        let res = AUCTION_OFFERS_STORE.has(&deps.storage, &offerer_addr.clone());
        assert_eq!(false, res)
    }

    #[test]
    fn test_execute_retreat_offer_no_active_auction() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();
        let offerer_addr = Addr::unchecked("offerer_addr");

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");

        let msg = ExecuteMsg::RetreatOffer {};

        let response = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();

        assert_eq!(ContractError::AuctionNotFound {}, response);
    }

    #[test]
    fn test_execute_complete_auction() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let offerers_addresses = vec![
            Addr::unchecked("first"),
            Addr::unchecked("second"),
            Addr::unchecked("third"),
        ];
        let mut amount = 100;
        let active_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            Some(Timestamp::from_nanos(10)),
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        let exp_response: Response<DesmosMsgWrapper> = Response::new()
            .add_attribute("action", "complete_auction")
            .add_attribute("user", info.sender.clone())
            .add_attribute("dtag", active_auction.dtag.clone())
            .add_attribute("winner", offerers_addresses[2].clone());

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_active_auction(deps.as_mut(), active_auction);

        for addr in offerers_addresses {
            amount += 10;
            add_auction_offer(deps.as_mut(), addr, vec![Coin::new(amount, "udsm")]);
        }

        let execute_msg = ExecuteMsg::CompleteAuction {};

        let response = execute(deps.as_mut(), env.clone(), info.clone(), execute_msg).unwrap();

        assert_eq!(exp_response.attributes, response.attributes);

        let res = AUCTIONS_STORE.has(&deps.storage, &info.sender.clone());
        assert_eq!(false, res)
    }

    #[test]
    fn test_execute_complete_auction_no_auction_found() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");

        let execute_msg = ExecuteMsg::CompleteAuction {};
        let response = execute(deps.as_mut(), env.clone(), info, execute_msg).unwrap_err();

        assert_eq!(ContractError::AuctionNotFound {}, response)
    }

    #[test]
    fn test_sudo_update_dtag_auction_status() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            None,
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_auction(deps.as_mut(), info.sender.clone(), auction);

        let sudo_msg = SudoMsg::UpdateDTagAuctionStatus {
            user: info.sender.clone(),
            transfer_status: "accept_dtag_transfer_request".to_string(),
        };

        let response = sudo(deps.as_mut(), env.clone(), sudo_msg).unwrap();

        let exp_response = Response::new().add_attributes(vec![
            attr("action", "update_dtag_auction_status"),
            attr("status", "Activated"),
            attr("user", info.sender.clone()),
        ]);

        assert_eq!(exp_response, response);

        let auction = ACTIVE_AUCTION.load(&deps.storage).unwrap();
        assert_eq!(env.clone().block.time, auction.start_time.unwrap())
    }

    #[test]
    fn test_sudo_update_dtag_auction_status_refused_dtag_transfer_request() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            None,
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_auction(deps.as_mut(), info.sender.clone(), auction);

        let sudo_msg = SudoMsg::UpdateDTagAuctionStatus {
            user: info.sender.clone(),
            transfer_status: "refuse_dtag_transfer_request".to_string(),
        };

        let response = sudo(deps.as_mut(), env.clone(), sudo_msg).unwrap();

        let exp_response = Response::new().add_attributes(vec![
            attr("action", "update_dtag_auction_status"),
            attr("status", "Deleted"),
            attr("user", info.sender.clone()),
        ]);

        assert_eq!(exp_response, response);

        let result = AUCTIONS_STORE.has(&deps.storage, &info.sender.clone());
        assert_eq!(false, result)
    }

    #[test]
    fn test_sudo_update_dtag_auction_status_no_auction_found() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            None,
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");

        let sudo_msg = SudoMsg::UpdateDTagAuctionStatus {
            user: info.sender.clone(),
            transfer_status: "accept_dtag_transfer_request".to_string(),
        };

        let err = sudo(deps.as_mut(), env, sudo_msg).unwrap_err();

        assert_eq!(ContractError::AuctionNotFound {}, err)
    }

    #[test]
    fn test_sudo_start_next_auction_activation_process() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");

        let users = vec![
            Addr::unchecked("first"),
            Addr::unchecked("second"),
            Addr::unchecked("third"),
        ];

        let auctions = vec![
            Auction::new(
                "sender_dtag".to_string(),
                Uint128::new(100),
                Uint64::new(50),
                None,
                None,
                AuctionStatus::Inactive,
                users[0].clone(),
            ),
            Auction::new(
                "sender_dtag".to_string(),
                Uint128::new(100),
                Uint64::new(50),
                None,
                None,
                AuctionStatus::Inactive,
                users[1].clone(),
            ),
            Auction::new(
                "sender_dtag".to_string(),
                Uint128::new(100),
                Uint64::new(50),
                None,
                None,
                AuctionStatus::Inactive,
                users[2].clone(),
            ),
        ];

        let exp_response: Response<DesmosMsgWrapper> = Response::new()
            .add_attribute("action", "start_next_auction")
            .add_attribute("user", auctions[0].creator.clone())
            .add_attribute("dtag", auctions[0].dtag.clone());

        for auction in auctions {
            save_auction(deps.as_mut(), auction.creator.clone(), auction);
        }

        let sudo_msg = SudoMsg::StartNextAuction {};
        let response = sudo(deps.as_mut(), env.clone(), sudo_msg).unwrap();

        assert_eq!(exp_response.attributes, response.attributes)
    }

    #[test]
    fn test_sudo_start_next_auction_activation_process_no_auctions_left() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");

        let sudo_msg = SudoMsg::StartNextAuction {};
        let error = sudo(deps.as_mut(), env.clone(), sudo_msg).unwrap_err();

        assert_eq!(ContractError::NoPendingAuctionsLeft {}, error)
    }

    #[test]
    fn test_query_active_auction() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let active_auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            None,
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_active_auction(deps.as_mut(), active_auction.clone());

        let msg = QueryMsg::GetActiveAuction {};

        let result: Auction = from_binary(&query(deps.as_ref(), env, msg).unwrap()).unwrap();

        assert_eq!(active_auction, result)
    }

    #[test]
    fn test_query_auction_by_user() {
        let contract_funds = Coin::new(100_000_000, "udsm");
        let mut deps = mock_dependencies_with_custom_querier(&[contract_funds]);
        let info = mock_info("test_addr", &[]);
        let env = mock_env();

        let auction = Auction::new(
            "sender_dtag".to_string(),
            Uint128::new(100),
            Uint64::new(50),
            None,
            None,
            AuctionStatus::Inactive,
            info.sender.clone(),
        );

        setup_test(deps.as_mut(), env.clone(), info.clone(), "contract_dtag");
        save_auction(deps.as_mut(), info.sender.clone(), auction.clone());

        let msg = QueryMsg::GetAuctionByUser {
            user: info.sender.clone(),
        };
        let result: Auction = from_binary(&query(deps.as_ref(), env, msg).unwrap()).unwrap();

        assert_eq!(auction, result)
    }
}
