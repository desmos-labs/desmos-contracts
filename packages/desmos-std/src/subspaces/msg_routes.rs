use cosmwasm_std::{Addr};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubspacesMsgs {
    CreateSubspace {
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        creator: Addr,
    },
    EditSubspace {
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        signer: Addr,
    },
    DeleteSubspace {
        subspace_id: u64,
        signer: Addr,
    },
    CreateUserGroup {
        subspace_id: u64,
        name: String,
        description: String,
        default_permissions: u32,
        creator: Addr,
    },
    EditUserGroup {
        subspace_id: u64,
        group_id: u32,
        name: String,
        description: String,
        signer: Addr,
    },
    SetUserGroupPermissions {
        subspace_id: u64,
        group_id: u32,
        permissions: u32,
        signer: Addr,
    },
    DeleteUserGroup {
        subspace_id: u64,
        group_id: u32,
        signer: Addr,
    },
    AddUserToUserGroup {
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    },
    RemoveUserFromUserGroup {
        subspace_id: u64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    },
    SetUserPermissions {
        subspace_id: u64,
        user: Addr,
        permissions: u32,
        signer: Addr,
    },
}
