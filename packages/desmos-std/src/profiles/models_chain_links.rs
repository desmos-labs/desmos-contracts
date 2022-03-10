use crate::profiles::models_common::PubKey;
use cosmwasm_std::Addr;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainLink {
    pub user: Addr,
    pub address: ChainLinkAddr,
    pub proof: Proof,
    pub chain_config: ChainConfig,
    pub creation_time: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainLinkAddr {
    #[serde(rename = "@type")]
    pub proto_type: String,
    pub value: String,
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Proof {
    pub pub_key: PubKey,
    pub signature: Signature,
    pub plain_text: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Signature {
    #[serde(rename = "@type")]
    pub proto_type: String,
    pub mode: String,
    pub signature: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct ChainConfig {
    pub name: String,
}
