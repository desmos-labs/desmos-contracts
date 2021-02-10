use cosmwasm_std::testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{to_binary, Binary, Coin, ContractResult, HumanAddr, OwnedDeps, SystemResult};
use desmos::custom_query::{DesmosQuery, PostsQueryResponse, ReportsQueryResponse};
use desmos::types::{Post, Report};

/// Replacement for cosmwasm_std::testing::mock_dependencies
/// this use our CustomQuerier
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DesmosQuery>> {
    let contract_addr = HumanAddr::from(MOCK_CONTRACT_ADDR);
    let custom_querier: MockQuerier<DesmosQuery> =
        MockQuerier::new(&[(&contract_addr, contract_balance)])
            .with_custom_handler(|query| SystemResult::Ok(custom_query_execute(&query)));
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}

/// custom_query_execute returns mock responses to custom queries
pub fn custom_query_execute(query: &DesmosQuery) -> ContractResult<Binary> {
    let response = match query {
        DesmosQuery::Posts {} => {
            let post = Post {
                post_id: "id123".to_string(),
                parent_id: String::from("id345"),
                message: String::from("message"),
                created: String::from("date-time"),
                last_edited: String::from("date-time"),
                allows_comments: false,
                subspace: String::from("subspace"),
                optional_data: vec![],
                attachments: vec![],
                poll_data: vec![],
                creator: String::from("default_creator"),
            };
            to_binary(&PostsQueryResponse { posts: vec![post] })
        }
        DesmosQuery::Reports { post_id } => {
            let report = Report {
                post_id: post_id.to_string(),
                _type: String::from("test"),
                message: String::from("test"),
                user: String::from("default_creator"),
            };
            to_binary(&ReportsQueryResponse {
                reports: vec![report],
            })
        }
    };
    response.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{from_binary, QuerierWrapper, QueryRequest};
    use desmos::types::Report;

    #[test]
    fn custom_query_execute_posts() {
        let post = Post {
            post_id: String::from("id123"),
            parent_id: String::from("id345"),
            message: String::from("message"),
            created: String::from("date-time"),
            last_edited: String::from("date-time"),
            allows_comments: false,
            subspace: String::from("subspace"),
            optional_data: vec![],
            attachments: vec![],
            poll_data: vec![],
            creator: String::from("default_creator"),
        };
        let expected = PostsQueryResponse { posts: vec![post] };
        let bz = custom_query_execute(&DesmosQuery::Posts {}).unwrap();
        let response: PostsQueryResponse = from_binary(&bz).unwrap();
        assert_eq!(response, expected)
    }

    #[test]
    fn custom_query_execute_reports() {
        let report = Report {
            post_id: String::from("id123"),
            _type: String::from("test"),
            message: String::from("test"),
            user: String::from("default_creator"),
        };
        let expected = ReportsQueryResponse {
            reports: vec![report],
        };
        let bz = custom_query_execute(&DesmosQuery::Reports {
            post_id: "id123".to_string(),
        })
        .unwrap();
        let response: ReportsQueryResponse = from_binary(&bz).unwrap();
        assert_eq!(response, expected)
    }

    #[test]
    fn custom_querier() {
        let deps = mock_dependencies_with_custom_querier(&[]);
        let req: QueryRequest<_> = DesmosQuery::Reports {
            post_id: "id123".to_string(),
        }
        .into();
        let wrapper = QuerierWrapper::new(&deps.querier);
        let response: ReportsQueryResponse = wrapper.custom_query(&req).unwrap();
        let expected = vec![Report {
            post_id: String::from("id123"),
            _type: String::from("test"),
            message: String::from("test"),
            user: String::from("default_creator"),
        }];
        assert_eq!(response.reports, expected);
    }
}
