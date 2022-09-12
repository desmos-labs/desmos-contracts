use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Invalid remarkables uri")]
    InvalidRemarkablesUri {},

    #[error("Caller is not subspace owner: {caller}")]
    NotSubspaceOwner { caller: Addr },

    #[error("Invalid reply ID")]
    InvalidReplyID {},

    #[error("Instantiate cw721 error")]
    InstantiateCw721Error {},

    #[error("Caller is not admin: {caller}")]
    NotAdmin { caller: Addr },

    #[error("Rarity doesn't exist on the level: {level}")]
    RarityNotExists { level: u32 },

    #[error("Mint fees not enough")]
    MintFeesNotEnough {},

    #[error("No eligibility error")]
    NoEligibilityError {},
}
