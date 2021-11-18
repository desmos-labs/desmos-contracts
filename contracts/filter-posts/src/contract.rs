use cosmwasm_std::{attr, entry_point, to_binary, Binary, Env, MessageInfo, Response, StdResult};

use desmos_cw::{
    querier::DesmosQuerier,
    query_types::PostsResponse,
    types::{Deps, DepsMut, Post},
};

use crate::{
    error::ContractError,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::{state_read, state_store, State},
};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors and declare a custom Error variant for the ones where you will want to make use of it

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> StdResult<Response> {
    let state = State {
        default_reports_limit: msg.reports_limit,
    };
    state_store(deps.storage).save(&state)?;

    let res = Response::new().add_attributes(vec![attr("action", "set_default_reports_limit")]);
    Ok(res)
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::EditReportsLimit {
            reports_limit: report_limit,
        } => handle_report_limit_edit(deps, info, report_limit),
    }
}

pub fn handle_report_limit_edit(
    deps: DepsMut,
    info: MessageInfo,
    report_limit: u16,
) -> Result<Response, ContractError> {
    let state = state_read(deps.storage).load()?;

    // if the given report_limit is equal to the stored one returns error
    if state.default_reports_limit == report_limit {
        return Err(ContractError::EqualsReportLimits {});
    };

    state_store(deps.storage).save(&State {
        default_reports_limit: report_limit,
    })?;

    let response = Response::new().add_attributes(vec![
        attr("action", "edit_reports_limit"),
        attr("editor", info.sender),
    ]);

    Ok(response)
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFilteredPosts { reports_limit } => {
            to_binary(&query_filtered_posts(deps, reports_limit)?)
        }
    }
}

/// is_under_report_limit checks if the post is has a number of reports that is less than reports_limit
pub fn is_under_reports_limit(deps: Deps, post: &Post, reports_limit: u16) -> bool {
    let querier = DesmosQuerier::new(&deps.querier);
    let reports_len = querier
        .query_post_reports(post.post_id.clone())
        .unwrap()
        .reports
        .len() as u16;

    reports_len < reports_limit
}

/// query_filtered_posts returns a list of filtered posts that has less reports than the reports_limit
pub fn query_filtered_posts(deps: Deps, reports_limit: u16) -> StdResult<PostsResponse> {
    let querier = DesmosQuerier::new(&deps.querier);
    let posts = querier.query_posts()?;
    let filtered_posts = posts
        .posts
        .into_iter()
        .filter(|post| is_under_reports_limit(deps, post, reports_limit))
        .collect::<Vec<Post>>();
    Ok(PostsResponse {
        posts: filtered_posts,
    })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_env, mock_info};

    use desmos_cw::{mock::mock_dependencies_with_custom_querier, types::Poll};

    use crate::{
        contract::{execute, instantiate, is_under_reports_limit, query_filtered_posts},
        msg::{ExecuteMsg, InstantiateMsg},
    };

    use super::*;

    fn setup_test(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        default_reports_limit: u16,
    ) -> (Post, PostsResponse) {
        let init_msg = InstantiateMsg {
            reports_limit: default_reports_limit,
        };
        instantiate(deps, env, info, init_msg).unwrap();

        let post = Post {
            post_id: "id123".to_string(),
            parent_id: Some("id345".to_string()),
            message: "message".to_string(),
            created: "date-time".to_string(),
            last_edited: "date-time".to_string(),
            comments_state: "ALLOWED".to_string(),
            subspace: "subspace".to_string(),
            additional_attributes: Some(vec![]),
            attachments: Some(vec![]),
            poll: Some(Poll {
                question: "".to_string(),
                provided_answers: vec![],
                end_date: "".to_string(),
                allows_multiple_answers: false,
                allows_answer_edits: false,
            }),
            creator: "default_creator".to_string(),
        };
        let response = PostsResponse {
            posts: vec![post.to_owned()],
        };

        return (post, response);
    }

    #[test]
    fn test_instantiate() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let info = mock_info("addr0001", &[]);

        let init_msg = InstantiateMsg { reports_limit: 5 };

        let res = instantiate(deps.as_mut(), mock_env(), info, init_msg).unwrap();
        assert_eq!(0, res.messages.len());

        let exp_log = vec![attr("action", "set_default_reports_limit")];

        assert_eq!(res.attributes, exp_log);

        // make sure that the state is set
        let state = state_read(&deps.storage).load().unwrap();
        assert_eq!(5, state.default_reports_limit)
    }

    #[test]
    fn test_execute() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let info = mock_info("editor", &[]);
        setup_test(deps.as_mut(), mock_env(), info.clone(), 3);

        let exp_res = Response::new().add_attributes(vec![
            attr("action", "edit_reports_limit"),
            attr("editor", info.sender.clone()),
        ]);

        let msg = ExecuteMsg::EditReportsLimit { reports_limit: 5 };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());

        // assert it not fails
        assert!(res.is_ok());

        assert_eq!(res.unwrap(), exp_res);

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        match res.unwrap_err() {
            ContractError::EqualsReportLimits { .. } => {}
            _ => panic!("expected unregistered error"),
        }
    }

    #[test]
    fn test_query() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let info = mock_info("editor", &[]);
        let (_, expected) = setup_test(deps.as_mut(), mock_env(), info.clone(), 3);

        let msg = QueryMsg::GetFilteredPosts { reports_limit: 3 };
        let res = query(deps.as_ref(), mock_env(), msg);

        let expected_res = to_binary(&expected);

        assert_eq!(res, expected_res)
    }

    #[test]
    fn is_under_reports_limit_checks_correctly() {
        let deps = mock_dependencies_with_custom_querier(&[]);
        let post = Post {
            post_id: "id123".to_string(),
            parent_id: Some("id345".to_string()),
            message: "message".to_string(),
            created: "date-time".to_string(),
            last_edited: "date-time".to_string(),
            comments_state: "ALLOWED".to_string(),
            subspace: "subspace".to_string(),
            additional_attributes: Some(vec![]),
            attachments: Some(vec![]),
            poll: Some(Poll {
                question: "question".to_string(),
                provided_answers: vec![],
                end_date: "".to_string(),
                allows_multiple_answers: false,
                allows_answer_edits: false,
            }),
            creator: "default_creator".to_string(),
        };

        // post is under the report limit
        let res = is_under_reports_limit(deps.as_ref(), &post, 5);
        assert_eq!(res, true);

        // post is over the report limit
        let res = is_under_reports_limit(deps.as_ref(), &post, 1);
        assert_eq!(res, false)
    }

    #[test]
    fn query_filtered_posts_filter_correctly() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let info = mock_info("addr0001", &[]);

        let (_, expected) = setup_test(deps.as_mut(), mock_env(), info, 5);

        // post has less reports than the limit
        let res = query_filtered_posts(deps.as_ref(), 3).unwrap();
        assert_eq!(res, expected);

        // post has equal reports to the limit
        let res = query_filtered_posts(deps.as_ref(), 1).unwrap();
        assert_eq!(res, PostsResponse { posts: vec![] })
    }
}
