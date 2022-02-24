use crate::types::PageResponse;
use cosmwasm_std::Addr;
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
pub struct OracleRequest {
    pub id: String,
    pub oracle_script_id: String,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryApplicationLinksResponse {
    pub links: Vec<ApplicationLink>,
    pub pagination: PageResponse,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryUserApplicationLinkResponse {
    pub link: ApplicationLink,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryApplicationLinkByClientIDResponse {
    pub link: ApplicationLink,
}
