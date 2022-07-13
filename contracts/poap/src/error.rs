use cosmwasm_std::{StdError, Timestamp};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Instantiate cw721 error")]
    InstantiateCw721Error {},

    #[error("The start time ({0}) is after the end time ({1})")]
    StartTimeAfterEndTime {
        start: Timestamp,
        end: Timestamp
    },

    #[error("The end_time is already passed {0}")]
    EndTimeAlreadyPassed {
        end: Timestamp
    },

    #[error("Invalid base poap URI (must be an IPFS URI)")]
    InvalidPoapUri {},

    #[error("Invalid event URI")]
    InvalidEventUri {},
}
