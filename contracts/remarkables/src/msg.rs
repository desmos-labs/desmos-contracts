use cosmwasm_std::{Addr, Coin, Uint64};
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub cw721_code_id: Uint64,
    pub cw721_instantiate_msg: Cw721InstantiateMsg,
    pub subspace_id: Uint64,
    pub rarities: Vec<Rarity>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Rarity {
    pub level: u32,
    pub engagement_threshold: u32,
    pub mint_fees: Vec<Coin>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    MintTo {
        post_id: Uint64,
        remarkables_uri: String,
        rarity_level: u32,
    },
    UpdateRarityMintFee {
        rarity_level: u32,
        new_fees: Vec<Coin>,
    },
    UpdateAdmin {
        new_admin: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return a QueryConfigResponse containing the configuration info of the contract
    Config {},
    Rarities {},
    /// Returns the nft info with approvals from cw721 contract as a [`AllNftInfoResponse`]
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    /// Returns all the tokens ids owned by the given owner from cw721 contract as a [`TokensResponse`]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryConfigResponse {
    pub admin: Addr,
    pub cw721_code_id: Uint64,
    pub cw721_address: Addr,
    pub subspace_id: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryRaritiesResponse {
    pub rarities: Vec<Rarity>,
}
