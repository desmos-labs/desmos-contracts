use crate::{
    query::DesmosQuery,
    relationships::{
        models_query::{QueryBlocksResponse, QueryRelationshipsResponse},
        query::RelationshipsQuery,
    },
    types::PageRequest,
};
use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult, Uint64};

pub struct RelationshipsQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> RelationshipsQuerier<'a> {
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
        }
    }

    pub fn query_relationships(
        &self,
        subspace_id: Uint64,
        user: Option<Addr>,
        counterparty: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryRelationshipsResponse> {
        let request = DesmosQuery::Relationships(RelationshipsQuery::Relationships {
            subspace_id,
            user,
            counterparty,
            pagination,
        });

        let res: QueryRelationshipsResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_blocks(
        &self,
        subspace_id: Uint64,
        blocker: Option<Addr>,
        blocked: Option<Addr>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryBlocksResponse> {
        let request = DesmosQuery::Relationships(RelationshipsQuery::Blocks {
            subspace_id,
            blocker,
            blocked,
            pagination,
        });

        let res: QueryBlocksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mock::mock_dependencies_with_custom_querier,
        relationships::{
            mock::MockRelationshipsQueries,
            models_query::{QueryBlocksResponse, QueryRelationshipsResponse},
            querier::RelationshipsQuerier,
        },
    };
    use cosmwasm_std::{Addr, Uint64};
    use std::ops::Deref;

    #[test]
    fn test_query_relationships() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let relationships_querier = RelationshipsQuerier::new(deps.querier.deref());

        let response = relationships_querier
            .query_relationships(
                Uint64::new(0),
                Some(Addr::unchecked("")),
                Some(Addr::unchecked("")),
                None,
            )
            .unwrap();
        let expected = QueryRelationshipsResponse {
            relationships: vec![MockRelationshipsQueries::get_mock_relationship()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_blocks() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let relationships_querier = RelationshipsQuerier::new(deps.querier.deref());

        let response = relationships_querier
            .query_blocks(
                Uint64::new(0),
                Some(Addr::unchecked("")),
                Some(Addr::unchecked("")),
                None,
            )
            .unwrap();
        let expected = QueryBlocksResponse {
            blocks: vec![MockRelationshipsQueries::get_mock_user_block()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }
}
