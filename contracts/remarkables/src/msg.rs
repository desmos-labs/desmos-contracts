use crate::ContractError;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin, Uint64};
use cw721::{AllNftInfoResponse, TokensResponse};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use cw721_remarkables::Metadata;
use url::Url;

#[cw_serde]
pub struct InstantiateMsg {
    /// Address of who will have the right to administer the contract.
    pub admin: String,
    /// Id of the CW721 contract to initialize together with this contract.
    pub cw721_code_id: Uint64,
    /// Initialization message that will be sent to the CW721 contract.
    pub cw721_instantiate_msg: Cw721InstantiateMsg,
    /// Id of the subspace to operate.
    pub subspace_id: Uint64,
    /// List of rarities to initialize with this contract.
    pub rarities: Vec<Rarity>,
}

impl InstantiateMsg {
    /// Checks that the data inside the message are coherent.
    /// NOTE: This function don't checks if the address are valid.
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.subspace_id.is_zero() {
            return Err(ContractError::InvalidSubspaceId {});
        }
        if self.cw721_code_id.is_zero() {
            return Err(ContractError::InvalidCw721CodeId {});
        }
        if self.rarities.is_empty() {
            return Err(ContractError::EmptyRarities {});
        }
        Ok(())
    }
}

#[cw_serde]
pub struct Rarity {
    /// Threshold of the reactions amount to mint.
    pub engagement_threshold: u32,
    /// Mint fees associated with the rarity
    pub mint_fees: Vec<Coin>,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Message allowing the user to mint a Remarkables for a specific post owned by the user.
    Mint {
        post_id: Uint64,
        remarkables_uri: String,
        rarity_level: u32,
    },
    /// Message allowing the contract administrator to update the mint fees of the given rarity level.
    UpdateRarityMintFees {
        rarity_level: u32,
        new_fees: Vec<Coin>,
    },
    /// Message allowing the contract's admin to transfer the admin rights to another user.
    UpdateAdmin { new_admin: String },
    /// Message allowing the contract's admin to claim fees in this contract.
    ClaimFees { receiver: String },
}

