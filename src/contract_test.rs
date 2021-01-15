use crate::contract::{handle, init, is_under_reports_limit, query_filtered_posts};
use crate::custom_query::PostsQueryResponse;
use crate::error::ContractError;
use crate::mock::mock_dependencies_with_custom_querier;
use crate::msg::{HandleMsg, InitMsg};
use crate::state::state_read;
use crate::types::Post;
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{attr, DepsMut, Env, HandleResponse, HumanAddr, MessageInfo};

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

    let msg = HandleMsg::EditReportLimit { report_limit: 5 };
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
        poll_data: vec![],
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
        poll_data: vec![],
        creator: "default_creator".to_string(),
    };

    let expected = PostsQueryResponse { posts: vec![post] };

    // post has less reports than the limit
    let res = query_filtered_posts(deps.as_ref(), 3).unwrap();
    assert_eq!(res, expected);

    // post has equal reports to the limit
    let res = query_filtered_posts(deps.as_ref(), 1).unwrap();
    assert_eq!(res, PostsQueryResponse { posts: vec![] })
}
