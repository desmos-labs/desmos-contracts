use crate::subspaces::msg::SubspacesMsg;
use cosmwasm_std::{Addr, Uint64};

pub struct SubspacesMsgBuilder;

impl SubspacesMsgBuilder {
    pub fn new() -> Self {
        SubspacesMsgBuilder {}
    }
}

impl Default for SubspacesMsgBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SubspacesMsgBuilder {
    pub fn create_subspace(
        &self,
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        creator: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::CreateSubspace {
            name,
            description,
            treasury,
            owner,
            creator,
        }
    }

    pub fn edit_subspace(
        &self,
        name: String,
        description: String,
        treasury: Addr,
        owner: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::EditSubspace {
            name,
            description,
            treasury,
            owner,
            signer,
        }
    }

    pub fn delete_subspace(&self, subspace_id: Uint64, signer: Addr) -> SubspacesMsg {
        SubspacesMsg::DeleteSubspace {
            subspace_id,
            signer,
        }
    }

    pub fn create_user_group(
        &self,
        subspace_id: Uint64,
        name: String,
        description: String,
        default_permissions: u32,
        creator: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::CreateUserGroup {
            subspace_id,
            name,
            description,
            default_permissions,
            creator,
        }
    }

    pub fn edit_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        name: String,
        description: String,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::EditUserGroup {
            subspace_id,
            group_id,
            name,
            description,
            signer,
        }
    }

    pub fn set_user_group_permissions(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        permissions: u32,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::SetUserGroupPermissions {
            subspace_id,
            group_id,
            permissions,
            signer,
        }
    }

    pub fn delete_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::DeleteUserGroup {
            subspace_id,
            group_id,
            signer,
        }
    }

    pub fn add_user_to_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::AddUserToUserGroup {
            subspace_id,
            group_id,
            user,
            signer,
        }
    }

    pub fn remove_user_from_user_group(
        &self,
        subspace_id: Uint64,
        group_id: u32,
        user: Addr,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::RemoveUserFromUserGroup {
            subspace_id,
            group_id,
            user,
            signer,
        }
    }

    pub fn set_user_permissions(
        &self,
        subspace_id: Uint64,
        user: Addr,
        permissions: u32,
        signer: Addr,
    ) -> SubspacesMsg {
        SubspacesMsg::SetUserPermissions {
            subspace_id,
            user,
            permissions,
            signer,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_subspace() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.create_subspace(
            "test".to_string(),
            "test".to_string(),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::CreateSubspace {
            name: "test".to_string(),
            description: "test".to_string(),
            treasury: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            owner: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_edit_subspace() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.edit_subspace(
            "test".to_string(),
            "test".to_string(),
            Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::EditSubspace {
            name: "test".to_string(),
            description: "test".to_string(),
            treasury: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            owner: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_delete_subspace() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.delete_subspace(
            Uint64::new(1),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::DeleteSubspace {
            subspace_id: Uint64::new(1),
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_create_user_group() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.create_user_group(
            Uint64::new(1),
            "test".to_string(),
            "test".to_string(),
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::CreateUserGroup {
            subspace_id: Uint64::new(1),
            name: "test".to_string(),
            description: "test".to_string(),
            default_permissions: 1,
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_edit_user_group() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.edit_user_group(
            Uint64::new(1),
            1,
            "test".to_string(),
            "test".to_string(),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::EditUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            name: "test".to_string(),
            description: "test".to_string(),
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_set_user_group_permissions() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.set_user_group_permissions(
            Uint64::new(1),
            1,
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::SetUserGroupPermissions {
            subspace_id: Uint64::new(1),
            group_id: 1,
            permissions: 1,
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_delete_user_group() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.delete_user_group(
            Uint64::new(1),
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        );
        let expected = SubspacesMsg::DeleteUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            signer: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_add_user_to_user_group() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.add_user_to_user_group(
            Uint64::new(1),
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        );
        let expected = SubspacesMsg::AddUserToUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            user: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            signer: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_remove_user_to_user_group() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.remove_user_from_user_group(
            Uint64::new(1),
            1,
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        );
        let expected = SubspacesMsg::RemoveUserFromUserGroup {
            subspace_id: Uint64::new(1),
            group_id: 1,
            user: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            signer: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        };
        assert_eq!(msg, expected)
    }

    #[test]
    fn test_set_user_permissions() {
        let builder = SubspacesMsgBuilder::default();
        let msg = builder.set_user_permissions(
            Uint64::new(1),
            Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            1,
            Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        );
        let expected = SubspacesMsg::SetUserPermissions {
            subspace_id: Uint64::new(1),
            user: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            permissions: 1,
            signer: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
        };
        assert_eq!(msg, expected)
    }
}
