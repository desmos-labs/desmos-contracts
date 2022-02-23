use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Subspace {
    pub id : u64,
    pub name : String,
    #[serde(default)]
    pub description : String,
    #[serde(default)]
    pub treasury : String,
    pub owner : String,
    pub creator : String,
    pub creation_time : String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserGroup {
    pub subspace_id : u64,
    pub id : u32,
    pub name : String,
    #[serde(default)]
    pub description : String,
    pub permissions : u32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum PermissionDetail {
    User(UserPermission),
    Group(GroupPermission)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserPermission {
    pub user : String,
    pub permission : u32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GroupPermission {
    pub group_id : u32,
    pub permission : u32
}