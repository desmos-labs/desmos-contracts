use cosmwasm_std::{
    attr, BankMsg, Coin, ContractResult, CosmosMsg, Env, MessageInfo, Response, SystemResult,
};
use cosmwasm_storage::to_length_prefixed;
use cosmwasm_vm::{
    testing::{
        instantiate, mock_env, mock_info, mock_instance_options, sudo, MockApi, MockQuerier,
        MockStorage, MOCK_CONTRACT_ADDR,
    },
    Backend, Instance, Storage,
};
use cw_desmos_dummy_tokenomics::{
    msg::{InstantiateMsg, SudoMsg},
    state::TOKEN_DENOM_KEY,
};
use desmos::{mock::custom_query_execute, query_types::DesmosQueryWrapper};

#[cfg(not(tarpaulin_include))]
const WASM: &[u8] = include_bytes!("dummy_tokenomics.wasm");

#[cfg(not(tarpaulin_include))]
fn setup_test(
    deps: &mut Instance<MockApi, MockStorage, MockQuerier<DesmosQueryWrapper>>,
    env: Env,
    info: MessageInfo,
    denom: String,
) {
    let instantiate_msg = InstantiateMsg { token_denom: denom };
    let _res: Response = instantiate(deps, env.clone(), info, instantiate_msg).unwrap();
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
fn test_instantiate() {
    let funds = Coin::new(100_000_000, "udesmos");
    let custom = mock_dependencies_with_custom_querier(&[funds]);
    let (instance_options, memory_limit) = mock_instance_options();
    let info = mock_info("addr0001", &[]);

    let mut deps = Instance::from_code(WASM, custom, instance_options, memory_limit).unwrap();

    let instantiate_msg = InstantiateMsg {
        token_denom: "udesmos".to_string(),
    };

    let res: Response = instantiate(&mut deps, mock_env(), info, instantiate_msg).unwrap();
    assert_eq!(0, res.messages.len());

    let exp_log = vec![attr("action", "set_token_denom")];
    assert_eq!(res.attributes, exp_log);

    // make sure that the state is set
    deps.with_storage(|storage| {
        let key = to_length_prefixed(TOKEN_DENOM_KEY);
        let data = storage.get(&key).0.unwrap().unwrap();
        let token_denom = String::from_utf8(data).unwrap();

        assert_eq!(token_denom, "\"udesmos\"");
        Ok(())
    })
    .unwrap();
}

#[test]
#[cfg(not(tarpaulin_include))]
fn test_execute_tokenomics_successfully() {
    let funds = Coin::new(100_000_000, "udesmos");
    let info = mock_info("addr0001", &[]);
    let custom = mock_dependencies_with_custom_querier(&[funds]);
    let (instance_options, memory_limit) = mock_instance_options();

    let mut deps = Instance::from_code(WASM, custom, instance_options, memory_limit).unwrap();

    setup_test(&mut deps, mock_env(), info, "udesmos".to_string());

    let exp_response = Response {
        messages: vec![CosmosMsg::from(BankMsg::Send {
            to_address: "default_creator".to_string(),
            amount: vec![Coin::new(1_000_000, "udesmos")],
        })],
        attributes: vec![
            attr("action", "executed_tokenomics"),
            attr("subspace_id", "subspace"),
        ],
        ..Response::default()
    };

    let msg = SudoMsg::ExecuteTokenomics {};
    let response: ContractResult<Response> = sudo(&mut deps, mock_env(), msg.clone());

    assert!(response.is_ok());
    assert_eq!(exp_response, response.unwrap())
}
