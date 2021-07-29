use crate::{
    msg::{SudoMsg},
    state::{reactions_store, reactions_read, ReactionsAmount},
};
use cosmwasm_std::{
    attr, entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use desmos::{
    custom_query::{query_posts},
    query_types::PostsResponse,
    types::Post,
};
use std::fmt::Error;

#[entry_point]
pub fn sudo(_deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, Error> {
    match msg {
        SudoMsg::ExecuteTokenomics {} => execute_tokenomics()
    }
}
