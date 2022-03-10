use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ApplicationLink {
    pub user: Addr,
    pub data: Data,
    pub state: String,
    pub oracle_request: OracleRequest,
    pub result: AppLinkResult,
    pub creation_time: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Data {
    pub application: String,
    pub username: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct TimeoutHeight {
    pub revision_number: Uint64,
    pub revision_height: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct OracleRequest {
    pub id: Uint64,
    pub oracle_script_id: Uint64,
    pub call_data: CallData,
    pub client_id: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct CallData {
    pub application: String,
    pub call_data: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AppLinkResult {
    Success { value: String, signature: String },
    Failed { error: String },
}
