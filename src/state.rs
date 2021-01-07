use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static _KEY: &[u8] = b"";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
}

pub fn config(storage: &mut dyn Storage) -> Singleton<S, State> {
    singleton(storage, _KEY)
}

pub fn config_read(storage: &dyn Storage) -> ReadonlySingleton<S, State> {
    singleton_read(storage, _KEY)
}
