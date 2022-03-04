#[cfg(not(target_arch = "wasm32"))]
pub mod mock;

pub mod msg;
pub mod subspaces;
pub mod types;
pub mod query;