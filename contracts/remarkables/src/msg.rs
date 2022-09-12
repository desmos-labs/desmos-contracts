use crate::ContractError;
use cosmwasm_std::{Addr, Coin, Uint64};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub cw721_code_id: Uint64,
    pub cw721_instantiate_msg: Cw721InstantiateMsg,
    pub subspace_id: Uint64,
    pub rarities: Vec<Rarity>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Rarity {
    pub level: u32,
    pub engagement_threshold: u32,
    pub mint_fees: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MintTo {
        post_id: Uint64,
        remarkables_uri: String,
        rarity_level: u32,
    },
    UpdateRarityMintFee {
        rarity_level: u32,
        new_fees: Vec<Coin>,
    },
    UpdateAdmin {
        new_admin: String,
    },
}

impl ExecuteMsg {
    /// Checks that the data inside the message are coherent.
    /// NOTE: This function don't checks if the address are valid.
    pub fn validate(&self) -> Result<(), ContractError> {
        match &self {
            ExecuteMsg::MintTo {
                remarkables_uri, ..
            } => {
                // Check that the poap uri is a valid IPFS url
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return a QueryConfigResponse containing the configuration info of the contract
    Config {},
    Rarities {},
    /// Returns the nft info with approvals from cw721 contract as a [`AllNftInfoResponse`]
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Returns all the tokens ids owned by the given owner from cw721 contract as a [`TokensResponse`]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryConfigResponse {
    pub admin: Addr,
    pub cw721_code_id: Uint64,
    pub cw721_address: Addr,
    pub subspace_id: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryRaritiesResponse {
    pub rarities: Vec<Rarity>,
}

mod tests {
    use super::*;
    #[test]
    fn mint_to_msg_without_valid_uri_error() {
        let msg = ExecuteMsg::MintTo {
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
    fn mint_to_msg_without_valid_uri_schema_error() {
        let msg = ExecuteMsg::MintTo {
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
    fn mint_to_msg_with_valid_uri_schema_no_error() {
        let msg = ExecuteMsg::MintTo {
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
