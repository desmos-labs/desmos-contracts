use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{profiles::query::ProfilesQuery, subspaces::query::SubspacesQuery};

// Use the serde `rename_all` tag in order to produce the following json file structure
// ## Example
// {
//      "route": "profiles",
//      "query_data": {
//          "method": {}
//      }
// }
// Reference: https://serde.rs/enum-representations.html#adjacently-tagged
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "route", content = "query_data")]
pub enum DesmosQuery {
    Profiles(ProfilesQuery),
    Subspaces(SubspacesQuery),
}

impl CustomQuery for DesmosQuery {}

impl From<ProfilesQuery> for DesmosQuery {
    fn from(query: ProfilesQuery) -> Self {
        Self::Profiles(query)
    }
}

impl From<SubspacesQuery> for DesmosQuery {
    fn from(query: SubspacesQuery) -> Self {
        Self::Subspaces(query)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        profiles::query::ProfilesQuery, query::DesmosQuery, subspaces::query::SubspacesQuery,
    };
    use cosmwasm_std::Addr;

    #[test]
    fn test_from_profiles_msg() {
        let query = ProfilesQuery::Profile {
            user: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
        };
        let expected = DesmosQuery::Profiles(query.clone());
        assert_eq!(expected, DesmosQuery::from(query));
    }

    #[test]
    fn test_from_subspaces_msg() {
        let query = SubspacesQuery::Subspaces {
            pagination: Default::default(),
        };
        let expected = DesmosQuery::Subspaces(query.clone());
        assert_eq!(expected, DesmosQuery::from(query));
    }
}
