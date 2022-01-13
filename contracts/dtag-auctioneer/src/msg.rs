use cosmwasm_std::{Addr, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {
    /// The dtag with which this contract will be initialized, since there will be more than one
    pub contract_dtag: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateAuction {
        dtag: String,
        starting_price: Uint128,
        max_participants: Uint64,
    },
    MakeOffer {},
    RetreatOffer {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    UpdateDTagAuctionStatus { user: Addr, transfer_status: String },
    CompleteAuction {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// GetFilteredPosts returns a list of filtered posts where each post has been reported at most (reports_limit - 1) time
    GetActiveAuction {},
    GetAuctionByUser {
        user: Addr,
    },
}
