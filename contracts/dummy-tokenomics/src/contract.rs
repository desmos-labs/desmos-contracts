use crate::{
    msg::{SudoMsg, InstantiateMsg},
    state::{reactions_store, reactions_read, denom_read},
    errors::ContractError
};

use cosmwasm_std::{attr, entry_point, Deps, DepsMut, Env, Response, Coin, BankMsg, CosmosMsg, MessageInfo};

use desmos::{
    custom_query::{query_posts, query_post_reactions},
};
use crate::state::denom_store;

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    denom_store(deps.storage).save(&msg.token_denom)?;

    let res = Response {
        attributes: vec![attr("action", "set_token_denom")],
        ..Response::default()
    };
    Ok(res)
}

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response<BankMsg>, ContractError> {
    match msg {
        SudoMsg::ExecuteTokenomics {} => execute_tokenomics(deps)
    }
}

/// execute_tokenomics takes care of distribute the rewards across all the users whose posts
/// have received some reaction
fn execute_tokenomics(
    deps: DepsMut,
) -> Result<Response<BankMsg>, ContractError > {
    let mut msgs: Vec<CosmosMsg<BankMsg>> = Vec::new();
    let mut subspace_id: String = "".to_string();

    // querying posts from Desmos chain
    let posts = query_posts(&deps.querier)?.posts;

    for post in posts.iter() {

        let actual_reactions_amount = query_post_reactions(&deps.querier, post.clone().post_id)?
            .reactions
            .len() as u128;

        let stored_reactions_amount = reactions_read(deps.storage)
            .load(post.clone().post_id.as_bytes())?;

        let calculated_reward = calculate_rewards(
            deps.as_ref(),
            stored_reactions_amount,
            actual_reactions_amount,
        )?;

        let msg = CosmosMsg::from(
            BankMsg::Send {
                to_address: post.clone().creator,
                amount: calculated_reward
            }
        );

        msgs.push(msg);

        reactions_store(deps.storage).save(
            post.clone().post_id.as_bytes(),
            &actual_reactions_amount
        )?;

        subspace_id = post.subspace.clone()
    }

    let response = Response{
        messages: msgs,
        attributes: vec![
            attr("action", "executed_tokenomics"),
            attr("subspace_id", subspace_id),
        ],
        ..Response::default()
    };

    Ok(response)
}

/// calculate_rewards calculate the rewards based on the difference between the current post
/// reactions and those saved inside the contract store
fn calculate_rewards(deps: Deps, stored_reactions_amount: u128, actual_reactions_amount: u128)
    -> Result<Vec<Coin>, ContractError> {
    let mut rewards_amount: u128 = 0;
    if actual_reactions_amount > stored_reactions_amount {
        let new_reactions = actual_reactions_amount - stored_reactions_amount;
        rewards_amount = new_reactions * 1_000_000;
    };

    let denom = denom_read(deps.storage).load()?;
    let rewards: Vec<Coin> = vec![Coin::new(rewards_amount, denom)];
    Ok(rewards)
}
