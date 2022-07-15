#[cfg(test)]
mod tests {
    use crate::helpers::CwTemplateContract;
    use crate::msg::{EventInfo, InstantiateMsg};
    use cosmwasm_std::{
        coins, Addr, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdError, StdResult,
        Timestamp,
    };
    use cw721_base::{
        ContractError as Cw721ContractError, Cw721Contract, ExecuteMsg as Cw721ExecuteMsg,
        Extension, InstantiateMsg as Cw721InstantiateMsg, QueryMsg as Cw721QueryMsg,
    };
    use cw_multi_test::{App, AppBuilder, Contract, ContractWrapper, Executor};

    const CREATOR: &str = "desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3";
    const ADMIN: &str = "desmos1jnpfa06xhflyjh6klwlrq8mk55s53czh6ncdm3";
    const USER: &str = "desmos1ptvq7l4jt7n9sc3fky22mfvc6waf2jd8nuc0jv";
    const NATIVE_DENOM: &str = "udsm";
    const CREATION_FEE: u128 = 1_000_000_000;
    const INITIAL_BALANCE: u128 = 2_000_000_000;

    fn cw721_execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721ExecuteMsg<Extension>,
    ) -> Result<Response, Cw721ContractError> {
        Cw721Contract::<'static, Extension, Empty>::default().execute(deps, env, info, msg)
    }

    fn cw721_instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: Cw721InstantiateMsg,
    ) -> Result<Response, StdError> {
        Cw721Contract::<'static, Extension, Empty>::default().instantiate(deps, env, info, msg)
    }

    fn cw721_query(deps: Deps, env: Env, msg: Cw721QueryMsg) -> StdResult<Binary> {
        Cw721Contract::<'static, Extension, Empty>::default().query(deps, env, msg)
    }

    pub fn contract_cw721() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(cw721_execute, cw721_instantiate, cw721_query);
        Box::new(contract)
    }

    pub fn contract_poap() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        )
        .with_reply(crate::contract::reply);
        Box::new(contract)
    }
    fn mock_app() -> App {
        AppBuilder::new().build(|router, _, storage| {
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

    fn proper_instantiate() -> (App, CwTemplateContract) {
        let mut app = mock_app();
        let cw721_code_id = app.store_code(contract_cw721());
        let poap_code_id = app.store_code(contract_poap());

        let block_info = app.block_info();
        let start_time = Timestamp::from_seconds(block_info.time.seconds() + 3600);
        let end_time = Timestamp::from_seconds(start_time.seconds() + 3600);

        let msg = InstantiateMsg {
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
        };

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

    mod count {
        use super::*;
        use crate::msg::ExecuteMsg;

        #[test]
        fn count() {
            let (mut app, cw_template_contract) = proper_instantiate();

            let msg = ExecuteMsg::MintTo {
                recipient: USER.to_string(),
            };
            let cosmos_msg = cw_template_contract.call(msg).unwrap();
            app.execute(Addr::unchecked(ADMIN), cosmos_msg).unwrap();
        }
    }
}
