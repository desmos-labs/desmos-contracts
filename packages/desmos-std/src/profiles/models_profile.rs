use cosmwasm_std::{Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Profile {
    //account: Addr,
    dtag: String,
    nickname: String,
    bio: String,
    pictures: Pictures,
    creation_date: String
}

impl Profile {
    pub fn new(
        //account: Addr,
        dtag: String,
        nickname: String,
        bio: String,
        pictures: Pictures,
        creation_date: String
    ) -> Self {
        Profile{
            //account,
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
    profile: String,
    cover: String
}

impl Pictures {
    pub fn new(profile: String, cover: String) -> Self {
        Pictures{ profile, cover }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct QueryProfileResponse {
    profile: Profile
}
