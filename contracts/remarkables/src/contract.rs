#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, wasm_instantiate, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    SubMsg,
};
use cw2::set_contract_version;
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;

use desmos_bindings::{msg::DesmosMsg, query::DesmosQuery};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ConfigState, RarityState, CONFIG, RARITY};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:remarkables";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    // Save the config
    let admin_addr = deps.api.addr_validate(&msg.admin)?;
    CONFIG.save(
        deps.storage,
        &ConfigState {
            admin: admin_addr,
            subspace_id: msg.subspace_id.into(),
            cw721_code_id: msg.cw721_code_id.into(),
        },
    )?;
    // Save the info of rarities
    for rarity in msg.rarities {
        let state = RarityState {
            level: rarity.level,
            mint_fees: rarity.mint_fees,
            engagement_threshold: rarity.engagement_threshold,
        };
        RARITY.save(deps.storage, rarity.level, &state)?;
    }
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    // Submessage to instantiate cw721 contract
    let cw721_submessage = SubMsg::reply_on_success(
        wasm_instantiate(
            msg.cw721_code_id.into(),
            &Cw721InstantiateMsg {
                name: msg.cw721_instantiate_msg.name,
                symbol: msg.cw721_instantiate_msg.symbol,
                minter: env.contract.address.to_string(),
            },
            info.funds,
            "remarkables_cw721".to_string(),
        )?,
        INSTANTIATE_CW721_REPLY_ID,
    );
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
        .add_submessage(cw721_submessage))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    match msg {
        ExecuteMsg::MintTo { .. } => Ok(Response::new()),
        ExecuteMsg::UpdateAdmin { .. } => Ok(Response::new()),
        ExecuteMsg::UpdateRarityMintFee { .. } => Ok(Response::new()),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config{} => Binary::from_base64(""),
        QueryMsg::EngagementThresholds{..} => Binary::from_base64(""),
        QueryMsg::RarityMintFees{..} => Binary::from_base64(""),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();
    }
}
