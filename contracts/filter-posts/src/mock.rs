use cosmwasm_std::{
    testing::{MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR},
    to_binary, Binary, Coin, ContractResult, OwnedDeps, SystemResult,
};
use desmos::{
    query_types::{DesmosQuery, DesmosQueryWrapper, PostsResponse, ReportsResponse},
    types::{Poll, Post, Report},
};

/// Replacement for cosmwasm_std::testing::mock_dependencies
/// this use our CustomQuerier
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MockQuerier<DesmosQueryWrapper>> {
    let contract_addr = MOCK_CONTRACT_ADDR;
    let custom_querier: MockQuerier<DesmosQueryWrapper> =
        MockQuerier::new(&[(&contract_addr, contract_balance)])
            .with_custom_handler(|query| SystemResult::Ok(custom_query_execute(&query)));
    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
    }
}

/// custom_query_execute returns mock responses to custom queries
pub fn custom_query_execute(query: &DesmosQueryWrapper) -> ContractResult<Binary> {
    let response = match query.clone().query_data {
        DesmosQuery::Posts {} => {
            let post = Post {
                post_id: "id123".to_string(),
                parent_id: Some(String::from("id345")),
                message: String::from("message"),
                created: String::from("date-time"),
                last_edited: String::from("date-time"),
                comments_state: String::from("ALLOWED"),
                subspace: String::from("subspace"),
                additional_attributes: Some(vec![]),
                attachments: Some(vec![]),
                poll: Some(Poll {
                    question: "".to_string(),
                    provided_answers: vec![],
                    end_date: "".to_string(),
                    allows_multiple_answers: false,
                    allows_answer_edits: false,
                }),
                creator: String::from("default_creator"),
            };
            to_binary(&PostsResponse { posts: vec![post] })
        }
        DesmosQuery::Reports { post_id } => {
            let report = Report {
                post_id,
                kind: String::from("test"),
                message: String::from("test"),
                user: String::from("default_creator"),
            };
            to_binary(&ReportsResponse {
                reports: vec![report],
            })
        }
    };
    response.into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::{from_binary, QuerierWrapper};
    use desmos::query_types::{DesmosRoute, PostsResponse};
    use desmos::types::Report;

    #[test]
    fn custom_query_execute_posts() {
        let post = Post {
            post_id: String::from("id123"),
            parent_id: Some(String::from("id345")),
            message: String::from("message"),
            created: String::from("date-time"),
            last_edited: String::from("date-time"),
            comments_state: String::from("ALLOWED"),
            subspace: String::from("subspace"),
            additional_attributes: Some(vec![]),
            attachments: Some(vec![]),
            poll: Some(Poll {
                question: "".to_string(),
                provided_answers: vec![],
                end_date: "".to_string(),
                allows_multiple_answers: false,
                allows_answer_edits: false,
            }),
            creator: String::from("default_creator"),
        };
        let expected = PostsResponse { posts: vec![post] };
        let desmos_query_wrapper = DesmosQueryWrapper {
            route: DesmosRoute::Posts,
            query_data: DesmosQuery::Posts {},
        };
        let bz = custom_query_execute(&desmos_query_wrapper).unwrap();
        let response: PostsResponse = from_binary(&bz).unwrap();
        assert_eq!(response, expected)
    }

    #[test]
    fn custom_query_execute_reports() {
        let report = Report {
            post_id: String::from("id123"),
            kind: String::from("test"),
            message: String::from("test"),
            user: String::from("default_creator"),
        };
        let expected = ReportsResponse {
            reports: vec![report],
        };
        let desmos_query_wrapper = DesmosQueryWrapper {
            route: DesmosRoute::Posts,
            query_data: DesmosQuery::Reports {
                post_id: "id123".to_string(),
            },
        };

        let bz = custom_query_execute(&desmos_query_wrapper).unwrap();
        let response: ReportsResponse = from_binary(&bz).unwrap();
        assert_eq!(response, expected)
    }

    #[test]
    fn custom_querier() {
        let deps = mock_dependencies_with_custom_querier(&[]);
        let req = DesmosQueryWrapper {
            route: DesmosRoute::Posts,
            query_data: DesmosQuery::Reports {
                post_id: "id123".to_string(),
            },
        }
            .into();
        let wrapper = QuerierWrapper::new(&deps.querier);
        let response: ReportsResponse = wrapper.custom_query(&req).unwrap();
        let expected = vec![Report {
            post_id: String::from("id123"),
            kind: String::from("test"),
            message: String::from("test"),
            user: String::from("default_creator"),
        }];
        assert_eq!(response.reports, expected);
    }
}
