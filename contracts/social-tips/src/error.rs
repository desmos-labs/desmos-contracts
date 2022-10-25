use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid max pending tips value: {value}, the value must be > 0 and les then {max}")]
    InvalidMaxPendingTipsValue { value: u32, max: u32 },

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

    #[error("To many pending tips for user with handle: {handle} on application: {application}")]
    ToManyPendingTips { application: String, handle: String },
}
