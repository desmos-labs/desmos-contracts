use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("Invalid max pending tips value: {value}, the value must be > 0 and les then {max}")]
    InvalidMaxPendingTipsValue { value: u16, max: u16 },

    #[error(
        "Invalid max sent pending tips value: {value}, the value must be > 0 and les then {max}"
    )]
    InvalidMaxSentPendingTipsValue { value: u16, max: u16 },

    #[error("The fund field is empty")]
    EmptyTipAmount {},

    #[error("Invalid application")]
    InvalidApplication {},

    #[error("Invalid user handle")]
    InvalidUserHandle {},

    #[error("No tips available for user with address: {user}")]
    NoTipsAvailable { user: String },

    #[error("To many pending tips for user with handle: {handle} on application: {application}")]
    ToManyPendingTipsForUser { application: String, handle: String },

    #[error("You have to many pending tips, please remove on to send a tip")]
    ToManySentPendingTips {},

    #[error("No pending tip for user with handle: {handle} on application: {application}")]
    NoPendingTip { application: String, handle: String },
}
