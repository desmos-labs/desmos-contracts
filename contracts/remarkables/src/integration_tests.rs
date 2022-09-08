#[cfg(test)]
mod tests {
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, Rarity};
    use crate::test_utils::*;
    use crate::ContractError;
    use cosmwasm_std::{coins, wasm_execute, Addr};
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use cw_multi_test::{Contract, ContractWrapper, Executor};
    use desmos_bindings::{
        mocks::mock_apps::{
            custom_desmos_app, mock_failing_desmos_app, DesmosApp, FailingDesmosApp,
        },
        msg::DesmosMsg,
        query::DesmosQuery,
    };

    pub fn contract_remarkables() -> Box<dyn Contract<DesmosMsg, DesmosQuery>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        )
        .with_reply(crate::contract::reply);
        Box::new(contract)
    }

    const ADMIN: &str = "admin";
    const DENOM: &str = "denom";
    const UNACCEPTED_RARITY_LEVEL: u32 = 0;
    const ACCEPTED_RARITY_LEVEL: u32 = 1;

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
            subspace_id: 1u64.into(),
            rarities: vec![
                Rarity {
                    level: 0,
                    engagement_threshold: 100,
                    mint_fees: vec![],
                },
                Rarity {
                    level: 1,
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

    fn proper_instantiate_failing_app() -> (FailingDesmosApp, Addr, (u64, u64)) {
        let mut app = mock_failing_desmos_app();
        let (cw721_code_id, remarkables_code_id) = store_contracts_to_failing_app(&mut app);
        let poap_manager_contract_addr = app
            .instantiate_contract(
                remarkables_code_id,
                Addr::unchecked(ADMIN),
                &get_valid_init_msg(cw721_code_id),
                &[],
                "remarkables_contract",
                None,
            )
            .unwrap();
        (
            app,
            poap_manager_contract_addr,
            (cw721_code_id, remarkables_code_id),
        )
    }

    mod instantiate {
        use super::*;
        #[test]
        fn instantiate_with_invalid_poap_code_id_error() {
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
        fn instantiate_propery() {
            let (app, addr, (cw721_code_id, _)) = proper_instantiate();
            let querier = app.wrap();
            // check config set properly
            let config: QueryConfigResponse = querier
                .query_wasm_smart(&addr, &QueryMsg::Config {})
                .unwrap();
            assert_eq!(config.admin, ADMIN);
            assert_eq!(config.cw721_code_id, cw721_code_id.into())
        }
    }
    mod mint_to {
        use super::*;
        #[test]
        fn mint_to_with_non_existing_post_error() {
            let (mut app, addr, _) = proper_instantiate_failing_app();
            let result = app.execute(
                Addr::unchecked(ADMIN),
                wasm_execute(
                    &addr,
                    &ExecuteMsg::MintTo {
                        post_id: 1u64.into(),
                        remarkables_uri: "ipfs://test.com".into(),
                        rarity_level: ACCEPTED_RARITY_LEVEL,
                    },
                    vec![],
                )
                .unwrap()
                .into(),
            );
            assert!(result.is_err())
        }
        #[test]
        fn mint_to_without_eligibility_error() {
            let (mut app, addr, _) = proper_instantiate();
            let result = app.execute(
                Addr::unchecked(ADMIN),
                wasm_execute(
                    &addr,
                    &ExecuteMsg::MintTo {
                        post_id: 1u64.into(),
                        remarkables_uri: "ipfs://test.com".into(),
                        rarity_level: UNACCEPTED_RARITY_LEVEL,
                    },
                    vec![],
                )
                .unwrap()
                .into(),
            );
            assert!(result.is_err())
        }
        #[test]
        fn mint_to_properly() {
            let (mut app, addr, _) = proper_instantiate();
            app.execute(
                Addr::unchecked(ADMIN),
                wasm_execute(
                    &addr,
                    &ExecuteMsg::MintTo {
                        post_id: 1u64.into(),
                        remarkables_uri: "ipfs://test.com".into(),
                        rarity_level: ACCEPTED_RARITY_LEVEL,
                    },
                    vec![],
                )
                .unwrap()
                .into(),
            ).unwrap();
        }
    }
}
