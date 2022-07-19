use cosmwasm_std::{Addr, Timestamp};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Address of who will have the right to administer the contract.
    pub admin: String,
    /// Address of who can call the [`ExecuteMsg::MintTo`] other then the admin.
    pub minter: String,
    /// Id of the CW721 contract to initialize together with this contract.
    pub cw721_code_id: u64,
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
    /// Uri
    pub event_uri: String,
    pub cw721_code_id: u64,
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
    pub cw721_contract_code: u64,
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
    /// Time at witch the event ends.
    pub end_time: Timestamp,
    /// IPFS uri where is stored the event's metadata.
    pub event_uri: String,
}
