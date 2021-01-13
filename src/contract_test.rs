use cosmwasm_std::testing::{MockQuerier, mock_dependencies, mock_info, mock_env};
use cosmwasm_std::{MessageInfo, DepsMut, Env, attr};
use crate::msg::InitMsg;
use crate::contract::{init, query_filtered_posts};
use crate::state::state_read;
use crate::query::query_post_reports;
use crate::mock::{update_posts, update_reports};
use crate::types::{Post, Report};

fn setup_test(deps: DepsMut, env: Env, info: MessageInfo, default_reports_limit: u16) {
    let init_msg = InitMsg {
        reports_limit: default_reports_limit
    };
    init(deps, env, info, init_msg).unwrap();
}

const POST: Post = Post {
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
creator: "default_creator".to_string()
};

fn set_posts(querier: &mut MockQuerier) {
    querier.with_custom_handler(update_posts(&[POST]));
}

fn set_reports(querier: &mut MockQuerier) {
    let report = Report{
        post_id: POST.post_id,
        _type: "type".to_string(),
        message: "spam".to_string(),
        user: "default_creator".to_string()
    };
    querier.with_custom_handler(update_reports(POST.post_id, &[report]));
}

#[test]
fn test_init() {
    let mut deps = mock_dependencies(&[]);

    let sender_addr = HumanAddr::from("addr0001");
    let info = mock_info(&sender_addr, &[]);

    let init_msg = InitMsg {
        reports_limit: 5,
    };

    let res = init(deps.as_mut(), mock_env(), info, init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    let exp_log = vec![attr("action", "set_default_reports_limit")];

    assert_eq!(res.attributes, exp_log);

    // make sure that the state is set
    let state = state_read(&deps.storage).load().unwrap();
    assert_eq!(5, state.default_reports_limit)
}

#[test]
fn query_filtered_posts_test() {
    let mut deps = mock_dependencies(&[]);
    let sender_addr = HumanAddr::from("addr0001");
    let info = mock_info(&sender_addr, &[]);

    setup_test(deps.as_mut(), mock_env(), info, 5);
    set_posts(&mut deps.querier);
    set_reports(&mut deps.querier);

    let res = query_filtered_posts(deps.as_ref(), 5);


}
