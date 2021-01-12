use super::*;
use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
use cosmwasm_std::{coins, from_binary, attr, HumanAddr, MessageInfo, DepsMut, Env};
use crate::msg::InitMsg;
use crate::contract::{init, query_filtered_posts};
use crate::state::state_read;
use crate::query::query_post_reports;

fn setup_test(deps: DepsMut, env: Env, info: MessageInfo, default_reports_limit: u16) {
    let init_msg = InitMsg {
        reports_limit: default_reports_limit
    };
    init(deps, env, info, init_msg).unwrap();
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

    let res = query_filtered_posts(deps.as_ref(), 5);



}
