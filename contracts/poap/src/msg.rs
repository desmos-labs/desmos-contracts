use crate::ContractError;
use cosmwasm_std::{Addr, Timestamp, Uint64};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Address of who will have the right to administer the contract.
    pub admin: String,
    /// Address of who can call the [`ExecuteMsg::MintTo`] other then the admin.
    pub minter: String,
    /// Id of the CW721 contract to initialize together with this contract.
    pub cw721_code_id: Uint64,
    /// Initialization message that will be sent to the CW721 contract.
    pub cw721_initiate_msg: Cw721InstantiateMsg,
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
    pub base_poap_uri: String,
    /// Uri of the poap event
    pub event_uri: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Allows the contract's admin to enable the [`ExecuteMsg::Mint`].
    EnableMint {},
    /// Allows the contract's admin to disable the [`ExecuteMsg::Mint`].
    DisableMint {},
    /// If the mint is enabled, allow the user to mint the poap by themself.
    /// It's disabled after the event's end.
    Mint {},
    /// Allows the contract's admin or the minter to mint a POAP for a specific recipient.
    /// It's disabled after the event's end.
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
    pub event_uri: String,
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
        let poap_uri = Url::parse(&self.event_info.base_poap_uri)
            .map_err(|_err| ContractError::InvalidPoapUri {})?;
        if poap_uri.scheme() != "ipfs" {
            return Err(ContractError::InvalidPoapUri {});
        }

        // Check that the event uri is a valid IPFS url
        let event_uri = Url::parse(&self.event_info.event_uri)
            .map_err(|_err| ContractError::InvalidEventUri {})?;
        if event_uri.scheme() != "ipfs" {
            return Err(ContractError::InvalidEventUri {});
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
