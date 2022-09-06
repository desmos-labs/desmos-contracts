#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    has_coins, to_binary, wasm_execute, wasm_instantiate, Addr, Binary, Coin, Deps, DepsMut, Empty,
    Env, MessageInfo, Order, Querier, Reply, Response, StdResult, Storage, SubMsg, Uint64,
};
use cw2::set_contract_version;
use cw721_base::{ExecuteMsg as Cw721ExecuteMsg, InstantiateMsg as Cw721InstantiateMsg, MintMsg};
use cw_utils::parse_reply_instantiate_data;
use desmos_bindings::{
    msg::DesmosMsg, query::DesmosQuery,
    reactions::querier::ReactionsQuerier, types::PageRequest,
};
use std::ops::Deref;

use crate::error::ContractError;
use crate::msg::{
    ExecuteMsg, InstantiateMsg, QueryConfigResponse, QueryMsg, QueryRaritiesResponse, Rarity,
};
use crate::state::{ConfigState, RarityState, CONFIG, CW721_ADDRESS, NEXT_TOKEN_ID, RARITY};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:remarkables";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const INSTANTIATE_CW721_REPLY_ID: u64 = 1;

// actions for executing messages
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_INSTANTIATE_CW721_REPLY: &str = "instantiate_cw721_reply";
const ACTION_MINT_TO: &str = "mint_to";
const ACTION_UPDATE_ADMIN: &str = "update_admin";
const ACTION_UPDATE_RARITY_MINT_FEES: &str = "update_rarity_mint_fees";

// attributes for executing messages
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_SENDER: &str = "sender";
const ATTRIBUTE_ADMIN: &str = "admin";
const ATTRIBUTE_CW721_CODE_ID: &str = "cw721_code_id";
const ATTRIBUTE_NEW_ADMIN: &str = "new_admin";
const ATTRIBUTE_RARITY_LEVEL: &str = "rarity_level";
const ATTRIBUTE_RECIPIENT: &str = "recipient";
const ATTRIBUTE_TOKEN_ID: &str = "token_id";
const ATTRIBUTE_TOKEN_URI: &str = "token_uri";

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
        .add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_ADMIN, msg.admin)
        .add_attribute(ATTRIBUTE_CW721_CODE_ID, msg.cw721_code_id)
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
        ExecuteMsg::MintTo {
            post_id,
            remarkables_uri,
            rarity_level,
        } => execute_mint_to(deps, info, rarity_level, post_id.into(), remarkables_uri),
        ExecuteMsg::UpdateAdmin { new_admin } => execute_update_admin(deps, info, new_admin),
        ExecuteMsg::UpdateRarityMintFee {
            rarity_level,
            new_fees,
        } => execute_update_rarity_mint_fees(deps, info, rarity_level, new_fees),
    }
}

fn execute_mint_to(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    rarity_level: u32,
    post_id: u64,
    remarkables_uri: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let rarity = RARITY.load(deps.storage, rarity_level)?;
    // Check if rarity mint fees is enough
    let mut is_enough_fees = false;
    for coin in rarity.mint_fees {
        is_enough_fees = has_coins(&info.funds, &coin)
    }
    if !is_enough_fees {
        return Err(ContractError::MintFeesNotEnough {});
    }
    // Check if post reaches the eligible threshold
    check_eligibility(
        deps.storage,
        deps.querier.deref(),
        post_id,
        rarity.engagement_threshold,
    )?;
    // Create the cw721 message to send to mint the remarkables
    let token_id = NEXT_TOKEN_ID.may_load(deps.storage)?.unwrap_or(1);
    NEXT_TOKEN_ID.save(deps.storage, &(token_id + 1))?;
    let mint_msg = Cw721ExecuteMsg::<Empty, Empty>::Mint(MintMsg::<Empty> {
        token_id: token_id.to_string(),
        owner: info.sender.clone().into(),
        token_uri: Some(remarkables_uri.clone()),
        extension: Empty {},
    });
    let wasm_execute_mint_msg = wasm_execute(CW721_ADDRESS.load(deps.storage)?, &mint_msg, vec![])?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_MINT_TO)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_TOKEN_ID, token_id.to_string())
        .add_attribute(ATTRIBUTE_RECIPIENT, &info.sender)
        .add_attribute(ATTRIBUTE_TOKEN_URI, remarkables_uri)
        .add_message(wasm_execute_mint_msg))
}

