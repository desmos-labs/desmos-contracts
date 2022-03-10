use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum SubspacesMsg {
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
        subspace_id: Uint64,
        signer: Addr,
    },
    CreateUserGroup {
        subspace_id: Uint64,
        name: String,
        description: String,
        default_permissions: u32,
        creator: Addr,
    },
    EditUserGroup {
        subspace_id: Uint64,
        group_id: u32,
        name: String,
        description: String,
        signer: Addr,
    },
    SetUserGroupPermissions {
        subspace_id: Uint64,
        group_id: u32,
        permissions: u32,
        signer: Addr,
    },
    DeleteUserGroup {
        subspace_id: Uint64,
        group_id: u32,
        signer: Addr,
    },
    AddUserToUserGroup {
        subspace_id: Uint64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    },
    RemoveUserFromUserGroup {
        subspace_id: Uint64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    },
    SetUserPermissions {
        subspace_id: Uint64,
        user: Addr,
        permissions: u32,
        signer: Addr,
    },
}
