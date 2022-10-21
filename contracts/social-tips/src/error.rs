use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("The fund field is empty")]
    EmptyTipAmount {},

    #[error("Invalid application")]
    InvalidApplication {},

    #[error("Invalid user handler")]
    InvalidUserHandler {},

    #[error("No tips available for user with address: {user}")]
    NoTipsAvailable { user: String },

    #[error("To many owners")]
    ToManyOwners {},
}
