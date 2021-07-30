use crate::{
    msg::{SudoMsg},
    state::{reactions_store, reactions_read, ReactionsAmount},
    errors::ContractError;
};
use cosmwasm_std::{attr, entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Coin, Uint128};
use desmos::{
    custom_query::{query_posts},
    query_types::PostsResponse,
    types::Post,
};
use desmos::custom_query::query_post_reactions;
use crate::state::denom_read;

#[entry_point]
pub fn sudo(_deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::ExecuteTokenomics {} => execute_tokenomics(deps, env, info, msg)
    }
}

fn execute_tokenomics(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: SudoMsg,
) -> Result<Response, ContractError > {
    let posts = query_posts(&deps.querier)?
        .posts
        .iter()
        .map(| &post |
            calculate_rewards(deps, post)

        );

}

fn calculate_rewards(deps: DepsMut, post: Post) -> Result<Coin, ContractError> {
    let reactions_amount = query_post_reactions(&deps.querier, post.post_id)?
        .reactions
        .len();

    let previous_reactions_amount = reactions_read(deps.storage)
        .load(post.post_id.as_bytes())?.reactions_number;

    let mut rewards_amount: u128 = 0;
    if reactions_amount > previous_reactions_amount {
        let new_reactions = (reactions_amount - previous_reactions_amount) as u64;
        rewards_amount = (new_reactions * 1_000_000) as u128;
    };

    let rewards = Coin::new(rewards_amount, denom_read(deps.storage)?);
    Ok(rewards)
}

fn send_rewards(deps: DepsMut, rewards: Coin, user: String) {

}
