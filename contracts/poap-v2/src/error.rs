use cosmwasm_std::StdError;
use cw721_base::ContractError as Cw721BaseContractError;
use cw_ownable::OwnershipError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error(transparent)]
    Std(#[from] StdError),

    #[error(transparent)]
    Ownership(#[from] OwnershipError),

    #[error(transparent)]
    Version(#[from] cw2::VersionError),

    #[error("token_id already claimed")]
    Claimed {},

    #[error("Cannot set approval that is already expired")]
    Expired {},

    #[error("Approval not found for: {spender}")]
    ApprovalNotFound { spender: String },

    #[error("Transfer is not allowed")]
    TransferDisabled {},

    #[error("Mint is not allowed")]
    MintDisabled {},

    #[error("You don't have the permission to mint")]
    MintUnauthorized {},

    #[error("Can't mint, event not started")]
    EventNotStarted {},

    #[error("Can't mint, event terminated")]
    EventTerminated {},

    #[error("The start time time must be lower then the end time")]
    InvalidTimestampValues {},
}

impl From<Cw721BaseContractError> for ContractError {
    fn from(error: Cw721BaseContractError) -> Self {
        match error {
            Cw721BaseContractError::Std(e) => ContractError::Std(e),
            Cw721BaseContractError::Ownership(e) => ContractError::Ownership(e),
            Cw721BaseContractError::Version(e) => ContractError::Version(e),
            Cw721BaseContractError::Claimed {} => ContractError::Claimed {},
            Cw721BaseContractError::Expired {} => ContractError::Expired {},
            Cw721BaseContractError::ApprovalNotFound { spender } => {
                ContractError::ApprovalNotFound { spender }
            }
        }
    }
}
