use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};
use crate::types::Post;

pub static REPORTS_LIMIT_KEY: &[u8] = b"reports_limit";
pub static POSTS_KEY: &[u8] = b"posts";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub default_reports_limit: u16
}

/// Get a writable state singleton
pub fn state_store(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, REPORTS_LIMIT_KEY)
}

/// Get a read-only state singleton
pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, REPORTS_LIMIT_KEY)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Posts {
    pub posts: Vec<Post>
}

/// Get a writable posts singleton
pub fn posts_store(storage: &mut dyn Storage) -> Singleton<Posts> {
    singleton(storage, POSTS_KEY)
}

/// Get a read-only state singleton
pub fn posts_store_read(storage: &dyn Storage) -> ReadonlySingleton<Posts> {
    singleton_read(storage, POSTS_KEY)
}
