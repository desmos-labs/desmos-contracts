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

use cosmwasm_vm::testing::{MockApi, MockQuerier, init, MOCK_CONTRACT_ADDR, mock_info, mock_env, mock_instance_options, MockStorage};
use cosmwasm_vm::{Instance, Backend};
use cosmwasm_std::{Coin, SystemResult, HumanAddr, attr};
use desmos_contracts::msg::InitMsg;
use desmos_contracts::state::state_read;
use desmos_contracts::custom_query::DesmosQuery;
use desmos_contracts::mock::custom_query_execute;

const WASM: &[u8] =
include_bytes!("../target/wasm32-unknown-unknown/release/desmos_contracts.wasm");

/*
fn setup_test(
    deps: &mut Backend<MockStorage, MockApi, MockQuerier>,
    env: &Env,
    info: MessageInfo,
    report_limit: u16,
) {
    let init_msg = InitMsg{
        reports_limit: report_limit
    };
    let _res: InitResponse = init(deps, env.clone(), info, init_msg).unwrap();
}
 */


pub fn mock_dependencies_with_custom_querier(
    contract_balance: &[Coin],
) -> Backend<MockApi, MockStorage, MockQuerier<DesmosQuery>> {
    let contract_addr = HumanAddr::from(MOCK_CONTRACT_ADDR);
    let custom_querier: MockQuerier<DesmosQuery> =
        MockQuerier::new(&[(&contract_addr, contract_balance)])
            .with_custom_handler(|query| SystemResult::Ok(custom_query_execute(query)));

    Backend {
        api: MockApi::default(),
        storage: MockStorage::default(),
        querier: custom_querier,
    }
}

#[test]
fn test_init() {
    let custom = mock_dependencies_with_custom_querier(&[]);
    let instance_options = mock_instance_options();
    let mut deps = Instance::from_code(WASM, custom, instance_options).unwrap();

    let sender_addr = HumanAddr::from("addr0001");
    let info = mock_info(&sender_addr, &[]);

    let init_msg = InitMsg { reports_limit: 5 };

    let res = init(&mut deps, mock_env(), info, init_msg).unwrap();
    assert_eq!(0, res.messages.len());

    let exp_log = vec![attr("action", "set_default_reports_limit")];
    assert_eq!(res.attributes, exp_log);

    // make sure that the state is set
    let state = state_read(&deps).load().unwrap();
    assert_eq!(5, state.default_reports_limit)
}
