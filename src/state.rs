use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Storage;
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static REPORTS_LIMIT_KEY: &[u8] = b"reports_limit";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub default_reports_limit: u16,
}

/// Get a writable state singleton
pub fn state_store(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, REPORTS_LIMIT_KEY)
}

/// Get a read-only state singleton
pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, REPORTS_LIMIT_KEY)
}
