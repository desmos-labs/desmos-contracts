use cosmwasm_std::{
    attr, to_binary, Binary, Deps, DepsMut, Env, HandleResponse, InitResponse, MessageInfo,
    StdResult,
};

use crate::custom_query::{query_post_reports, query_posts, PostsQueryResponse};
use crate::error::ContractError;
use crate::msg::{HandleMsg, InitMsg, QueryMsg};
use crate::state::{state_read, state_store, State};
use crate::types::Post;

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
        HandleMsg::EditReportLimit { report_limit } => {
            handle_report_limit_edit(deps, info, report_limit)
        }
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
    }
}

/// is_under_report_limit checks if the post is has a number of reports that is less than reports_limit
pub fn is_under_reports_limit(deps: Deps, post: &Post, reports_limit: u16) -> bool {
    let reports_len = query_post_reports(&deps.querier, post.post_id.clone())
        .unwrap()
        .len() as u16;
    reports_len < reports_limit
}

/// query_filtered_posts returns a list of filtered posts that has less reports than the reports_limit
pub fn query_filtered_posts(deps: Deps, reports_limit: u16) -> StdResult<PostsQueryResponse> {
    let posts = query_posts(&deps.querier)?;
    let filtered_posts = posts
        .into_iter()
        .filter(|post| is_under_reports_limit(deps, post, reports_limit))
        .collect::<Vec<Post>>();
    Ok(PostsQueryResponse {
        posts: filtered_posts,
    })
}
