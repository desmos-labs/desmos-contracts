use crate::{
    error::ContractError,
    msg::{HandleMsg, InitMsg, QueryMsg},
    state::{state_read, state_store, State},
};
use cosmwasm_std::{
    attr, to_binary, Binary, Deps, DepsMut, Env, HandleResponse, InitResponse, MessageInfo,
    StdResult,
};
use desmos::{
    custom_query::{query_post_reports, query_posts},
    query_types::PostsResponse,
    types::Post,
};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
pub fn init(deps: DepsMut, _env: Env, _: MessageInfo, msg: InitMsg) -> StdResult<InitResponse> {
    let state = State {
        default_reports_limit: msg.reports_limit,
    };
    state_store(deps.storage).save(&state)?;

    let mut res = InitResponse::default();
    res.attributes = vec![attr("action", "set_default_reports_limit")];
    Ok(res)
}

// And declare a custom Error variant for the ones where you will want to make use of it
pub fn handle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {
        HandleMsg::EditReportsLimit {
            reports_limit: report_limit,
        } => handle_report_limit_edit(deps, info, report_limit),
    }
}

pub fn handle_report_limit_edit(
    deps: DepsMut,
    info: MessageInfo,
    report_limit: u16,
) -> Result<HandleResponse, ContractError> {
    let state = state_read(deps.storage).load()?;

    // if the given report_limit is equal to the stored one returns error
    if state.default_reports_limit == report_limit {
        return Err(ContractError::EqualsReportLimits {});
    };

    state_store(deps.storage).save(&State {
        default_reports_limit: report_limit,
    })?;

    let response = HandleResponse {
        messages: vec![],
        attributes: vec![
            attr("action", "edit_reports_limit"),
            attr("editor", info.sender),
        ],
        data: None,
    };

    Ok(response)
}

pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFilteredPosts { reports_limit } => {
            to_binary(&query_filtered_posts(deps, reports_limit)?)
        }
        QueryMsg::Posts {} => to_binary(&query_posts(&deps.querier)?),
    }
}

/// is_under_report_limit checks if the post is has a number of reports that is less than reports_limit
pub fn is_under_reports_limit(deps: Deps, post: &Post, reports_limit: u16) -> bool {
    let reports_len = query_post_reports(&deps.querier, post.post_id.clone())
        .unwrap()
        .reports
        .len() as u16;
    reports_len < reports_limit
}

/// query_filtered_posts returns a list of filtered posts that has less reports than the reports_limit
pub fn query_filtered_posts(deps: Deps, reports_limit: u16) -> StdResult<PostsResponse> {
    let posts = query_posts(&deps.querier)?;
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
    use super::*;
    use crate::{
        contract::{handle, init, is_under_reports_limit, query_filtered_posts},
        mock::mock_dependencies_with_custom_querier,
        msg::{HandleMsg, InitMsg},
    };
    use cosmwasm_std::{
        testing::{mock_env, mock_info},
        HumanAddr,
    };
    use desmos::types::PollData;

    fn setup_test(deps: DepsMut, env: Env, info: MessageInfo, default_reports_limit: u16) {
        let init_msg = InitMsg {
            reports_limit: default_reports_limit,
        };
        init(deps, env, info, init_msg).unwrap();
    }

    #[test]
    fn test_init() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);

        let sender_addr = HumanAddr::from("addr0001");
        let info = mock_info(&sender_addr, &[]);

        let init_msg = InitMsg { reports_limit: 5 };

        let res = init(deps.as_mut(), mock_env(), info, init_msg).unwrap();
        assert_eq!(0, res.messages.len());

        let exp_log = vec![attr("action", "set_default_reports_limit")];

        assert_eq!(res.attributes, exp_log);

        // make sure that the state is set
        let state = state_read(&deps.storage).load().unwrap();
        assert_eq!(5, state.default_reports_limit)
    }

    #[test]
    fn test_handle() {
        let mut deps = mock_dependencies_with_custom_querier(&[]);
        let editor_addr = HumanAddr::from("editor");
        let info = mock_info(&editor_addr, &[]);
        setup_test(deps.as_mut(), mock_env(), info.clone(), 3);

        let exp_res = HandleResponse {
            messages: vec![],
            attributes: vec![
                attr("action", "edit_reports_limit"),
                attr("editor", info.sender.clone()),
            ],
            data: None,
        };

        let msg = HandleMsg::EditReportsLimit { reports_limit: 5 };
        let res = handle(deps.as_mut(), mock_env(), info.clone(), msg.clone());

        // assert it not fails
        assert!(res.is_ok());

        assert_eq!(res.unwrap(), exp_res);

        let res = handle(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        match res.unwrap_err() {
            ContractError::EqualsReportLimits { .. } => {}
            _ => panic!("expected unregistered error"),
        }
    }

    #[test]
    fn is_under_reports_limit_checks_correctly() {
        let deps = mock_dependencies_with_custom_querier(&[]);
        let post = Post {
            post_id: "id123".to_string(),
            parent_id: "id345".to_string(),
            message: "message".to_string(),
            created: "date-time".to_string(),
            last_edited: "date-time".to_string(),
            allows_comments: false,
            subspace: "subspace".to_string(),
            optional_data: vec![],
            attachments: vec![],
            poll_data: PollData {
                question: "".to_string(),
                provided_answers: vec![],
                end_date: "".to_string(),
                allows_multiple_answers: false,
                allows_answer_edits: false,
            },
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
        let sender_addr = HumanAddr::from("addr0001");
        let info = mock_info(&sender_addr, &[]);

        setup_test(deps.as_mut(), mock_env(), info, 5);

        let post = Post {
            post_id: "id123".to_string(),
            parent_id: "id345".to_string(),
            message: "message".to_string(),
            created: "date-time".to_string(),
            last_edited: "date-time".to_string(),
            allows_comments: false,
            subspace: "subspace".to_string(),
            optional_data: vec![],
            attachments: vec![],
            poll_data: PollData {
                question: "".to_string(),
                provided_answers: vec![],
                end_date: "".to_string(),
                allows_multiple_answers: false,
                allows_answer_edits: false,
            },
            creator: "default_creator".to_string(),
        };

        let expected = PostsResponse { posts: vec![post] };

        // post has less reports than the limit
        let res = query_filtered_posts(deps.as_ref(), 3).unwrap();
        assert_eq!(res, expected);

        // post has equal reports to the limit
        let res = query_filtered_posts(deps.as_ref(), 1).unwrap();
        assert_eq!(res, PostsResponse { posts: vec![] })
    }
}
