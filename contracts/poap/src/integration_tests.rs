#[cfg(test)]
mod tests {
    use crate::cw721_utils;
    use crate::helpers::CwTemplateContract;
    use crate::msg::{EventInfo, InstantiateMsg};
    use cosmwasm_std::{coins, Addr, BlockInfo, Empty, Timestamp};
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    const CREATOR: &str = "desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3";
    const ADMIN: &str = "desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3";
    const USER: &str = "desmos1ptvq7l4jt7n9sc3fky22mfvc6waf2jd8nuc0jv";
    const NATIVE_DENOM: &str = "udsm";
    const CREATION_FEE: u128 = 1_000_000_000;
    const INITIAL_BALANCE: u128 = 2_000_000_000;

    fn contract_poap() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        )
        .with_reply(crate::contract::reply);
        Box::new(contract)
    }

    fn mock_app() -> App {
        AppBuilder::new()
            .with_block(BlockInfo {
                height: 42,
                time: Timestamp::from_seconds(1_000_000),
                chain_id: "testchain".to_string(),
            })
            .build(|router, _, storage| {
                router
                    .bank
                    .init_balance(
                        storage,
                        &Addr::unchecked(USER),
                        coins(INITIAL_BALANCE, NATIVE_DENOM),
                    )
                    .unwrap();
                router
                    .bank
                    .init_balance(
                        storage,
                        &Addr::unchecked(ADMIN),
                        coins(INITIAL_BALANCE + CREATION_FEE, NATIVE_DENOM),
                    )
                    .unwrap();
            })
    }

    /// Uploads the contracts to the app.
    /// Returns a pair of ids where the first refers to the cw721
    /// and the second to the poap.
    fn store_contracts(app: &mut App) -> (u64, u64) {
        let cw721_code_id = app.store_code(cw721_utils::contract_cw721());
        let poap_code_id = app.store_code(contract_poap());

        (cw721_code_id, poap_code_id)
    }

    fn get_valid_init_msg(app: &App, cw721_code_id: u64) -> InstantiateMsg {
        let block_info = app.block_info();
        let start_time = Timestamp::from_seconds(block_info.time.seconds() + 3600);
        let end_time = Timestamp::from_seconds(start_time.seconds() + 3600);

        InstantiateMsg {
            admin: None,
            minter: None,
            cw721_code_id: cw721_code_id.into(),
            cw721_initiate_msg: Cw721InstantiateMsg {
                name: "test-poap".to_string(),
                symbol: "poap".to_string(),
                minter: "".to_string(),
            },
            event_info: EventInfo {
                creator: CREATOR.to_string(),
                start_time,
                end_time,
                per_address_limit: 10,
                base_poap_uri: "ipfs://popap-uri".to_string(),
                event_uri: "https://event-uri.com".to_string(),
                cw721_code_id: 1,
            },
        }
    }

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let msg = get_valid_init_msg(&app, cw721_code_id);

        let cw_template_contract_addr = app
            .instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "Poap contract",
                None,
            )
            .unwrap();

        let cw_template_contract = CwTemplateContract(cw_template_contract_addr);
        (app, cw_template_contract)
    }

    mod instantiate {
        use crate::cw721_utils;
        use crate::integration_tests::tests::{
            get_valid_init_msg, mock_app, store_contracts, ADMIN,
        };
        use cosmwasm_std::{Addr, Timestamp};
        use cw_multi_test::Executor;

        #[test]
        fn invalid_admin_addr() {
            let mut app = mock_app();
            let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
            let mut init_msg = get_valid_init_msg(&app, cw721_code_id);

            init_msg.admin = Some("a".to_string());

            let init_result = app.instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "Poap contract",
                None,
            );
            assert!(init_result.is_err());
        }

        #[test]
        fn invalid_minter_addr() {
            let mut app = mock_app();
            let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
            let mut init_msg = get_valid_init_msg(&app, cw721_code_id);

            init_msg.minter = Some("a".to_string());

            let init_result = app.instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "Poap contract",
                None,
            );
            assert!(init_result.is_err());
        }

        #[test]
        fn invalid_creator_addr() {
            let mut app = mock_app();
            let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
            let mut init_msg = get_valid_init_msg(&app, cw721_code_id);

            init_msg.event_info.creator = "a".to_string();

            let init_result = app.instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "Poap contract",
                None,
            );
            assert!(init_result.is_err());
        }

        #[test]
        fn invalid_cw721_code_id() {
            let mut app = mock_app();
            let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
            let mut init_msg = get_valid_init_msg(&app, cw721_code_id);

            init_msg.cw721_code_id = 42u64.into();

            let init_result = app.instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "Poap contract",
                None,
            );
            assert!(init_result.is_err());
        }

        #[test]
        fn failing_cw721_contract() {
            let mut app = mock_app();
            let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
            let failing_cw721_code_id = app.store_code(cw721_utils::failing_cw721());
            let mut init_msg = get_valid_init_msg(&app, cw721_code_id);

            init_msg.cw721_code_id = failing_cw721_code_id.into();

            let init_result = app.instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "Poap contract",
                None,
            );
            assert!(init_result.is_err());
        }

        #[test]
        fn event_end_before_current_time() {
            let mut app = mock_app();
            let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
            let mut init_msg = get_valid_init_msg(&app, cw721_code_id);

            let current_block = app.block_info();
            // Create a start time 200 seconds before the current block time
            let start = current_block.time.seconds() - 200;
            init_msg.event_info.start_time = Timestamp::from_seconds(start);
            // Start time 100 seconds before the current block time
            init_msg.event_info.end_time = Timestamp::from_seconds(start + 100);

            let init_result = app.instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "Poap contract",
                None,
            );

            assert!(init_result.is_err());
        }

        #[test]
        fn event_start_after_end() {
            let mut app = mock_app();
            let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
            let mut init_msg = get_valid_init_msg(&app, cw721_code_id);

            let current_block = app.block_info();
            // Create a start time 200 seconds after the current block time
            let start = current_block.time.seconds() + 200;
            init_msg.event_info.start_time = Timestamp::from_seconds(start);
            // Start time 100 seconds before the event start time
            init_msg.event_info.end_time = Timestamp::from_seconds(start - 100);

            let init_result = app.instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "Poap contract",
                None,
            );

            assert!(init_result.is_err());
        }

        #[test]
        fn event_start_eq_end() {
            let mut app = mock_app();
            let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
            let mut init_msg = get_valid_init_msg(&app, cw721_code_id);

            let current_block = app.block_info();
            // Create a start time 200 seconds after the current block time
            let start = current_block.time.seconds() + 200;
            init_msg.event_info.start_time = Timestamp::from_seconds(start);
            init_msg.event_info.end_time = Timestamp::from_seconds(start);

            let init_result = app.instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "Poap contract",
                None,
            );

            assert!(init_result.is_err());
        }
    }

    mod enable_mint {
        use crate::integration_tests::tests::{proper_instantiate, ADMIN};
        use crate::msg::ExecuteMsg;
        use cosmwasm_std::Addr;
        use cw_multi_test::Executor;

        #[test]
        fn enable_mint() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::EnableMint {};
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();
        }
    }
}
