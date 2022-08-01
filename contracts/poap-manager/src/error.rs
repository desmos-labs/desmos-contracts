use cosmwasm_std::StdError;
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid reply ID")]
    InvalidReplyID,

    #[error("Invalid POAP code ID")]
    InvalidPOAPCodeID,

    #[error("Instantiate POAP error")]
    InstantiatePOAPError,

    #[error("Caller is not admin")]
    NotAdmin,

    #[error("{0}")]
    ParseReplyError(#[from] ParseReplyError),

    #[error("No eligibility error")]
    NoEligibilityError,
}