impl ExecuteMsg {
    /// Checks that the data inside the message are coherent.
    /// NOTE: This function don't checks if the address are valid.
    pub fn validate(&self) -> Result<(), ContractError> {
        match &self {
            ExecuteMsg::Mint {
                remarkables_uri,
                post_id,
                ..
            } => {
                if post_id.is_zero() {
                    return Err(ContractError::InvalidPostId {});
                }
                // Check that the remarkable uri is a valid IPFS url
                let uri = Url::parse(remarkables_uri)
                    .map_err(|_err| ContractError::InvalidRemarkablesUri {})?;
                if uri.scheme() != "ipfs" {
                    return Err(ContractError::InvalidRemarkablesUri {});
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Returns the configuration info as a [`QueryConfigResponse`].
    #[returns(QueryConfigResponse)]
    Config {},
    // Returns all the rarities info as a [`QueryRaritiesResponse`].
    #[returns(QueryRaritiesResponse)]
    Rarities {},
    /// Returns the nft info with approvals from cw721 contract as a [`AllNftInfoResponse`].
    #[returns(AllNftInfoResponse<Metadata>)]
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Returns all the tokens ids owned by the given owner from cw721 contract as a [`TokensResponse`].
    #[returns(TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

/// Response to [`QueryMsg::Config`].
#[cw_serde]
pub struct QueryConfigResponse {
    /// Address of the contract administrator.
    pub admin: Addr,
    /// Id of the cw721 contract that this contract has initialized.
    pub cw721_code_id: Uint64,
    /// Address of the cw721 contract that this contract is using to mint.
    pub cw721_address: Addr,
    /// Id of the subspace to operate.
    pub subspace_id: Uint64,
}

/// Response to [`QueryMsg::Rarities`].
#[cw_serde]
pub struct QueryRaritiesResponse {
    /// List of rarities state in this contract.
    pub rarities: Vec<Rarity>,
}

#[cfg(test)]
mod tests {
    use super::*;
    mod instantiate {
        use super::*;
        #[test]
        fn instantiate_msg_with_invalid_subspace_id_error() {
            let msg = InstantiateMsg {
                admin: "admin".into(),
                cw721_code_id: 1u64.into(),
                cw721_instantiate_msg: Cw721InstantiateMsg {
                    name: "".to_string(),
                    minter: "".to_string(),
                    symbol: "".to_string(),
                },
                subspace_id: 0u64.into(),
                rarities: vec![],
            };
            assert_eq!(
                ContractError::InvalidSubspaceId {},
                msg.validate().unwrap_err()
            )
        }
        #[test]
        fn instantiate_msg_with_cw721_code_id_error() {
            let msg = InstantiateMsg {
                admin: "admin".into(),
                cw721_code_id: 0u64.into(),
                cw721_instantiate_msg: Cw721InstantiateMsg {
                    name: "".to_string(),
                    minter: "".to_string(),
                    symbol: "".to_string(),
                },
                subspace_id: 1u64.into(),
                rarities: vec![],
            };
            assert_eq!(
                ContractError::InvalidCw721CodeId {},
                msg.validate().unwrap_err()
            )
        }
        #[test]
        fn instantiate_msg_with_empty_rarities_error() {
            let msg = InstantiateMsg {
                admin: "admin".into(),
                cw721_code_id: 1u64.into(),
                cw721_instantiate_msg: Cw721InstantiateMsg {
                    name: "".to_string(),
                    minter: "".to_string(),
                    symbol: "".to_string(),
                },
                subspace_id: 1u64.into(),
                rarities: vec![],
            };
            assert_eq!(ContractError::EmptyRarities {}, msg.validate().unwrap_err())
        }
        #[test]
        fn valid_instantiate_msg_no_error() {
            let msg = InstantiateMsg {
                admin: "admin".into(),
                cw721_code_id: 1u64.into(),
                cw721_instantiate_msg: Cw721InstantiateMsg {
                    name: "".to_string(),
                    minter: "".to_string(),
                    symbol: "".to_string(),
                },
                subspace_id: 1u64.into(),
                rarities: vec![Rarity {
                    engagement_threshold: 100,
                    mint_fees: vec![],
                }],
            };
            msg.validate().unwrap()
        }
    }
    mod execute_msg {
        use super::*;
        #[test]
        fn mint_msg_without_valid_uri_error() {
            let msg = ExecuteMsg::Mint {
                post_id: 1u64.into(),
                rarity_level: 1,
                remarkables_uri: "".into(),
            };
            assert_eq!(
                msg.validate().unwrap_err(),
                ContractError::InvalidRemarkablesUri {}
            )
        }
        #[test]
        fn mint_msg_with_invalid_uri_schema_error() {
            let msg = ExecuteMsg::Mint {
                post_id: 1u64.into(),
                rarity_level: 1,
                remarkables_uri: "https://remarkables.com".into(),
            };
            assert_eq!(
                msg.validate().unwrap_err(),
                ContractError::InvalidRemarkablesUri {}
            )
        }
        #[test]
        fn mint_msg_with_invalid_post_id_error() {
            let msg = ExecuteMsg::Mint {
                post_id: 0u64.into(),
                rarity_level: 1,
                remarkables_uri: "https://remarkables.com".into(),
            };
            assert_eq!(msg.validate().unwrap_err(), ContractError::InvalidPostId {})
        }
        #[test]
        fn mint_msg_with_valid_uri_schema_no_error() {
            let msg = ExecuteMsg::Mint {
                post_id: 1u64.into(),
                rarity_level: 1,
                remarkables_uri: "ipfs://remarkables.com".into(),
            };
            msg.validate().unwrap()
        }
        #[test]
        fn other_msgs_no_error() {
            let msg = ExecuteMsg::UpdateAdmin {
                new_admin: "new_admin".into(),
            };
            msg.validate().unwrap()
        }
    }
}
