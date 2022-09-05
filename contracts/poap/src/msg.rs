use crate::ContractError;
use cosmwasm_std::{Addr, Timestamp, Uint64};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[schemars(rename = "PoapInstantiateMsg", title = "InstantiateMsg")]
pub struct InstantiateMsg {
    /// Address of who will have the right to administer the contract.
    pub admin: String,
    /// Address of who can call the [`ExecuteMsg::MintTo`] other then the admin.
    pub minter: String,
    /// Id of the CW721 contract to initialize together with this contract.
    pub cw721_code_id: Uint64,
    /// Initialization message that will be sent to the CW721 contract.
    pub cw721_instantiate_msg: Cw721InstantiateMsg,
    /// Information about the event.
    pub event_info: EventInfo,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EventInfo {
    /// User that created the event.
    pub creator: String,
    /// Time at which the event begins.
    pub start_time: Timestamp,
    /// Time at which the event ends.
    pub end_time: Timestamp,
    /// Max amount of poap that a single user can mint.
    pub per_address_limit: u32,
    /// Identifies a valid IPFS URI corresponding to where the assets and metadata of the POAPs are stored.
    pub poap_uri: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Allows the contract's admin to enable the [`ExecuteMsg::Mint`].
    EnableMint {},
    /// Allows the contract's admin to disable the [`ExecuteMsg::Mint`].
    DisableMint {},
    /// If the mint is enabled, allow the user to mint the poap by themself.
    /// It's disabled before the start of the event and after the event's end.
    Mint {},
    /// Allows the contract's admin or the minter to mint a POAP for a specific recipient.
    /// It's disabled before the start of the event and after the event's end.
    MintTo { recipient: String },
    /// Message that allows the event's creator to change the time frame of the event
    /// if it's not started or finished.
    UpdateEventInfo {
        start_time: Timestamp,
        end_time: Timestamp,
    },
    /// Allows the contract's admin to transfer the admin rights to another user.
    UpdateAdmin { new_admin: String },
    /// Allows the contract's admin to transfer the minting rights to another user.
    UpdateMinter { new_minter: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns the configuration info as a [`QueryConfigResponse`].
    Config {},
    /// Returns the event info as a [`QueryEventInfoResponse`].
    EventInfo {},
    /// Returns the amount of poaps minted from `user` as [`QueryMintedAmountResponse`].
    MintedAmount { user: String },
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

/// Response to [`QueryMsg::Config`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryConfigResponse {
    /// Address of the contract administrator.
    pub admin: Addr,
    /// Address of the entity that is allowed to use [`ExecuteMsg::MintTo`].
    pub minter: Addr,
    /// Tells if the users can execute the [`ExecuteMsg::Mint`].
    pub mint_enabled: bool,
    /// The maximus number of poap that an user can request.
    pub per_address_limit: u32,
    /// Id of the cw721 contract that this contract has initialized.
    pub cw721_contract_code: Uint64,
    /// Address of the cw721 contract that this contract is using to
    /// mint the poaps.
    pub cw721_contract: Addr,
}

/// Response to [`QueryMsg::EventInfo`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryEventInfoResponse {
    /// Address of who created the event.
    pub creator: Addr,
    /// Time at which the event starts.
    pub start_time: Timestamp,
    /// Time at which the event ends.
    pub end_time: Timestamp,
    /// IPFS uri where the event's metadata are stored
    pub poap_uri: String,
}

/// Response to [`QueryMsg::MintedAmount`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryMintedAmountResponse {
    /// Address for which the request was made.
    pub user: Addr,
    /// Amount of poaps minted from the user.
    pub amount: u32,
}

impl InstantiateMsg {
    /// Checks that the data inside the message are coherent.
    /// NOTE: This function don't checks if the address are valid.
    pub fn validate(&self) -> Result<(), ContractError> {
        // Check that the end time is after the start time
        if self.event_info.start_time >= self.event_info.end_time {
            return Err(ContractError::StartTimeAfterEndTime {
                start: self.event_info.start_time.to_owned(),
                end: self.event_info.end_time.to_owned(),
            });
        }

        // Check per address limit
        if self.event_info.per_address_limit == 0 {
            return Err(ContractError::InvalidPerAddressLimit {});
        }

        // Check that the poap uri is a valid IPFS url
        let poap_uri = Url::parse(&self.event_info.poap_uri)
            .map_err(|_err| ContractError::InvalidPoapUri {})?;
        if poap_uri.scheme() != "ipfs" {
            return Err(ContractError::InvalidPoapUri {});
        }

        Ok(())
    }
}

