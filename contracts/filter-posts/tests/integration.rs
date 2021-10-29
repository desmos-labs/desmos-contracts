//! This integration tests try to run and call the generated wasm.
//! They depends on a Wasm build being available, which you can create with `cargo wasm`.
//! Then running `cargo integration-test` will validate we can properly call into that generated Wasm.
//! You can easily convert unit tests to integration tests as follows:
//! 1. Copy them over verbatim
//! 2. Then change
//!      let mut deps = mock_dependencies(20, &[]);
//!    to
//!      let mut deps = mock_instance(WASM, &[]);
//! 3. If you access raw storage, where ever you see something like:
//!      deps.storage.get(CONFIG_KEY).expect("no data stored");
//!    replace it with:
//!      deps.with_storage(|store| {
//!          let data = store.get(CONFIG_KEY).expect("no data stored");
//!          //...
//!      });
//! 4. Anywhere you see query(&deps, ...) you must replace it with query(&mut deps, ...)

use cosmwasm_std::{
    attr, from_binary, Coin, ContractResult, Env, MessageInfo, Response, SystemResult,
};
use cosmwasm_storage::to_length_prefixed;
use cosmwasm_vm::{
    testing::{
        execute, instantiate, mock_env, mock_info, mock_instance_options, query, MockApi,
        MockQuerier, MockStorage, MOCK_CONTRACT_ADDR,
    },
    Backend, Instance, Storage,
};

use cw_desmos_filter_posts::{
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
    state::REPORTS_LIMIT_KEY,
};
use desmos::mock::custom_query_execute;
use desmos::{
    query_types::{DesmosQueryWrapper, PostsResponse},
    types::{Poll, Post},
};

#[cfg(not(tarpaulin_include))]
const WASM: &[u8] = include_bytes!("cw_desmos_filter_posts.wasm");

#[cfg(not(tarpaulin_include))]
fn setup_test(
    deps: &mut Instance<MockApi, MockStorage, MockQuerier<DesmosQueryWrapper>>,
    env: Env,
    info: MessageInfo,
    report_limit: u16,
) {
    let init_msg = InstantiateMsg {
        reports_limit: report_limit,
    };
    let _res: Response = instantiate(deps, env.clone(), info, init_msg).unwrap();
}

#[cfg(not(tarpaulin_include))]
pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> Backend<MockApi, MockStorage, MockQuerier<DesmosQueryWrapper>> {
    let custom_querier: MockQuerier<DesmosQueryWrapper> =
        MockQuerier::new(&[(MOCK_CONTRACT_ADDR, contract_balance)])
            .with_custom_handler(|query| SystemResult::Ok(custom_query_execute(query)));

    Backend {
        api: MockApi::default(),
        storage: MockStorage::default(),
        querier: custom_querier,
    }
}

#[test]
#[cfg(not(tarpaulin_include))]
fn test_init() {
    let custom = mock_dependencies_with_custom_querier(&[]);
    let instance_options = mock_instance_options();
    let mut deps =
        Instance::from_code(WASM, custom, instance_options.0, instance_options.1).unwrap();

    let info = mock_info("addr0001", &[]);

    let init_msg = InstantiateMsg { reports_limit: 5 };

    let res: Response = instantiate(&mut deps, mock_env(), info, init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    let exp_log = vec![attr("action", "set_default_reports_limit")];
    assert_eq!(res.attributes, exp_log);

    // make sure that the state is set
    deps.with_storage(|storage| {
        let key = to_length_prefixed(REPORTS_LIMIT_KEY);
        let data = storage.get(&key).0.unwrap().unwrap();
        let default_limit = String::from_utf8(data).unwrap();

        assert_eq!(default_limit, "{\"default_reports_limit\":5}");
        Ok(())
    })
        .unwrap();
}

#[test]
#[cfg(not(tarpaulin_include))]
fn test_handle() {
    let custom = mock_dependencies_with_custom_querier(&[]);
    let instance_options = mock_instance_options();
    let mut deps =
        Instance::from_code(WASM, custom, instance_options.0, instance_options.1).unwrap();

    let info = mock_info("addr0001", &[]);

    setup_test(&mut deps, mock_env(), info.clone(), 3);

    let exp_res = Response::new()
        .add_attributes(vec![
            attr("action", "edit_reports_limit"),
            attr("editor", info.sender.clone()),
        ]);

    let msg = ExecuteMsg::EditReportsLimit { reports_limit: 5 };
    let res: ContractResult<Response> = execute(&mut deps, mock_env(), info.clone(), msg.clone());

    assert_eq!(res.unwrap(), exp_res);

    let res: ContractResult<Response> = execute(&mut deps, mock_env(), info.clone(), msg.clone());

    assert!(res
        .unwrap_err()
        .contains("Report limit is the same as the stored one"));
}

#[test]
#[cfg(not(tarpaulin_include))]
fn query_filtered_posts_filter_correctly() {
    let custom = mock_dependencies_with_custom_querier(&[]);
    let instance_options = mock_instance_options();
    let mut deps =
        Instance::from_code(WASM, custom, instance_options.0, instance_options.1).unwrap();

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

    let expected = PostsResponse { posts: vec![post] };
    let query_msg = QueryMsg::GetFilteredPosts { reports_limit: 3 };

    // post has less reports than the limit
    let res = query(&mut deps, mock_env(), query_msg).unwrap();
    let unwrapped: PostsResponse = from_binary(&res).unwrap();
    assert_eq!(unwrapped, expected);

    // post has equal reports to the limit
    let query_msg = QueryMsg::GetFilteredPosts { reports_limit: 1 };
    let res = query(&mut deps, mock_env(), query_msg).unwrap();
    let unwrapped: PostsResponse = from_binary(&res).unwrap();
    assert_eq!(unwrapped, PostsResponse { posts: vec![] })
}
