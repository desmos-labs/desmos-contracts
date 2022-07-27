#[cfg(test)]
mod tests {
    use crate::cw721_test_utils;
    use crate::msg::{
        EventInfo, ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryEventInfoResponse,
        QueryMintedAmountResponse, QueryMsg,
    };
    use cosmwasm_std::{coins, Addr, BlockInfo, Empty, Timestamp, Uint64};
    use cw721_base::{
        InstantiateMsg as Cw721InstantiateMsg, MinterResponse, QueryMsg as Cw721QueryMsg,
    };
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    const CREATOR: &str = "creator";
    const ADMIN: &str = "admin";
    const MINTER: &str = "minter";
    const USER: &str = "user";
    const NATIVE_DENOM: &str = "udsm";
    const INITIAL_BLOCK_TIME_SECONDS: u64 = 3600;
    const EVENT_START_SECONDS: u64 = INITIAL_BLOCK_TIME_SECONDS + 3600;
    const EVENT_END_SECONDS: u64 = EVENT_START_SECONDS + 3600;
    const CREATION_FEE: u128 = 1_000_000_000;
    const INITIAL_BALANCE: u128 = 2_000_000_000;
    const EVENT_URI: &str = "ipfs://event-uri";

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
                time: Timestamp::from_seconds(INITIAL_BLOCK_TIME_SECONDS),
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
        let cw721_code_id = app.store_code(cw721_test_utils::contract_cw721());
        let poap_code_id = app.store_code(contract_poap());

