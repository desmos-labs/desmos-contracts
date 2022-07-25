use cosmwasm_std::StdError;
use thiserror::Error;
use cw_utils::ParseReplyError;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid message {msg}")]
    InvalidMessage{ msg: String },

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Instantiate POAP error")]
    InstantiatePOAPError {},

    #[error("Caller is not admin")]
    NotAdmin{},

    #[error("{0}")]
    ParseReplyError(#[from] ParseReplyError),

    #[error("No eligibility error")]
    NoEligibilityError{}
}

impl ContractError{
    pub fn invalid_message(msg: impl Into<String>) -> Self {
        Self::InvalidMessage{ msg: msg.into() }
    }
}