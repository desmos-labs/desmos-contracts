use cosmwasm_std::StdError;
use thiserror::Error;
use cw_controllers::AdminError;
use cw_utils::ParseReplyError;

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
    AdminError(#[from] AdminError),

    #[error("{0}")]
    ParseReplyError(#[from] ParseReplyError),

    #[error("No eligibility error")]
    NoEligibilityError{}
}
