use cosmwasm_std::StdError;
use thiserror::Error;
use cw_controllers::AdminError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Instantiate POAP error")]
    InstantiatePOAPError {},

    #[error("{0}")]
    Admin(#[from] AdminError),
}
