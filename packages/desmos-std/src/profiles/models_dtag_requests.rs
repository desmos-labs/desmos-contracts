use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DtagTransferRequest {
    pub dtag_to_trade: String,
    pub sender: Addr,
    pub receiver: Addr,
}
