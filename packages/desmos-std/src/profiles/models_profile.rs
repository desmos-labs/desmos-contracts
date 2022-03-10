use crate::profiles::models_common::PubKey;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Profile {
    pub account: Account,
    pub dtag: String,
    pub nickname: String,
    pub bio: String,
    pub pictures: Pictures,
    pub creation_date: String,
}

impl Profile {
    pub fn new(
        account: Account,
        dtag: String,
        nickname: String,
        bio: String,
        pictures: Pictures,
        creation_date: String,
    ) -> Self {
        Profile {
            account,
            dtag,
            nickname,
            bio,
            pictures,
            creation_date,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Account {
    #[serde(rename = "@type")]
    pub proto_type: String,
    pub address: Addr,
    pub pub_key: PubKey,
    pub account_number: Uint64,
    pub sequence: Uint64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Pictures {
    pub profile: String,
    pub cover: String,
}

impl Pictures {
    pub fn new(profile: String, cover: String) -> Self {
        Pictures { profile, cover }
    }
}
