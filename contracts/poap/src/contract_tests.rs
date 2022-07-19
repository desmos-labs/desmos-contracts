#[cfg(test)]
mod tests {
    use crate::cw721_test_utils;
    use crate::msg::{EventInfo, ExecuteMsg, InstantiateMsg};
    use cosmwasm_std::{coins, Addr, BlockInfo, Empty, Timestamp};
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
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
            cw721_code_id,
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
                event_uri: "ipfs://event-uri".to_string(),
                cw721_code_id: 1,
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
    fn enable_mint() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(Addr::unchecked(ADMIN), poap_contract_addr, &msg, &vec![])
            .unwrap();
    }

    #[test]
    fn normal_user_can_t_enable_mint() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::EnableMint {};
        let result = app.execute_contract(Addr::unchecked(USER), poap_contract_addr, &msg, &vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn disable_mint() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::DisableMint {};
        app.execute_contract(Addr::unchecked(ADMIN), poap_contract_addr, &msg, &vec![])
            .unwrap();
    }

    #[test]
    fn normal_user_can_t_disable_mint() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::DisableMint {};
        let result = app.execute_contract(Addr::unchecked(USER), poap_contract_addr, &msg, &vec![]);
        assert!(result.is_err());
    }

    #[test]
    fn mint() {
        let (mut app, poap_contract_addr) = proper_instantiate();

        let msg = ExecuteMsg::Mint {};
        let mint_result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Event is not started
        assert!(mint_result.is_err());

        // Change the chain time so that the event is started
        app.update_block(|block_info| {
            block_info.time = Timestamp::from_seconds(EVENT_START_SECONDS)
        });

        // Mint should fail since mint is disabled by default
        let msg = ExecuteMsg::Mint {};
        let mint_result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(mint_result.is_err());

        // Enable mint
        let msg = ExecuteMsg::EnableMint {};
        app.execute_contract(
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        // Now mint should work
        let msg = ExecuteMsg::Mint {};
        app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        )
        .unwrap();

        // Mint should not work when the event is terminated
        app.update_block(|block_info| block_info.time = Timestamp::from_seconds(EVENT_END_SECONDS));

        let msg = ExecuteMsg::Mint {};
        let mint_result = app.execute_contract(
            Addr::unchecked(USER),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        assert!(mint_result.is_err())
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

        // Test that minter can call mint to
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
        // Admin should not be authorized to update the event info
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
            Addr::unchecked(ADMIN),
            poap_contract_addr.clone(),
            &msg,
            &vec![],
        );
        // Admin can update minter
        assert!(result.is_ok());
    }
}
