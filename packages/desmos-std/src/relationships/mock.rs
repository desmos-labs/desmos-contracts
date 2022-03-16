use crate::relationships::{
    models::{Relationship, UserBlock},
    models_query::{QueryBlocksResponse, QueryRelationshipsResponse},
    query::RelationshipsQuery,
};
use cosmwasm_std::{to_binary, Addr, Binary, ContractResult, Uint64};

/**
This file contains some useful mocks of the Desmos x/relationships module's types ready made to be used
in any test
 **/

pub struct MockRelationshipsQueries {}

impl MockRelationshipsQueries {
    pub fn get_mock_relationship() -> Relationship {
        Relationship {
            creator: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            counterparty: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
            subspace_id: Uint64::new(1),
        }
    }

    pub fn get_mock_user_block() -> UserBlock {
        UserBlock {
            blocker: Addr::unchecked("desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc"),
            blocked: Addr::unchecked("desmos1rfv0f7mx7w9d3jv3h803u38vqym9ygg344asm3"),
            reason: "test".to_string(),
            subspace_id: Uint64::new(1),
        }
    }
}

pub struct MockRelationshipsQuerier {}

impl MockRelationshipsQuerier {
    pub fn query(query: &RelationshipsQuery) -> ContractResult<Binary> {
        let response = match query {
            RelationshipsQuery::Relationships { .. } => {
                let relationship = MockRelationshipsQueries::get_mock_relationship();
                to_binary(&QueryRelationshipsResponse {
                    relationships: vec![relationship],
                    pagination: Default::default(),
                })
            }
            RelationshipsQuery::Blocks { .. } => {
                let block = MockRelationshipsQueries::get_mock_user_block();
                to_binary(&QueryBlocksResponse {
                    blocks: vec![block],
                    pagination: Default::default(),
                })
            }
        };
        response.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::relationships::{
        mock::{MockRelationshipsQuerier, MockRelationshipsQueries},
        models_query::{QueryBlocksResponse, QueryRelationshipsResponse},
        query::RelationshipsQuery,
    };
    use cosmwasm_std::{to_binary, Addr, Uint64};

    #[test]
    fn test_query_relationships() {
        let query = RelationshipsQuery::Relationships {
            user: Some(Addr::unchecked("")),
            counterparty: Some(Addr::unchecked("")),
            subspace_id: Uint64::new(1),
            pagination: Default::default(),
        };
        let response = MockRelationshipsQuerier::query(&query);
        let expected = to_binary(&QueryRelationshipsResponse {
            relationships: vec![MockRelationshipsQueries::get_mock_relationship()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }

    #[test]
    fn test_query_blocks() {
        let query = RelationshipsQuery::Blocks {
            blocker: Some(Addr::unchecked("")),
            blocked: Some(Addr::unchecked("")),
            subspace_id: Uint64::new(1),
            pagination: Default::default(),
        };
        let response = MockRelationshipsQuerier::query(&query);
        let expected = to_binary(&QueryBlocksResponse {
            blocks: vec![MockRelationshipsQueries::get_mock_user_block()],
            pagination: Default::default(),
        });
        assert_eq!(response.into_result().ok(), expected.ok())
    }
}
