use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("An auction has already been started by this contract")]
    PendingAuction{},

    #[error("Dtag request already present in store")]
    AlreadyExistentDtagRequest {},

    #[error("Dtag auction record not found")]
    DtagAuctionRecordNotFound {}

}
