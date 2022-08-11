use cosmwasm_std::{StdError, Timestamp};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Instantiate cw721 error")]
    InstantiateCw721Error {},

    #[error("The start time ({start}) is after the end time ({end})")]
    StartTimeAfterEndTime { start: Timestamp, end: Timestamp },

    #[error("Event start time is before current time: {current_time} start: {start_time}")]
    StartTimeBeforeCurrentTime {
        current_time: Timestamp,
        start_time: Timestamp,
    },

    #[error("Event end time is before current time: {current_time} end: {end_time}")]
    EndTimeBeforeCurrentTime {
        current_time: Timestamp,
        end_time: Timestamp,
    },

    #[error("Invalid poap URI (must be an IPFS URI)")]
    InvalidPoapUri {},

    #[error("Invalid per address limit value")]
    InvalidPerAddressLimit {},

    #[error("Mint operation is disabled")]
    MintDisabled {},

    #[error("Minting limit reached for {recipient_addr}")]
    MaxPerAddressLimitExceeded { recipient_addr: String },

    #[error("Event started, current time: {current_time}, start: {start_time}")]
    EventStarted {
        current_time: Timestamp,
        start_time: Timestamp,
    },

    #[error("Event not started, current time: {current_time}, start time: {start_time}")]
    EventNotStarted {
        current_time: Timestamp,
        start_time: Timestamp,
    },

    #[error("Event terminated, current time: {current_time} end time: {end_time}")]
    EventTerminated {
        current_time: Timestamp,
        end_time: Timestamp,
    },
}