impl ExecuteMsg {
    /// Checks that the data inside the message are coherent.
    /// NOTE: This function don't checks if the address are valid.
    pub fn validate(&self) -> Result<(), ContractError> {
        match &self {
            ExecuteMsg::UpdateEventInfo {
                start_time,
                end_time,
            } => {
                if start_time >= end_time {
                    Err(ContractError::StartTimeAfterEndTime {
                        start: start_time.to_owned(),
                        end: end_time.to_owned(),
                    })
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::{EventInfo, ExecuteMsg, InstantiateMsg};
    use crate::ContractError;
    use cosmwasm_std::Timestamp;
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;

    #[test]
    fn instantiate_with_start_time_after_end_time_error() {
        let start = Timestamp::from_seconds(2);
        let end = Timestamp::from_seconds(1);
        let msg = InstantiateMsg {
            admin: "".to_string(),
            minter: "".to_string(),
            cw721_code_id: 0u64.into(),
            cw721_instantiate_msg: Cw721InstantiateMsg {
                name: "".to_string(),
                minter: "".to_string(),
                symbol: "".to_string(),
            },
            event_info: EventInfo {
                creator: "".to_string(),
                start_time: start.clone(),
                end_time: end.clone(),
                per_address_limit: 1,
                poap_uri: "ipfs://domain.com".to_string(),
            },
        };

        assert_eq!(
            ContractError::StartTimeAfterEndTime { start, end },
            msg.validate().unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_start_time_equal_end_time_error() {
        let start = Timestamp::from_seconds(1);
        let end = Timestamp::from_seconds(1);
        let msg = InstantiateMsg {
            admin: "".to_string(),
            minter: "".to_string(),
            cw721_code_id: 0u64.into(),
            cw721_instantiate_msg: Cw721InstantiateMsg {
                name: "".to_string(),
                minter: "".to_string(),
                symbol: "".to_string(),
            },
            event_info: EventInfo {
                creator: "".to_string(),
                start_time: start.clone(),
                end_time: end.clone(),
                per_address_limit: 1,
                poap_uri: "ipfs://domain.com".to_string(),
            },
        };

        assert_eq!(
            ContractError::StartTimeAfterEndTime { start, end },
            msg.validate().unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_invalid_per_address_limit_error() {
        let msg = InstantiateMsg {
            admin: "".to_string(),
            minter: "".to_string(),
            cw721_code_id: 0u64.into(),
            cw721_instantiate_msg: Cw721InstantiateMsg {
                name: "".to_string(),
                minter: "".to_string(),
                symbol: "".to_string(),
            },
            event_info: EventInfo {
                creator: "".to_string(),
                start_time: Timestamp::from_seconds(1),
                end_time: Timestamp::from_seconds(2),
                per_address_limit: 0,
                poap_uri: "ipfs://domain.com".to_string(),
            },
        };

        assert_eq!(
            ContractError::InvalidPerAddressLimit {},
            msg.validate().unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_invalid_poap_uri_error() {
        let msg = InstantiateMsg {
            admin: "".to_string(),
            minter: "".to_string(),
            cw721_code_id: 0u64.into(),
            cw721_instantiate_msg: Cw721InstantiateMsg {
                name: "".to_string(),
                minter: "".to_string(),
                symbol: "".to_string(),
            },
            event_info: EventInfo {
                creator: "".to_string(),
                start_time: Timestamp::from_seconds(1),
                end_time: Timestamp::from_seconds(2),
                per_address_limit: 1,
                poap_uri: "invalid_base_poap_uri".to_string(),
            },
        };

        assert_eq!(
            ContractError::InvalidPoapUri {},
            msg.validate().unwrap_err()
        );
    }

    #[test]
    fn instantiate_with_non_ipfs_poap_uri_error() {
        let msg = InstantiateMsg {
            admin: "".to_string(),
            minter: "".to_string(),
            cw721_code_id: 0u64.into(),
            cw721_instantiate_msg: Cw721InstantiateMsg {
                name: "".to_string(),
                minter: "".to_string(),
                symbol: "".to_string(),
            },
            event_info: EventInfo {
                creator: "".to_string(),
                start_time: Timestamp::from_seconds(1),
                end_time: Timestamp::from_seconds(2),
                per_address_limit: 1,
                poap_uri: "https://domain.com".to_string(),
            },
        };

        assert_eq!(
            ContractError::InvalidPoapUri {},
            msg.validate().unwrap_err()
        );
    }

    #[test]
    fn update_event_info_start_time_after_end_time_error() {
        let start = Timestamp::from_seconds(2);
        let end = Timestamp::from_seconds(1);
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: start.clone(),
            end_time: end.clone(),
        };

        assert_eq!(
            ContractError::StartTimeAfterEndTime { start, end },
            msg.validate().unwrap_err()
        );
    }

    #[test]
    fn update_event_info_start_time_equal_end_time_error() {
        let start = Timestamp::from_seconds(1);
        let end = Timestamp::from_seconds(1);
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: start.clone(),
            end_time: end.clone(),
        };

        assert_eq!(
            ContractError::StartTimeAfterEndTime { start, end },
            msg.validate().unwrap_err()
        );
    }
}
