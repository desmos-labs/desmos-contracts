use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use crate::state::AuctionStatus;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    /// The dtag with which this contract will be initialized, since there will be more than one
    pub contract_dtag: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AskMeForDtagTransferRequest {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    UpdateDtagAuctionStatus {
        user:   String,
        status: AuctionStatus
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// GetFilteredPosts returns a list of filtered posts where each post has been reported at most (reports_limit - 1) time
    GetPendingAuctions {},
}
