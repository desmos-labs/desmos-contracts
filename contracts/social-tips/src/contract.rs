use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryPendingTipsResponse};
use crate::state::{PendingTip, PendingTips, PENDING_TIPS};
use crate::utils::{serialize_coins, sum_coins_sorted};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, BankMsg, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;
use desmos_bindings::msg::DesmosMsg;
use desmos_bindings::profiles::models_app_links::ApplicationLinkState;
use desmos_bindings::profiles::querier::ProfilesQuerier;
use desmos_bindings::query::DesmosQuery;
use std::ops::Deref;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:social-tips";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// consts
const ATTRIBUTE_ACTION: &str = "action";
const ATTRIBUTE_TIP_COLLECTED: &str = "tip_collected";
const ATTRIBUTE_TIP_RECEIVER: &str = "tip_receiver";
const ATTRIBUTE_TIP_AMOUNT: &str = "tip_amount";
const ACTION_INSTANTIATE: &str = "instantiate";
const ACTION_SEND_TIPS: &str = "send_tips";
const ACTION_CLAIM_PENDING_TIPS: &str = "claim_pending_tips";

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut<DesmosQuery>,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute(ATTRIBUTE_ACTION, ACTION_INSTANTIATE))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<DesmosMsg>, ContractError> {
    msg.validate()?;

    match msg {
        ExecuteMsg::SendTip {
            application,
            handler,
        } => send_tip(deps, env, info, application, handler),
        ExecuteMsg::ClaimTips {} => claim_tips(deps, info),
    }
}

pub fn send_tip(
    deps: DepsMut<DesmosQuery>,
    env: Env,
    info: MessageInfo,
    application: String,
    handler: String,
) -> Result<Response<DesmosMsg>, ContractError> {
    let querier = ProfilesQuerier::new(deps.querier.deref());
    let sender = info.sender;
    let funds = sum_coins_sorted(info.funds)?;

    if funds.is_empty() {
        return Err(ContractError::EmptyTipAmount {});
    }

    // Query users that have that application linked to their accounts.
    let response = querier.query_application_link_owners(
        Some(application.clone()),
        Some(handler.clone()),
        None,
    )?;

    // Fail if there are more then 1 users with the same application link.
    if response.owners.len() > 1 {
        return Err(ContractError::ToManyOwners {});
    }

    let serialized_coins = serialize_coins(&funds);

    return if !response.owners.is_empty() {
        let owner = response.owners.first().unwrap().user.to_string();

        Ok(Response::new()
            .add_attribute(ATTRIBUTE_ACTION, ACTION_SEND_TIPS)
            .add_attribute(ATTRIBUTE_TIP_COLLECTED, "false")
            .add_attribute(ATTRIBUTE_TIP_RECEIVER, &owner)
            .add_attribute(ATTRIBUTE_TIP_AMOUNT, serialized_coins)
            .add_message(BankMsg::Send {
                amount: funds,
                to_address: owner,
            }))
    } else {
        // No one have linked the provided username and application.
        PENDING_TIPS.update::<_, ContractError>(deps.storage, (application, handler), |tips| {
            let mut tips = tips.unwrap_or_default();
            tips.push(PendingTip {
                sender,
                amount: funds,
                block_height: env.block.height,
            });
            Ok(tips)
        })?;

        Ok(Response::new()
            .add_attribute(ATTRIBUTE_ACTION, ACTION_SEND_TIPS)
            .add_attribute(ATTRIBUTE_TIP_COLLECTED, "true")
            .add_attribute(ATTRIBUTE_TIP_AMOUNT, serialized_coins))
    };
}

pub fn claim_tips(
    deps: DepsMut<DesmosQuery>,
    info: MessageInfo,
) -> Result<Response<DesmosMsg>, ContractError> {
    let mut coins = Vec::<Coin>::new();
    let querier = ProfilesQuerier::new(deps.querier.deref());

    for app_link_result in
        querier.iterate_application_links(Some(info.sender.clone()), None, None, 10)
    {
        let app_link = app_link_result?;
        if app_link.state == ApplicationLinkState::VerificationSuccess {
            let key = (app_link.data.application, app_link.data.username);
            let pending_tips = PENDING_TIPS.may_load(deps.storage, key.clone())?;

            if let Some(mut tips) = pending_tips {
                tips.drain(0..)
                    .for_each(|mut tip| tip.amount.drain(0..).for_each(|coin| coins.push(coin)));

                PENDING_TIPS.remove(deps.storage, key);
            }
        }
    }

    let merged_coins = sum_coins_sorted(coins)?;

    if merged_coins.is_empty() {
        return Err(ContractError::NoTipsAvailable {
            user: info.sender.to_string(),
        });
    }

    Ok(Response::new()
        .add_attribute(ATTRIBUTE_ACTION, ACTION_CLAIM_PENDING_TIPS)
        .add_attribute(ATTRIBUTE_TIP_RECEIVER, &info.sender)
        .add_attribute(ATTRIBUTE_TIP_AMOUNT, serialize_coins(&merged_coins))
        .add_message(BankMsg::Send {
            amount: merged_coins,
            to_address: info.sender.to_string(),
        }))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps<DesmosQuery>, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::UserPendingTips { user } => to_binary(&query_user_pending_tips(deps, user)?),
    }
}

fn query_user_pending_tips(
    deps: Deps<DesmosQuery>,
    user: String,
) -> StdResult<QueryPendingTipsResponse> {
    let user_addr = deps.api.addr_validate(&user)?;
    let querier = ProfilesQuerier::new(deps.querier.deref());
    let mut tips = PendingTips::new();

    for app_link_result in querier.iterate_application_links(Some(user_addr), None, None, 10) {
        let app_link = app_link_result?;

        if app_link.state == ApplicationLinkState::VerificationSuccess {
            let key = (app_link.data.application, app_link.data.username);
            let pending_tips = PENDING_TIPS.may_load(deps.storage, key.clone())?;

            if let Some(mut pending_tips) = pending_tips {
                pending_tips.drain(0..).for_each(|tip| tips.push(tip));
            }
        }
    }

    Ok(QueryPendingTipsResponse { tips })
}

#[cfg(test)]
mod tests {}
