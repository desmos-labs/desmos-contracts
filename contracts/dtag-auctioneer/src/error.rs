use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Dtag request already present in store")]
    AlreadyStoredDtagRequest {},

    #[error("Dtag auction record not found")]
    DtagAuctionRecordNotFound {}

}
