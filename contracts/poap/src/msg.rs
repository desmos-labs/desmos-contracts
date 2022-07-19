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
    /// Admin command.
    EnableMint {},
    /// Admin command.
    DisableMint {},
    /// Can be called from any user, mint must be enabled.
    Mint {},
    /// Can be called from minter or admin, bypass mint enable flag.
    MintTo { recipient: String },
    /// Message that allows the event's creator to change the time frame of the event
    /// if it's not in progress
    UpdateEventInfo {
        start_time: Timestamp,
        end_time: Timestamp,
    },
    /// Admin command.
    UpdateAdmin { new_admin: String },
    /// Admin command.
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryConfigResponse {
    pub admin: Addr,
    pub minter: Addr,
    pub mint_enabled: bool,
    pub per_address_limit: u32,
    pub cw721_contract_code: u64,
    pub cw721_contract: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryEventInfoResponse {
    pub creator: Addr,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub event_uri: String,
}
