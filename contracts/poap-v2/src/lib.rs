mod error;
mod execute;
pub mod msg;
mod query;
pub mod state;

use cosmwasm_std::Empty;

pub use crate::error::ContractError;
pub use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
pub use crate::state::PoapContract;

// This type is re-exported so that contracts interacting with this
// one don't need a direct dependency on cw721_base to use the API.
pub use cw721_base::MinterResponse;

// These types are re-exported so that contracts interacting with this
// one don't need a direct dependency on cw_ownable to use the API.
//
// `Action` is used in `ExecuteMsg::UpdateOwnership`, `Ownership` is
// used in `QueryMsg::Ownership`, and `OwnershipError` is used in
// `ContractError::Ownership`.
pub use cw_ownable::{Action, Ownership, OwnershipError};

// This is a simple type to let us handle empty extensions
pub type Extension = Option<Empty>;

// Version info for migration
pub const CONTRACT_NAME: &str = "crates.io:poap-v2";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub mod entry {
    use super::*;
    #[cfg(not(feature = "library"))]
    use cosmwasm_std::entry_point;
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn instantiate(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> StdResult<Response> {
        cw2::set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

        let tract = PoapContract::<Extension, Empty, Empty, Empty>::default();
        tract.instantiate(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg<Extension, Empty>,
    ) -> Result<Response, ContractError> {
        let tract = PoapContract::<Extension, Empty, Empty, Empty>::default();
        tract.execute(deps, env, info, msg)
    }

    #[cfg_attr(not(feature = "library"), entry_point)]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg<Empty>) -> StdResult<Binary> {
        let tract = PoapContract::<Extension, Empty, Empty, Empty>::default();
        tract.query(deps, env, msg)
    }
}
