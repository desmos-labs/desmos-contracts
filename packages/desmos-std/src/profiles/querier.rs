use crate::{
    profiles::{
        models_query::{
            QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
            QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse,
            QueryProfileResponse,
        },
        query::ProfilesQuery,
    },
    query::DesmosQuery,
    types::PageRequest,
};
use cosmwasm_std::{Addr, Querier, QuerierWrapper, StdResult};

pub struct ProfilesQuerier<'a> {
    querier: QuerierWrapper<'a, DesmosQuery>,
}

impl<'a> ProfilesQuerier<'a> {
    pub fn new(querier: &'a dyn Querier) -> Self {
        Self {
            querier: QuerierWrapper::<'a, DesmosQuery>::new(querier),
        }
    }

    pub fn query_profile(&self, user: Addr) -> StdResult<QueryProfileResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::Profile { user });

        let res: QueryProfileResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_incoming_dtag_transfer_requests(
        &self,
        receiver: Addr,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryIncomingDtagTransferRequestResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::IncomingDtagTransferRequests {
            receiver,
            pagination,
        });

        let res: QueryIncomingDtagTransferRequestResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_chain_links(
        &self,
        user: Option<Addr>,
        chain_name: Option<String>,
        target: Option<String>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryChainLinksResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::ChainLinks {
            user,
            chain_name,
            target,
            pagination,
        });

        let res: QueryChainLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_application_links(
        &self,
        user: Option<Addr>,
        application: Option<String>,
        username: Option<String>,
        pagination: Option<PageRequest>,
    ) -> StdResult<QueryApplicationLinksResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::AppLinks {
            user,
            application,
            username,
            pagination,
        });

        let res: QueryApplicationLinksResponse = self.querier.query(&request.into())?;
        Ok(res)
    }

    pub fn query_application_link_by_client_id(
        &self,
        client_id: String,
    ) -> StdResult<QueryApplicationLinkByClientIDResponse> {
        let request = DesmosQuery::Profiles(ProfilesQuery::ApplicationLinkByChainID { client_id });

        let res: QueryApplicationLinkByClientIDResponse = self.querier.query(&request.into())?;
        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        mock::mock_dependencies_with_custom_querier,
        profiles::{
            mock::MockProfilesQueries,
            models_query::{
                QueryApplicationLinkByClientIDResponse, QueryApplicationLinksResponse,
                QueryChainLinksResponse, QueryIncomingDtagTransferRequestResponse,
                QueryProfileResponse,
            },
            querier::ProfilesQuerier,
        },
    };
    use cosmwasm_std::Addr;
    use std::ops::Deref;

    #[test]
    fn test_query_profile() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier.query_profile(Addr::unchecked("")).unwrap();
        let expected = QueryProfileResponse {
            profile: MockProfilesQueries::get_mock_profile(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_incoming_dtag_transfer_requests() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_incoming_dtag_transfer_requests(Addr::unchecked(""), None)
            .unwrap();
        let expected = QueryIncomingDtagTransferRequestResponse {
            requests: vec![MockProfilesQueries::get_mock_dtag_transfer_request()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_chain_links() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_chain_links(
                Some(Addr::unchecked("")),
                Some("cosmos".to_string()),
                Some("cosmos18xnmlzqrqr6zt526pnczxe65zk3f4xgmndpxn2".to_string()),
                None,
            )
            .unwrap();
        let expected = QueryChainLinksResponse {
            links: vec![MockProfilesQueries::get_mock_chain_link()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_app_links() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_application_links(
                Some(Addr::unchecked("")),
                Some("twitter".to_string()),
                Some("goldrake".to_string()),
                None,
            )
            .unwrap();
        let expected = QueryApplicationLinksResponse {
            links: vec![MockProfilesQueries::get_mock_application_link()],
            pagination: Default::default(),
        };

        assert_eq!(response, expected)
    }

    #[test]
    fn test_query_application_link_by_chain_id() {
        let owned_deps = mock_dependencies_with_custom_querier(&[]);
        let deps = owned_deps.as_ref();
        let profiles_querier = ProfilesQuerier::new(deps.querier.deref());

        let response = profiles_querier
            .query_application_link_by_client_id("".to_string())
            .unwrap();
        let expected = QueryApplicationLinkByClientIDResponse {
            link: MockProfilesQueries::get_mock_application_link(),
        };

        assert_eq!(response, expected)
    }
}