        (cw721_code_id, poap_code_id)
    }

    fn get_valid_init_msg(cw721_code_id: u64) -> InstantiateMsg {
        let start_time = Timestamp::from_seconds(EVENT_START_SECONDS);
        let end_time = Timestamp::from_seconds(EVENT_END_SECONDS);

        InstantiateMsg {
            admin: ADMIN.to_string(),
            minter: MINTER.to_string(),
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
                per_address_limit: 2,
                base_poap_uri: "ipfs://popap-uri".to_string(),
                event_uri: EVENT_URI.to_string(),
            },
        }
    }

    fn proper_instantiate() -> (App, Addr) {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let msg = get_valid_init_msg(cw721_code_id);

        let poap_contract_addr = app
            .instantiate_contract(
                poap_code_id,
                Addr::unchecked(ADMIN),
                &msg,
                &[],
                "Poap contract",
                None,
            )
            .unwrap();

        (app, poap_contract_addr)
    }

    #[test]
    fn instantiate_with_invalid_admin_addr() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

        init_msg.admin = "a".to_string();

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
    fn instantiate_with_invalid_minter_addr() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

        init_msg.minter = "a".to_string();

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
    fn instantiate_with_invalid_creator_addr() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

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
    fn instantiate_with_invalid_cw721_code_id() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

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
    fn instantiate_with_failing_cw721_contract() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let failing_cw721_code_id = app.store_code(cw721_test_utils::failing_cw721());
        let mut init_msg = get_valid_init_msg(cw721_code_id);

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
    fn instantiate_with_event_start_before_current_time() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

        let current_block = app.block_info();
        // Create a start time 200 seconds before the current block time
        let start = current_block.time.seconds() - 200;
        init_msg.event_info.start_time = Timestamp::from_seconds(start);
        init_msg.event_info.end_time = Timestamp::from_seconds(start + 600);

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
    fn instantiate_with_event_start_equal_current_time() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

        let current_block = app.block_info();
        // Create a start time 200 seconds before the current block time
        let start = current_block.time.seconds();
        init_msg.event_info.start_time = Timestamp::from_seconds(start);
        init_msg.event_info.end_time = Timestamp::from_seconds(start + 600);

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
    fn instantiate_with_event_end_before_current_time() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

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
    fn instantiate_with_event_end_equal_current_time() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

        let current_block = app.block_info();
        init_msg.event_info.start_time =
            Timestamp::from_seconds(current_block.time.seconds() + 500);
        init_msg.event_info.end_time = Timestamp::from_seconds(current_block.time.seconds());

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
    fn instantiate_with_event_start_after_end() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

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
    fn instantiate_with_event_start_eq_end() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

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

    #[test]
    fn instantiate_with_invalid_poap_uri() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

        // Invalid uri
        init_msg.event_info.base_poap_uri = "invalid_uri".to_string();
        let init_result = app.instantiate_contract(
            poap_code_id,
            Addr::unchecked(ADMIN),
            &init_msg,
            &[],
            "Poap contract",
            None,
        );
        assert!(init_result.is_err());

        // Non ipfs uri
        init_msg.event_info.base_poap_uri = "https://random_domain.com".to_string();
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
    fn instantiate_with_invalid_event_uri() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id);

        // Invalid uri
        init_msg.event_info.event_uri = "invalid_uri".to_string();
        let init_result = app.instantiate_contract(
            poap_code_id,
            Addr::unchecked(ADMIN),
            &init_msg,
            &[],
            "Poap contract",
            None,
        );
        assert!(init_result.is_err());

        // Non ipfs uri
        init_msg.event_info.event_uri = "https://random_domain.com".to_string();
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
    fn proper_contracts_instantiation() {
        let (app, poap_contract_addr) = proper_instantiate();

        let querier = app.wrap();

        let poap_config: QueryConfigResponse = querier
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::Config {})
            .unwrap();

        assert_eq!(Addr::unchecked(ADMIN), poap_config.admin);
        assert_eq!(Addr::unchecked(MINTER), poap_config.minter);
        assert_eq!(false, poap_config.mint_enabled);
        // 1 since is the first uploaded.
        assert_eq!(Uint64::new(1), poap_config.cw721_contract_code);

        let poap_event_info: QueryEventInfoResponse = querier
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::EventInfo {})
            .unwrap();

        assert_eq!(Addr::unchecked(CREATOR), poap_event_info.creator);
        assert_eq!(
            Timestamp::from_seconds(EVENT_START_SECONDS),
            poap_event_info.start_time
        );
        assert_eq!(
            Timestamp::from_seconds(EVENT_END_SECONDS),
            poap_event_info.end_time
        );
        assert_eq!(EVENT_URI, poap_event_info.event_uri.as_str());

        let cw721_minter_response: MinterResponse = querier
            .query_wasm_smart(&poap_config.cw721_contract, &Cw721QueryMsg::Minter {})
            .unwrap();

        // The cw721 minter should be the poap contract address.
        assert_eq!(poap_contract_addr.to_string(), cw721_minter_response.minter)
    }

    #[test]
    fn enable_mint() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let response: QueryConfigResponse = app
            .wrap()
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::Config {})
            .unwrap();

        assert_eq!(true, response.mint_enabled)
    }

    #[test]
    fn normal_user_can_not_enable_mint() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::EnableMint {};
        let result = app.execute_contract(Addr::unchecked(USER), poap_contract_addr, &msg, &vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn disable_mint() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::DisableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let response: QueryConfigResponse = app
            .wrap()
            .query_wasm_smart(&poap_contract_addr, &QueryMsg::Config {})
            .unwrap();

        assert_eq!(false, response.mint_enabled)
    }

    #[test]
    fn normal_user_can_not_disable_mint() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::DisableMint {};
        let result = app.execute_contract(Addr::unchecked(USER), poap_contract_addr, &msg, &vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn mint_event_not_started() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Enable mint since is disable by default.
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let msg = ExecuteMsg::Mint {};
        let mint_result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );

        // Event is not started
        assert!(mint_result.is_err());
    }

    #[test]
    fn mint_event_terminated() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Enable mint since is disable by default.
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        // Update chain time to event end
        app.update_block(|block_info| block_info.time = Timestamp::from_seconds(EVENT_END_SECONDS));

        let msg = ExecuteMsg::Mint {};
        let mint_result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );

        // Mint should fail if the event is terminated
        assert!(mint_result.is_err())
    }

    #[test]
    fn mint_without_permissions() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Update chain time to event end
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        let msg = ExecuteMsg::Mint {};
        let mint_result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );

        // Mint should fail since mint from user should be enabled.
        assert!(mint_result.is_err())
    }

    #[test]
    fn mint_with_permission_properly() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Change the chain time so that the event is started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        // Enable mint
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        // Mint should work since the event is started and the user is allowed to mint
        let msg = ExecuteMsg::Mint {};
        app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let querier = app.wrap();
        let response: QueryMintedAmountResponse = querier
            .query_wasm_smart(
                &poap_contract_addr,
                &QueryMsg::MintedAmount {
                    user: USER.to_string(),
                },
            )
            .unwrap();

        assert_eq!(Addr::unchecked(USER), response.user);
        assert_eq!(1, response.amount);
    }

    #[test]
    fn mint_limit() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Change the chain time so that the event is started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        // Enable mint
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let msg = ExecuteMsg::Mint {};
        // Mint the first poap
        app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();
        // Mint the second and last allowed poap
        app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Reached the per address limit.
        assert!(result.is_err());

        // Test also with the MintTo
        let msg = ExecuteMsg::MintTo {
            recipient: USER.to_string(),
        };
        let result = app.execute_contract(
            Addr::unchecked(MINTER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());
    }

    #[test]
    fn mint_to_limit() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Change the chain time so that the event is started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        // Enable mint
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let msg = ExecuteMsg::MintTo {
            recipient: USER.to_string(),
        };
        // Mint the first poap
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();
        // Mint the second and last allowed poap
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        let result = app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Should fail since the user have already received the max allowed poaps.
        assert!(result.is_err());

        // Test also with Mint from use
        let msg = ExecuteMsg::Mint {};
        let result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Should fail since the user have already received the max allowed poaps.
        assert!(result.is_err());
    }

    #[test]
    fn mint_to_only_for_minter_and_admin() {
        let (mut app, poap_contract_addr) = proper_instantiate();
        // Change the chain time so that the event is started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        let msg = ExecuteMsg::MintTo {
            recipient: USER.to_string(),
        };
        let mint_result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // User should not be authorized to use the mint to action
        assert!(mint_result.is_err());

        // Test that minter can call mint to
        let msg = ExecuteMsg::MintTo {
            recipient: USER.to_string(),
        };
        app.execute_contract(
            Addr::unchecked(MINTER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        // Test that admin can call mint to
        let msg = ExecuteMsg::MintTo {
            recipient: USER.to_string(),
        };
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();
    }

    #[test]
    fn only_creator_can_change_event_info() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(app.block_info().time.seconds() + 100),
            end_time: Timestamp::from_seconds(app.block_info().time.seconds() + 400),
        };

        let result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // User should not be authorized to update the event info
        assert!(result.is_err());

        let result = app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Admin should not be authorized to update the event info
        assert!(result.is_err());

        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Creator is authorised to update the event info
        assert!(result.is_ok());
    }

    #[test]
    fn event_info_update_only_if_event_not_started_or_ended() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(EVENT_START_SECONDS),
            // Add 300 seconds to prevent end time to be already passed
            end_time: Timestamp::from_seconds(EVENT_END_SECONDS + 300),
        };

        // Fake current time to event in progress
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS + 100)
        });

        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());

        // Edge case current time is event start
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());

        // Fake current time to event ended
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_END_SECONDS + 100)
        });

        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());

        // Edge case current time is event end
        app.update_block(|block_info| block_info.time = Timestamp::from_seconds(EVENT_END_SECONDS));

        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());

        // Current time is before event started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS - 100);
        });

        let mint_result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(mint_result.is_ok());
    }

    #[test]
    fn invalid_event_info() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        // Start time eq end time
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(EVENT_START_SECONDS),
            end_time: Timestamp::from_seconds(EVENT_START_SECONDS),
        };
        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());

        // Start time is after end time
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(EVENT_START_SECONDS + 100),
            end_time: Timestamp::from_seconds(EVENT_START_SECONDS),
        };
        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());

        // Start time before current time
        let current_block = app.block_info();
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(current_block.time.seconds() - 100),
            end_time: Timestamp::from_seconds(EVENT_END_SECONDS),
        };
        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());

        // Edge case start time eq current time
        let current_block = app.block_info();
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(current_block.time.seconds()),
            end_time: Timestamp::from_seconds(EVENT_END_SECONDS),
        };
        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());

        // End time before current time
        let current_block = app.block_info();
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(current_block.time.seconds() + 100),
            end_time: Timestamp::from_seconds(current_block.time.seconds() - 100),
        };
        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());

        // Edge case end time eq current time
        let current_block = app.block_info();
        let msg = ExecuteMsg::UpdateEventInfo {
            start_time: Timestamp::from_seconds(current_block.time.seconds() + 100),
            end_time: Timestamp::from_seconds(current_block.time.seconds()),
        };
        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(result.is_err());
    }

    #[test]
    fn update_admin_only_from_admin() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::UpdateAdmin {
            new_admin: "admin2".to_string(),
        };

        let result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // User can't update admin
        assert!(result.is_err());

        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Creator can't update admin
        assert!(result.is_err());

        let result = app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Admin can update admin
        assert!(result.is_ok());

        let result = app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Admin now can't update the admin anymore since is changed
        assert!(result.is_err());
    }

    #[test]
    fn update_minter_only_from_admin() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::UpdateMinter {
            new_minter: "minter2".to_string(),
        };

        let result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // User can't update minter
        assert!(result.is_err());

        let result = app.execute_contract(
            Addr::unchecked(CREATOR),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Creator can't update minter
        assert!(result.is_err());

        let result = app.execute_contract(
            Addr::unchecked(MINTER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Minter can't update minter
        assert!(result.is_err());

        let result = app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Admin can update minter
        assert!(result.is_ok());
    }
}
