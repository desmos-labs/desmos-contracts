use cosmwasm_std::{Addr, Uint64};
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
    CreateAuction{
        dtag: String,
        starting_price: Uint64,
        max_participants: Uint64,
    },
    MakeOffer{
        amount: Uint64,
        user: String
    },
    RetreatOffer{
        user: String
    },
    CloseAuctionAndSellDTag{
        user: String
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SudoMsg {
    ActivateAuctionForUser {
        creator:   Addr,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// GetFilteredPosts returns a list of filtered posts where each post has been reported at most (reports_limit - 1) time
    GetPendingAuctions {},
    GetAuctionByUser{user: String},
    GetOffers{},
    GetBestOffer{},
}
