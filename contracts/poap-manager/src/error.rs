use cosmwasm_std::{StdError, Addr};
use cw_utils::ParseReplyError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Invalid POAP code ID")]
    InvalidPOAPCodeID {},

    #[error("Instantiate POAP contract error")]
    InstantiatePOAPError {},

    #[error("Caller is not admin: {caller}")]
    NotAdmin { caller: Addr },

    #[error("{0}")]
    ParseReplyError(#[from] ParseReplyError),

    #[error("No eligibility error")]
    NoEligibilityError {},
}
