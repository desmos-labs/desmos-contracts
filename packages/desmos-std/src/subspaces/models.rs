use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Timestamp;
use crate::types::{PageResponse};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Subspace {
    pub id : u64
    pub name : String
    pub description : String
    pub treasury : String
    pub owner : String
    pub creator : String
    pub creation_time : Timestamp
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserGroup {
    pub subspace_id : u64
    pub id : u32
    pub name : String
    pub description : String
    pub permissions : u32
}

// TODO: PermissionDetail
