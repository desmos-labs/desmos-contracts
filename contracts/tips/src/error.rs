use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("invalid tips history size, value: {value} max allowed: {max}")]
    InvalidTipsHistorySize { value: u32, max: u32 },

    #[error("invalid subspace id")]
    InvalidSubspaceId {},

    #[error("subspace with id {id} does not exist, {error}")]
    SubspaceNotExist { id: u64, error: StdError },

    #[error("invalid post id")]
    InvalidPostId {},

    #[error("post with id: {id} not found")]
    PostNotFound { id: u64 },

    #[error("to be able to tip you must have a profile")]
    ProfileRequired {},

    #[error("you can't tip yourself")]
    SenderEqReceiver {},

    #[error("provided a fee coin with value = 0, denom: {denom}")]
    ZeroFeeCoin { denom: String },

    #[error("empty fixed fee")]
    EmptyFixedFee {},

    #[error("invalid percentage fee")]
    InvalidPercentageFee {},

    #[error("insufficient funds, requested: {requested} provided: {provided}")]
    InsufficientFunds { requested: String, provided: String },

    #[error("funds message field is empty")]
    EmptyFunds {},

    #[error("block index overflow")]
    BlockIndexOverflow {},
}
