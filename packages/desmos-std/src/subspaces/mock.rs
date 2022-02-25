use cosmwasm_std::{Addr, Binary, ContractResult};
use crate::subspaces::{
    models::{Subspace, UserGroup, PermissionDetail},
};

/**
This file contains some useful mocks of the Desmos x/subspaces modules types ready made to be used
in any test
**/

pub struct MockSubspacesQueries {}

impl MockSubspacesQueries{
    // pub fn get_mock_subspace() -> Subspace {
    //     Subspace{
    //         id: 1,
    //         name: "Test subspace".to_string(),
    //         description: "Test subspace".to_string(),
    //         treasury: Addr::unchecked(""),
    //         owner: Addr::unchecked(""),
    //         creator: Addr::unchecked(""),
    //     }
    // }
}