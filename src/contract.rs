use cosmwasm_std::{to_binary, Binary, Env, HandleResponse, InitResponse, MessageInfo, StdResult, Deps, DepsMut};

use crate::error::ContractError;
use crate::msg::{FilteredPostsResponse, HandleMsg, InitMsg, QueryMsg};
use crate::state::{state_store, state_read, State};

// Note, you can use StdResult in some functions where you do not
// make use of the custom errors
pub fn init(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InitMsg,
) -> StdResult<InitResponse> {
    let state = State{ default_reports_limit: msg.reports_limit };
    /// TODO query the posts from desmos here and save it in the store for later usage
    state_store(deps.storage).save(&state)?;
    Ok(InitResponse::default())
}

// And declare a custom Error variant for the ones where you will want to make use of it
pub fn handle(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: HandleMsg,
) -> Result<HandleResponse, ContractError> {
    match msg {

    }
}

pub fn query(
    deps: Deps,
    _env: Env,
    msg: QueryMsg,
) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetFilteredPosts { reports_limit} =>
            to_binary(&query_filtered_posts(deps, reports_limit)?),
    }
}

pub fn query_filtered_posts(deps: Deps, reports_limit: u16) -> StdResult<Binary>{
    /// TODO Query the stored posts and filter them
    /// use separate function to filter posts
}
