use cosmwasm_std::{StdError, Uint128};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("unauthorized")]
    Unauthorized {},

    #[error("invalid subspace id")]
    InvalidSubspaceId {},

    #[error("subspace with id {id} does not exist, {error}")]
    SubspaceNotExist { id: u64, error: StdError },

    #[error("invalid post id")]
    InvalidPostId {},

    #[error("can't tip yourself")]
    SenderEqReceiver {},

    #[error("provided a fee coin with value = 0, denom: {denom}")]
    ZeroFeeCoin { denom: String },

    #[error("empty fixed fee")]
    EmptyFixedFee {},

    #[error("invalid percentage fee")]
    InvalidPercentageFee {},

    #[error("insufficient {denom}, requested: {requested} provided: {provided}")]
    InsufficientAmount {
        denom: String,
        requested: Uint128,
        provided: Uint128,
    },

    #[error("founds message field is empty")]
    EmptyFounds {},

    #[error("block index overflow")]
    BlockIndexOverflow {},
}
