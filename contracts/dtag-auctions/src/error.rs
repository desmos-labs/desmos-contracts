use cosmwasm_std::{Addr, StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Auction not found")]
    AuctionNotFound {},

    #[error("An auction has already been started by this user")]
    AlreadyExistentAuction {creator: Addr},

    #[error("The auction is expired")]
    ExpiredAuction {},

    #[error("The auction has already been activated")]
    AlreadyActivatedAuction{},

    #[error("The auction dTag claiming period is still ongoing")]
    StillInClaimingPeriodAuctionDTag {},

    #[error("The auction is still active and cant be closed now")]
    StillActiveAuction{},

    #[error("The sender is not the creator of the auction")]
    InvalidAuctionCreator{user: Addr, creator: Addr},

    #[error("Bid not found for user")]
    BidNotFoundForUser {user: Addr},

    #[error("Unknown dTag transfer status")]
    UnknownDTagTransferStatus {status: String},

    #[error("Minimum bid amount not satisfied")]
    MinimumBidAmountNotSatisfied {min_amount: Uint128},

    #[error("No pending auctions left")]
    NoPendingAuctionsLeft {},

    #[error("Max participants number reached")]
    MaxParticipantsNumberReached { max_participants: u64},

    #[error("Bid not found for user")]
    BidNotFound{bidder: Addr}
}
