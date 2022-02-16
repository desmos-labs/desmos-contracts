use cosmwasm_std::{Addr, Timestamp};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Profile {
    account: Addr,
    dtag: String,
    nickname: Option<String>,
    bio: Option<String>,
    pictures: Option<Pictures>,
    creation_date: Timestamp
}

impl Profile {
    pub fn new(
        account: Addr,
        dtag: String,
        nickname: Option<String>,
        bio: Option<String>,
        pictures: Option<Pictures>,
        creation_date: Timestamp
    ) -> Self {
        Profile{
            account,
            dtag,
            nickname,
            bio,
            pictures,
            creation_date
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Pictures {
    profile: Option<String>,
    cover: Option<String>
}

impl Pictures {
    pub fn new(profile: Option<String>, cover: Option<String>) -> Self {
        Pictures{ profile, cover }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryProfileResponse {
    profile: Profile
}
