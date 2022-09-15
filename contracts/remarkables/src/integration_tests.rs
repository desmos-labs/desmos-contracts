#[cfg(test)]
mod tests {
    use crate::contract::convert_post_id_to_token_id;
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, Rarity};
    use crate::test_utils::*;
    use cosmwasm_std::{wasm_execute, Addr, Empty, Uint64};
    use cw721::{AllNftInfoResponse, NftInfoResponse, OwnerOfResponse, TokensResponse};
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use cw721_base::QueryMsg as Cw721QueryMsg;
    use cw_multi_test::{Contract, ContractWrapper, Executor};
    use desmos_bindings::{
        mocks::mock_apps::{
            custom_desmos_app, mock_failing_desmos_app, DesmosApp, FailingDesmosApp,
        },
        msg::DesmosMsg,
        query::DesmosQuery,
    };

    const ADMIN: &str = "cosmos17qcf9sv5yk0ly5vt3ztev70nwf6c5sprkwfh8t";
    const RARITY_LEVEL: u32 = 1;
    const POST_ID: Uint64 = Uint64::new(1);
    const REMARKABLES_URI: &str = "ipfs://remarkables.com";
    const AUTHOR: &str = "desmos1nwp8gxrnmrsrzjdhvk47vvmthzxjtphgxp5ftc";

    fn contract_remarkables() -> Box<dyn Contract<DesmosMsg, DesmosQuery>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        )
        .with_reply(crate::contract::reply);
        Box::new(contract)
    }
    fn store_contracts(app: &mut DesmosApp) -> (u64, u64) {
        let cw721_code_id = app.store_code(CW721TestContract::success_contract());
        let remarkables_code_id = app.store_code(contract_remarkables());
        (cw721_code_id, remarkables_code_id)
    }
    fn store_contracts_to_failing_app(app: &mut FailingDesmosApp) -> (u64, u64) {
        let cw721_code_id = app.store_code(CW721TestContract::success_contract());
        let remarkables_code_id = app.store_code(contract_remarkables());
        (cw721_code_id, remarkables_code_id)
    }
    fn mock_app() -> DesmosApp {
        custom_desmos_app(|router, _, storage| {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(ADMIN), vec![])
                .unwrap();
        })
    }
    fn get_valid_init_msg(cw721_code_id: u64) -> InstantiateMsg {
        InstantiateMsg {
            admin: ADMIN.into(),
            cw721_code_id: cw721_code_id.into(),
            cw721_instantiate_msg: Cw721InstantiateMsg {
                minter: "".into(),
                name: "test".into(),
                symbol: "test".into(),
            },
            subspace_id: POST_ID.into(),
            rarities: vec![
                Rarity {
                    engagement_threshold: 100,
                    mint_fees: vec![],
                },
                Rarity {
                    engagement_threshold: 0,
                    mint_fees: vec![],
                },
            ],
        }
    }
    fn proper_instantiate() -> (DesmosApp, Addr, (u64, u64)) {
        let mut app = mock_app();
        let (cw721_code_id, remarkables_code_id) = store_contracts(&mut app);
        let addr = app
            .instantiate_contract(
                remarkables_code_id,
                Addr::unchecked(ADMIN),
                &get_valid_init_msg(cw721_code_id),
                &[],
                "remarkables_contract",
                None,
            )
            .unwrap();
        (app, addr, (cw721_code_id, remarkables_code_id))
    }
    mod instantiate {
        use super::*;
        #[test]
        fn instantiate_with_invalid_cw721_code_id_error() {
            let mut app = mock_app();
            let (cw721_code_id, remarkables_code_id) = store_contracts(&mut app);
            let mut init_msg = get_valid_init_msg(cw721_code_id);
            // change code cw721_code_id to the invalid one
            init_msg.cw721_code_id = 0u64.into();
            let init_result = app.instantiate_contract(
                remarkables_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "remarkables_contract",
                None,
            );
            assert!(init_result.is_err());
        }
        #[test]
        fn instantiate_with_failing_cw721_contract_error() {
            let mut app = mock_app();
            let (_, remarkables_code_id) = store_contracts(&mut app);
            let failing_cw721_code_id = app.store_code(CW721TestContract::failing_contract());
            let mut init_msg = get_valid_init_msg(failing_cw721_code_id);
            // change id to the failing one
            init_msg.cw721_code_id = failing_cw721_code_id.into();
            let init_result = app.instantiate_contract(
                remarkables_code_id,
                Addr::unchecked(ADMIN),
                &init_msg,
                &[],
                "remarkables_contract",
                None,
            );
            assert!(init_result.is_err());
        }
        #[test]
        fn proper_instantiate_failing_app_error() {
            let mut app = mock_failing_desmos_app();
            let (cw721_code_id, remarkables_code_id) = store_contracts_to_failing_app(&mut app);
            let result = app.instantiate_contract(
                remarkables_code_id,
                Addr::unchecked(ADMIN),
                &get_valid_init_msg(cw721_code_id),
                &[],
                "remarkables_contract",
                None,
            );
            assert!(result.is_err());
        }
        #[test]
        fn instantiate_propery() {
            let (app, addr, (cw721_code_id, _)) = proper_instantiate();
            let querier = app.wrap();
            // check config set properly
            let config: QueryConfigResponse = querier
                .query_wasm_smart(&addr, &QueryMsg::Config {})
                .unwrap();
            assert_eq!(config.admin, ADMIN);
            assert_eq!(config.cw721_code_id.u64(), cw721_code_id)
        }
    }
    mod mint {
        use super::*;
        #[test]
        fn mint_properly() {
            let (mut app, addr, _) = proper_instantiate();
            app.execute(
                Addr::unchecked(AUTHOR),
                wasm_execute(
                    &addr,
                    &ExecuteMsg::Mint {
                        post_id: POST_ID,
                        remarkables_uri: REMARKABLES_URI.into(),
                        rarity_level: RARITY_LEVEL,
                    },
                    vec![],
                )
                .unwrap()
                .into(),
            )
            .unwrap();
            let querier = app.wrap();
            let config: QueryConfigResponse = querier
                .query_wasm_smart(addr, &QueryMsg::Config {})
                .unwrap();
            let response: TokensResponse = querier
                .query_wasm_smart(
                    config.cw721_address.as_str(),
                    &Cw721QueryMsg::<Empty>::Tokens {
                        owner: AUTHOR.to_string(),
                        start_after: None,
                        limit: None,
                    },
                )
                .unwrap();
            let token_id = convert_post_id_to_token_id(POST_ID.into(), RARITY_LEVEL);
            assert_eq!(vec![token_id.clone()], response.tokens);
            let minted_nft_info: NftInfoResponse<Empty> = querier
                .query_wasm_smart(
                    config.cw721_address.as_str(),
                    &Cw721QueryMsg::<Empty>::NftInfo { token_id },
                )
                .unwrap();
            assert_eq!(
                NftInfoResponse {
                    token_uri: Some(REMARKABLES_URI.into()),
                    extension: Empty {},
                },
                minted_nft_info
            )
        }
    }
    mod query {
        use super::*;
        #[test]
        fn query_tokens() {
            let (mut app, addr, _) = proper_instantiate();
            app.execute(
                Addr::unchecked(AUTHOR),
                wasm_execute(
                    &addr,
                    &ExecuteMsg::Mint {
                        post_id: POST_ID,
                        remarkables_uri: REMARKABLES_URI.into(),
                        rarity_level: RARITY_LEVEL,
                    },
                    vec![],
                )
                .unwrap()
                .into(),
            )
            .unwrap();
            let querier = app.wrap();
            let config: QueryConfigResponse = querier
                .query_wasm_smart(&addr, &QueryMsg::Config {})
                .unwrap();
            let querier = app.wrap();
            let response: TokensResponse = querier
                .query_wasm_smart(
                    &addr,
                    &QueryMsg::Tokens {
                        owner: AUTHOR.into(),
                        start_after: None,
                        limit: None,
                    },
                )
                .unwrap();
            let cw721_response: TokensResponse = querier
                .query_wasm_smart(
                    config.cw721_address.as_str(),
                    &Cw721QueryMsg::<Empty>::Tokens {
                        owner: AUTHOR.to_string(),
                        start_after: None,
                        limit: None,
                    },
                )
                .unwrap();
            assert_eq!(cw721_response, response);
            assert_eq!(1, response.tokens.len());
        }

        #[test]
        fn query_nft_info() {
            let (mut app, addr, _) = proper_instantiate();
            app.execute(
                Addr::unchecked(AUTHOR),
                wasm_execute(
                    &addr,
                    &ExecuteMsg::Mint {
                        post_id: POST_ID,
                        remarkables_uri: REMARKABLES_URI.into(),
                        rarity_level: RARITY_LEVEL,
                    },
                    vec![],
                )
                .unwrap()
                .into(),
            )
            .unwrap();
            let querier = app.wrap();
            let config: QueryConfigResponse = querier
                .query_wasm_smart(&addr, &QueryMsg::Config {})
                .unwrap();

            let querier = app.wrap();
            let response: TokensResponse = querier
                .query_wasm_smart(
                    &addr,
                    &QueryMsg::Tokens {
                        owner: AUTHOR.to_string(),
                        start_after: None,
                        limit: None,
                    },
                )
                .unwrap();
            assert_eq!(1, response.tokens.len());
            let cw721_response: AllNftInfoResponse<Empty> = querier
                .query_wasm_smart(
                    config.cw721_address.as_str(),
                    &Cw721QueryMsg::<Empty>::AllNftInfo {
                        token_id: convert_post_id_to_token_id(POST_ID.into(), RARITY_LEVEL),
                        include_expired: None,
                    },
                )
                .unwrap();
            let response: AllNftInfoResponse<Empty> = querier
                .query_wasm_smart(
                    &addr,
                    &QueryMsg::AllNftInfo {
                        token_id: convert_post_id_to_token_id(POST_ID.into(), RARITY_LEVEL),
                        include_expired: None,
                    },
                )
                .unwrap();
            assert_eq!(cw721_response, response);
            assert_eq!(
                AllNftInfoResponse {
                    access: OwnerOfResponse {
                        owner: AUTHOR.to_string(),
                        approvals: vec![]
                    },
                    info: NftInfoResponse {
                        token_uri: Some(REMARKABLES_URI.to_string()),
                        extension: Empty {},
                    }
                },
                response
            );
        }
    }
}
