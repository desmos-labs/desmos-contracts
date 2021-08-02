use crate::{
    msg::{SudoMsg, InstantiateMsg},
    state::{reactions_store, reactions_read, denom_read},
    errors::ContractError
};

use cosmwasm_std::{attr, entry_point, Deps, DepsMut, Env, Response, Coin, BankMsg, CosmosMsg, MessageInfo, Uint128};

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
            .load(post.clone().post_id.as_bytes()).unwrap_or(Uint128(0));

        let calculated_reward = calculate_rewards(
            deps.as_ref(),
            stored_reactions_amount.0,
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
            &Uint128(actual_reactions_amount)
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

#[cfg(test)]
mod tests {
    use cosmwasm_std::{
        DepsMut, Env, MessageInfo, Coin, attr,
        testing::{mock_env, mock_info}, Response, BankMsg, CosmosMsg
    };
    use crate::{
        contract::{instantiate, calculate_rewards, execute_tokenomics},
        msg::InstantiateMsg,
        state::denom_read,
    };
    use desmos::mock::mock_dependencies_with_custom_querier;

    fn setup_test(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        denom: String,
    ) {
        let instantiate_msg = InstantiateMsg{
            token_denom: denom,
        };
        instantiate(deps, env, info, instantiate_msg).unwrap();
    }

    #[test]
    fn test_instantiate() {
        let funds = Coin::new(100_000_000, "udesmos");
        let mut deps = mock_dependencies_with_custom_querier(&[funds]);
        let info = mock_info("addr0001", &[]);

        let instantiate_msg = InstantiateMsg{ token_denom: "udesmos".to_string()};

        let res = instantiate(
            deps.as_mut(),
            mock_env(),
            info,
            instantiate_msg
        ).unwrap();

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

        let actual_rewards = calculate_rewards(
            deps.as_ref(),
            stored_reactions_amount,
            reactions_amount
        ).unwrap();

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

        let _error = calculate_rewards(
            deps.as_ref(),
            stored_reactions_amount,
            reactions_amount
        ).unwrap();
    }

    #[test]
    fn test_execute_tokenomics_successfully() {
        let funds = Coin::new(100_000_000, "udesmos");
        let info = mock_info("addr0001", &[]);
        let mut deps = mock_dependencies_with_custom_querier(&[funds]);
        setup_test(deps.as_mut(), mock_env(), info, "udesmos".to_string());

        let response = execute_tokenomics(deps.as_mut());

        assert!(response.is_ok());

        let exp_response = Response {
            messages: vec![CosmosMsg::from(
                BankMsg::Send {
                    to_address: "default_creator".to_string(),
                    amount: vec![Coin::new(1_000_000, "udesmos")],
                }
            )],
            attributes: vec![
                attr("action", "executed_tokenomics"),
                attr("subspace_id", "subspace"),
            ],
            ..Response::default()
        };

        assert_eq!(exp_response, response.unwrap())
    }
}
