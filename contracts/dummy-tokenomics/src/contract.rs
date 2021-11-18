use crate::{
    errors::ContractError,
    msg::{InstantiateMsg, SudoMsg},
    state::{denom_read, denom_store, reactions_read, reactions_store},
};
use cosmwasm_std::{attr, entry_point, BankMsg, Coin, Env, MessageInfo, Response, Uint128};
use desmos_cw::{
    querier::DesmosQuerier,
    types::{Deps, DepsMut},
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    denom_store(deps.storage).save(&msg.token_denom)?;

    let res = Response::new().add_attributes(vec![attr("action", "set_token_denom")]);
    Ok(res)
}

#[entry_point]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    match msg {
        SudoMsg::ExecuteTokenomics {} => execute_tokenomics(deps),
    }
}

/// execute_tokenomics takes care of distribute the rewards across all the users whose posts
/// have received some reaction
fn execute_tokenomics(deps: DepsMut) -> Result<Response, ContractError> {
    let mut msgs: Vec<BankMsg> = Vec::new();
    let mut subspace_id: String = "".to_string();

    // querying posts from Desmos chain
    let querier = DesmosQuerier::new(&deps.querier);
    let posts = querier.query_posts()?.posts;

    for post in posts.iter() {
        let actual_reactions_amount = querier
            .query_post_reactions(post.clone().post_id)?
            .reactions
            .into_iter()
            .filter(|reaction| reaction.owner != post.creator)
            .count() as u128;

        let stored_reactions_amount = reactions_read(deps.storage)
            .load(post.clone().post_id.as_bytes())
            .unwrap_or_else(|_| Uint128::new(0));

        let calculated_reward = calculate_rewards(
            deps.as_ref(),
            u128::from(stored_reactions_amount),
            actual_reactions_amount,
        )?;

        if !calculated_reward[0].amount.is_zero() {
            let msg = BankMsg::Send {
                to_address: post.clone().creator,
                amount: calculated_reward,
            };

            msgs.push(msg);
        }

        reactions_store(deps.storage).save(
            post.clone().post_id.as_bytes(),
            &Uint128::new(actual_reactions_amount),
        )?;

        subspace_id = post.subspace.clone()
    }

    let response = Response::new().add_messages(msgs).add_attributes(vec![
        attr("action", "executed_tokenomics"),
        attr("subspace_id", subspace_id),
    ]);

    Ok(response)
}

/// calculate_rewards calculate the rewards based on the difference between the current post
/// reactions and those saved inside the contract store
fn calculate_rewards(
    deps: Deps,
    stored_reactions_amount: u128,
    actual_reactions_amount: u128,
) -> Result<Vec<Coin>, ContractError> {
    let mut rewards_amount: u128 = 0;
    if actual_reactions_amount > stored_reactions_amount {
        let new_reactions = actual_reactions_amount - stored_reactions_amount;
        rewards_amount = new_reactions * 1_000_000;
    };

    let denom = denom_read(deps.storage).load()?;
    let rewards: Vec<Coin> = vec![Coin::new(rewards_amount, denom)];
    Ok(rewards)
}

#[cfg(test)]
mod tests {
    use crate::{
        contract::{calculate_rewards, execute_tokenomics, instantiate, DepsMut},
        msg::InstantiateMsg,
        state::denom_read,
    };
    use cosmwasm_std::{
        attr,
        testing::{mock_env, mock_info},
        BankMsg, Coin, Env, MessageInfo, Response,
    };
    use desmos_cw::mock::mock_dependencies_with_custom_querier;

    fn setup_test(deps: DepsMut, env: Env, info: MessageInfo, denom: String) {
        let instantiate_msg = InstantiateMsg { token_denom: denom };
        instantiate(deps, env, info, instantiate_msg).unwrap();
    }

    #[test]
    fn test_instantiate() {
        let funds = Coin::new(100_000_000, "udesmos");
        let mut deps = mock_dependencies_with_custom_querier(&[funds]);
        let info = mock_info("addr0001", &[]);

        let instantiate_msg = InstantiateMsg {
            token_denom: "udesmos".to_string(),
        };

        let res = instantiate(deps.as_mut(), mock_env(), info, instantiate_msg).unwrap();

        let exp_log = vec![attr("action", "set_token_denom")];
        assert_eq!(res.attributes, exp_log);

        let denom = denom_read(&deps.storage).load().unwrap();
        assert_eq!("udesmos".to_string(), denom)
    }

    #[test]
    fn test_calculate_rewards() {
        let funds = Coin::new(100_000_000, "udesmos");
        let info = mock_info("addr0001", &[]);
        let mut deps = mock_dependencies_with_custom_querier(&[funds]);
        setup_test(deps.as_mut(), mock_env(), info, "udesmos".to_string());

        let stored_reactions_amount: u128 = 5;
        let reactions_amount: u128 = 7;

        let actual_rewards =
            calculate_rewards(deps.as_ref(), stored_reactions_amount, reactions_amount).unwrap();

        let exp_rewards = vec![Coin::new(2_000_000, "udesmos")];

        assert_eq!(exp_rewards[0], actual_rewards[0])
    }

    #[test]
    #[should_panic]
    fn test_calculate_rewards_error() {
        let funds = Coin::new(100_000_000, "udesmos");
        let deps = mock_dependencies_with_custom_querier(&[funds]);

        let stored_reactions_amount: u128 = 5;
        let reactions_amount: u128 = 7;

        let _error =
            calculate_rewards(deps.as_ref(), stored_reactions_amount, reactions_amount).unwrap();
    }

    #[test]
    fn test_execute_tokenomics_successfully() {
        let funds = Coin::new(100_000_000, "udesmos");
        let info = mock_info("addr0001", &[]);
        let mut deps = mock_dependencies_with_custom_querier(&[funds]);
        setup_test(deps.as_mut(), mock_env(), info, "udesmos".to_string());

        let response = execute_tokenomics(deps.as_mut());

        assert!(response.is_ok());

        let exp_response = Response::new()
            .add_messages(vec![BankMsg::Send {
                to_address: "default_creator".to_string(),
                amount: vec![Coin::new(1_000_000, "udesmos")],
            }])
            .add_attributes(vec![
                attr("action", "executed_tokenomics"),
                attr("subspace_id", "subspace"),
            ]);

        assert_eq!(exp_response, response.unwrap())
    }
}
