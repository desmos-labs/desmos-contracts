#[cfg(test)]
mod tests {
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg};
    use crate::test_utils::*;
    use cosmwasm_std::{wasm_execute, Addr, BlockInfo, Empty, Timestamp};
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};
    use poap::msg::{
        EventInfo, InstantiateMsg as POAPInstantiateMsg,
        QueryConfigResponse as POAPQueryConfigResponse,
        QueryMintedAmountResponse as POAPQueryMintedAmountResponse, QueryMsg as POAPQueryMsg,
    };

    const ADMIN: &str = "admin";
    const RECIPIENT: &str = "recipient";

    fn mock_app() -> App {
        AppBuilder::new()
            .with_block(BlockInfo {
                height: 42,
                time: Timestamp::from_seconds(0),
                chain_id: "testchain".to_string(),
            })
            .build(|_, _, _| {})
    }

    fn contract_poap_manger() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        )
        .with_reply(crate::contract::reply);
        Box::new(contract)
    }

    fn store_contracts(app: &mut App) -> (u64, u64, u64) {
        let cw721_code_id = app.store_code(CW721TestContract::success_contract());
        let poap_code_id = app.store_code(POAPTestContract::success_contract());
        let poap_manager_code_id = app.store_code(contract_poap_manger());
        (cw721_code_id, poap_code_id, poap_manager_code_id)
    }

    fn get_valid_init_msg(cw721_code_id: u64, poap_code_id: u64) -> InstantiateMsg {
        InstantiateMsg {
            admin: ADMIN.into(),
            poap_code_id: poap_code_id.into(),
            poap_instantiate_msg: POAPInstantiateMsg {
                admin: ADMIN.into(),
                minter: "".into(),
                cw721_code_id: cw721_code_id.into(),
                cw721_initiate_msg: Cw721InstantiateMsg {
                    minter: "".into(),
                    name: "test".into(),
                    symbol: "test".into(),
                },
                event_info: EventInfo {
                    creator: "creator".to_string(),
                    start_time: Timestamp::from_seconds(10),
                    end_time: Timestamp::from_seconds(20),
                    per_address_limit: 2,
                    base_poap_uri: "ipfs://popap-uri".to_string(),
                    event_uri: "ipfs://event-uri".to_string(),
                },
            },
        }
    }

    fn proper_instantiate() -> (App, Addr) {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id, poap_manager_code_id) = store_contracts(&mut app);
        let poap_manager_contract_addr = app
            .instantiate_contract(
                poap_manager_code_id,
                Addr::unchecked(ADMIN),
                &get_valid_init_msg(cw721_code_id, poap_code_id),
                &[],
                "Poap manager contract",
                None,
            )
            .unwrap();
        app.update_block(|block| {
            // update the time to start time of event
            block.time = Timestamp::from_seconds(10);
        });
        (app, poap_manager_contract_addr)
    }

    #[test]
    fn instantiate_with_invalid_poap_code_id_error() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id, poap_manager_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id, poap_code_id);
        // change code poap_code_id to the invalid one
        init_msg.poap_code_id = cw721_code_id.into();
        let init_result = app.instantiate_contract(
            poap_manager_code_id,
            Addr::unchecked(ADMIN),
            &init_msg,
            &[],
            "Poap manager contract",
            None,
        );
        assert!(init_result.is_err());
    }

    #[test]
    fn instantiate_with_invalid_cw721_code_id_error() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id, poap_manager_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id, poap_code_id);
        // change cw721_code_id to the invalid one
        init_msg.poap_instantiate_msg.cw721_code_id = poap_code_id.into();
        let init_result = app.instantiate_contract(
            poap_manager_code_id,
            Addr::unchecked(ADMIN),
            &init_msg,
            &[],
            "Poap manager contract",
            None,
        );
        assert!(init_result.is_err());
    }

    #[test]
    fn instantiate_with_failing_poap_contract_error() {
        let mut app = mock_app();
        let (cw721_code_id, _, poap_manager_code_id) = store_contracts(&mut app);
        let failing_poap_code_id = app.store_code(POAPTestContract::failing_contract());
        let mut init_msg = get_valid_init_msg(cw721_code_id, failing_poap_code_id);
        // change id to the failing one
        init_msg.poap_code_id = failing_poap_code_id.into();
        let init_result = app.instantiate_contract(
            poap_manager_code_id,
            Addr::unchecked(ADMIN),
            &init_msg,
            &[],
            "Poap manager contract",
            None,
        );
        assert!(init_result.is_err());
    }

    #[test]
    fn instantiate_propery() {
        let (app, manager_addr) = proper_instantiate();
        let querier = app.wrap();

        // check if poap minter is manager contract
        let manager_config: QueryConfigResponse = querier
            .query_wasm_smart(&manager_addr, &QueryMsg::Config {})
            .unwrap();
        let poap_config: POAPQueryConfigResponse = querier
            .query_wasm_smart(manager_config.poap_address, &POAPQueryMsg::Config {})
            .unwrap();
        assert_eq!(manager_addr, poap_config.minter);
    }

    #[test]
    fn user_claim_poap_properly() {
        // TODO: build a test after desmos_bindings::mocks::mock_apps is released
    }

    #[test]
    fn mint_poap_to_recipient_properly() {
        let (mut app, manager_addr) = proper_instantiate();
        let result = app.execute(
            Addr::unchecked(ADMIN),
            wasm_execute(
                &manager_addr,
                &ExecuteMsg::MintTo {
                    recipient: RECIPIENT.into(),
                },
                vec![],
            )
            .unwrap()
            .into(),
        );
        assert!(result.is_ok());

        // check the state of poap contract
        let querier = app.wrap();
        let manager_config: QueryConfigResponse = querier
            .query_wasm_smart(&manager_addr, &QueryMsg::Config {})
            .unwrap();
        let minted_amount_response: POAPQueryMintedAmountResponse = querier
            .query_wasm_smart(
                manager_config.poap_address,
                &POAPQueryMsg::MintedAmount {
                    user: RECIPIENT.into(),
                },
            )
            .unwrap();
        assert_eq!(minted_amount_response.user, RECIPIENT);
        assert_eq!(minted_amount_response.amount, 1)
    }
}
