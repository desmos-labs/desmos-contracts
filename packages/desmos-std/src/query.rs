use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{profiles::query_router::ProfilesQuery, types::DesmosRoute};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQuery {
    pub route: DesmosRoute,
    pub query_data: DesmosQueryRoute,
}

impl CustomQuery for DesmosQuery {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DesmosQueryRoute {
    Profiles(ProfilesQuery),
}

impl From<ProfilesQuery> for DesmosQuery {
    fn from(query: ProfilesQuery) -> Self {
        Self {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(query),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        profiles::query_router::ProfilesQuery,
        query::{DesmosQuery, DesmosQueryRoute},
        types::DesmosRoute,
    };
    use cosmwasm_std::Addr;

    #[test]
    fn test_from_profiles_msg() {
        let query = ProfilesQuery::Profile {
            user: Addr::unchecked("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2"),
        };
        let expected = DesmosQuery {
            route: DesmosRoute::Profiles,
            query_data: DesmosQueryRoute::Profiles(query.clone()),
        };
        assert_eq!(expected, DesmosQuery::from(query));
    }
}
