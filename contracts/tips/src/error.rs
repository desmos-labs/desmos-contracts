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

    #[error("invalid percentage fee")]
    InvalidPercentageFee {},

    #[error("invalid post id")]
    InvalidPostId {},

    #[error("insufficient {denom}, requested: {requested} provided: {provided}")]
    InsufficientFee {
        denom: String,
        requested: Uint128,
        provided: Uint128,
    },

    #[error("fee coin {denom} not provided")]
    FeeCoinNotProvided { denom: String },
}
