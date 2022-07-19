use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use poap::msg::InstantiateMsg as POAPInstantiateMsg;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub poap_code_id: u64,
    pub poap_instantiate_msg: POAPInstantiateMsg,
    pub subspace_id: u64,
    pub event_post_id: u64
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg{
    Claim{post_id: u64},
    MintTo{recipient: String},
    UpdateAdmin{new_admin: String}
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryMsg {
    /// Return a ConfigResponse containing the configuration info of the Manager contract
    Config{},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryConfigResponse {}
