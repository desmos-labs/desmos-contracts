use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct Subspace {
    #[serde(skip_serializing)]
    pub id : u64,

    #[serde(skip_serializing)]
    pub name : String,

    #[serde(skip_serializing)]
    pub description : String,

    #[serde(skip_serializing)]
    pub treasury : String,

    #[serde(skip_serializing)]
    pub owner : String,

    #[serde(skip_serializing)]
    pub creator : String,
    
    #[serde(skip_serializing)]
    pub creation_time : String
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct UserGroup {
    #[serde(skip_serializing)]
    pub subspace_id : u64,

    #[serde(skip_serializing)]
    pub id : u32,

    #[serde(skip_serializing)]
    pub name : String,

    #[serde(skip_serializing)]
    pub description : String,

    #[serde(skip_serializing)]
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
    #[serde(skip_serializing)]
    pub user : String,

    #[serde(skip_serializing)]
    pub permission : u32
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct GroupPermission {
    #[serde(skip_serializing)]
    pub group_id : u32,

    #[serde(skip_serializing)]
    pub permission : u32
}