fn check_eligibility<'a>(
    storage: &dyn Storage,
    querier: &'a dyn Querier,
    post_id: u64,
    engagement_threshold: u32,
) -> Result<(), ContractError> {
    let subspace_id = CONFIG.load(storage)?.subspace_id;
    let reactions_pagination = ReactionsQuerier::new(querier)
        .query_reactions(
            subspace_id,
            post_id,
            None,
            Some(PageRequest {
                key: None,
                offset: None,
                limit: Uint64::new(1),
                count_total: true,
                reverse: false,
            }),
        )?
        .pagination
        .unwrap();
    let reaction_count = reactions_pagination.total.unwrap();
    if engagement_threshold as u64 > reaction_count.into() {
        return Err(ContractError::NoEligibilityError {});
    }
    Ok(())
}

fn execute_update_admin(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    new_admin: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    check_admin(deps.storage, &info)?;
    let new_admin_addr = deps.api.addr_validate(&new_admin)?;
    CONFIG.update(deps.storage, |mut config| -> Result<_, ContractError> {
        config.admin = new_admin_addr.clone();
        Ok(config)
    })?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_ADMIN)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_ADMIN, &info.sender)
        .add_attribute(ATTRIBUTE_NEW_ADMIN, new_admin_addr))
}

fn execute_update_rarity_mint_fees(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
    level: u32,
    new_fees: Vec<Coin>,
) -> Result<Response<DesmosMsg>, ContractError> {
    check_admin(deps.storage, &info)?;
    RARITY.update(deps.storage, level, |rarity| -> Result<_, ContractError> {
        let mut new_rarity = rarity.ok_or(ContractError::RarityNotExists { level })?;
        new_rarity.mint_fees = new_fees;
        Ok(new_rarity)
    })?;
    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_UPDATE_RARITY_MINT_FEES)
        .add_attribute(ATTRIBUTE_SENDER, &info.sender)
        .add_attribute(ATTRIBUTE_RARITY_LEVEL, level.to_string()))
}

fn check_admin(storage: &dyn Storage, info: &MessageInfo) -> Result<(), ContractError> {
    let config = CONFIG.load(storage)?;
    if config.admin != info.sender {
        return Err(ContractError::NotAdmin {
            caller: info.sender.clone(),
        });
    }
    Ok(())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::Rarities {} => to_binary(&query_rarities(deps)?),
    }
}

fn query_config(deps: Deps<DesmosQuery>) -> StdResult<QueryConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let cw721_address = CW721_ADDRESS.load(deps.storage)?;
    Ok(QueryConfigResponse {
        admin: config.admin,
        cw721_code_id: config.cw721_code_id.into(),
        cw721_address,
        subspace_id: config.subspace_id.into(),
    })
}

fn query_rarities(deps: Deps<DesmosQuery>) -> StdResult<QueryRaritiesResponse> {
    let res: StdResult<Vec<_>> = RARITY
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    let rarities = res?
        .iter()
        .map(|(level, rarity)| Rarity {
            level: *level,
            engagement_threshold: rarity.engagement_threshold,
            mint_fees: rarity.mint_fees.clone(),
        })
        .collect();
    Ok(QueryRaritiesResponse { rarities })
}

// Reply callback triggered from cw721 contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    msg: Reply,
) -> Result<Response<DesmosMsg>, ContractError> {
    if msg.id != INSTANTIATE_CW721_REPLY_ID {
        return Err(ContractError::InvalidReplyID {});
    }
    let reply = parse_reply_instantiate_data(msg);
    match reply {
        Ok(res) => {
            CW721_ADDRESS.save(deps.storage, &Addr::unchecked(res.contract_address))?;
            Ok(Response::default().add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE_CW721_REPLY))
        }
        Err(_) => Err(ContractError::InstantiateCw721Error {}),
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
