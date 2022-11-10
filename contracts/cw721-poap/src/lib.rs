use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Empty};
pub use cw721_base::{ContractError, InstantiateMsg, MintMsg, MinterResponse};
use desmos_bindings::{msg::DesmosMsg, query::DesmosQuery};

#[cw_serde]
pub struct Metadata {
    pub claimer: Addr,
}

pub type Cw721MetadataContract<'a> =
    cw721_base::Cw721Contract<'a, Metadata, Empty, Empty, DesmosMsg, DesmosQuery>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Metadata, Empty>;
pub type QueryMsg = cw721_base::QueryMsg<Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
    use cw2::set_contract_version;

    // Version info for migration
    const CONTRACT_NAME: &str = "crates.io:cw721-poap";
    const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut<DesmosQuery>,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response<DesmosMsg>, ContractError> {
        let res = Cw721MetadataContract::default().instantiate(deps.branch(), env, info, msg)?;
        // Explicitly set contract name and version, otherwise set to cw721-base info
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)
            .map_err(ContractError::Std)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut<DesmosQuery>,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response<DesmosMsg>, ContractError> {
        Cw721MetadataContract::default().execute(deps, env, info, msg)
    }

    #[entry_point]
    pub fn query(deps: Deps<DesmosQuery>, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        Cw721MetadataContract::default().query(deps, env, msg)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cw721::Cw721Query;
    use desmos_bindings::mocks::mock_queriers::mock_desmos_dependencies;

    const CREATOR: &str = "creator";

    #[test]
    fn use_metadata_extension() {
        let mut deps = mock_desmos_dependencies();
        let contract = Cw721MetadataContract::default();

        let info = mock_info(CREATOR, &[]);
        let init_msg = InstantiateMsg {
            name: "SpaceShips".to_string(),
            symbol: "SPACE".to_string(),
            minter: CREATOR.to_string(),
        };
        contract
            .instantiate(deps.as_mut(), mock_env(), info.clone(), init_msg)
            .unwrap();

        let token_id = "Enterprise";
        let mint_msg = MintMsg {
            token_id: token_id.to_string(),
            owner: "john".to_string(),
            token_uri: Some("https://starships.example.com/Starship/Enterprise.json".into()),
            extension: Metadata {
                claimer: Addr::unchecked("claimer"),
            },
        };
        let exec_msg = ExecuteMsg::Mint(mint_msg.clone());
        contract
            .execute(deps.as_mut(), mock_env(), info, exec_msg)
            .unwrap();

        let res = contract.nft_info(deps.as_ref(), token_id.into()).unwrap();
        assert_eq!(res.token_uri, mint_msg.token_uri);
        assert_eq!(res.extension, mint_msg.extension);
    }
}
