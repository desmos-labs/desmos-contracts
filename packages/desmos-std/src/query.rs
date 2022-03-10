use cosmwasm_std::CustomQuery;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::subspaces::query::SubspacesQuery;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case", tag = "route", content = "query_data")]
pub enum DesmosQuery {
    Subspaces(SubspacesQuery),
}

impl CustomQuery for DesmosQuery {}

impl From<SubspacesQuery> for DesmosQuery {
    fn from(query: SubspacesQuery) -> Self {
        Self::Subspaces(query)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_subspaces_msg() {
        let query = SubspacesQuery::Subspaces {
            pagination: Default::default(),
        };
        let expected = DesmosQuery::Subspaces(query.clone());
        assert_eq!(expected, DesmosQuery::from(query));
    }
}
