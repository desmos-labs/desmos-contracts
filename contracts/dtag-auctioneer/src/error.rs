use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("An auction has already been started by this contract")]
    AlreadyExistentAuction {},

    #[error("The auction is closed")]
    AlreadyClosedAuction{},

    #[error("Already activated auction by sender")]
    AlreadyActivatedAuction{},

    #[error("The auction dtag is still in the claiming procedure")]
    StillInClaimingProcedureAuctionDTag{},

    #[error("The auction is still active and cant be closed now")]
    StillActiveAuction{},

    #[error("Auction not found")]
    AuctionNotFound {},

    #[error("Dtag request already present in store")]
    AlreadyExistentDtagRequest {},

    #[error("User is not the creator of the auction")]
    InvalidAuctionCreator{},

    #[error("Offer not found")]
    OfferNotFound {},

    #[error("Unknown dtag transfer status")]
    UnknownDTagTransferStatus {},

    #[error("Offer doesn't match the minimum starting price")]
    MinimumPriceNotReached {},

    #[error("No pending auctions left")]
    NoPendingAuctionsLeft {},

    #[error("Max participants number reached")]
    MaxParticipantsNumberReached {},
}
