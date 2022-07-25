#[cfg(test)]
mod tests {
    use crate::msg::InstantiateMsg;
    use crate::state::CONFIG;
    use crate::test_utils::*;
    use cosmwasm_std::{Addr, Empty, Timestamp};
    use cosmwasm_std::testing::MockStorage;
    use cw721_base::InstantiateMsg as Cw721InstantiateMsg;
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};
    use poap::msg::{EventInfo, InstantiateMsg as POAPInstantiateMsg};

    const ADMIN: &str = "admin";

    fn mock_app() -> App {
        App::default()
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
                    start_time: Timestamp::from_seconds(0),
                    end_time: Timestamp::from_seconds(10),
                    per_address_limit: 2,
                    base_poap_uri: "ipfs://popap-uri".to_string(),
                    event_uri: "ipfs://event-uri".to_string(),
                    cw721_code_id: 1,
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

        (app, poap_manager_contract_addr)
    }

    #[test]
    fn instantiate_with_invalid_admin_addr_error() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id, poap_manager_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id, poap_code_id);

        init_msg.admin = "a".to_string();

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
    fn instantiate_with_invalid_poap_code_id_error() {
        let mut app = mock_app();
        let (cw721_code_id, poap_code_id, poap_manager_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id, poap_code_id);
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
        init_msg.poap_instantiate_msg.cw721_code_id = poap_code_id;

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
        let (cw721_code_id, poap_code_id, poap_manager_code_id) = store_contracts(&mut app);
        let mut init_msg = get_valid_init_msg(cw721_code_id, poap_code_id);
        let failing_poap_code_id = app.store_code(POAPTestContract::failing_contract());
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
}
