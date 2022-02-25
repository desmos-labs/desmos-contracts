use crate::subspaces::models::{GroupPermission, PermissionDetail, Subspace, UserGroup};
use cosmwasm_std::{Addr, Binary, ContractResult};

/**
This file contains some useful mocks of the Desmos x/subspaces modules types ready made to be used
in any test
**/

pub struct MockSubspacesQueries {}

impl MockSubspacesQueries {
    pub fn get_mock_subspace() -> Subspace {
        Subspace {
            id: 1,
            name: String::from("Test subspace"),
            description: String::from("Test subspace"),
            treasury: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
            owner: Addr::unchecked("cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t"),
            creator: Addr::unchecked("cosmos18atyyv6zycryhvnhpr2mjxgusdcah6kdpkffq0"),
            creation_time: String::from(),
        }
    }

    pub fn get_mock_user_group() -> UserGroup {
        UserGroup {
            subspace_id: 1,
            name: String::from("Test group"),
            description: String::from("Test group"),
            permissions: 1,
            creator: Addr::unchecked("cosmos1qzskhrcjnkdz2ln4yeafzsdwht8ch08j4wed69"),
        }
    }

    pub fn get_mock_permission_detail() -> PermissionDetails {
        PermissionDetails::Group(GroupPermission {
            group_id: 1,
            permissions: 1,
        })
    }
}
