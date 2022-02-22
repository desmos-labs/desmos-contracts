use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::types::{PageResponse};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DtagTransferRequest {
    pub dtag_to_trade: String,
    pub sender: Addr,
    pub receiver: Addr,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryIncomingDtagTransferRequestResponse {
    pub requests: Vec<DtagTransferRequest>,
    pub pagination: PageResponse
}
