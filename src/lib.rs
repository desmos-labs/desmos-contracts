pub mod contract;
pub mod error;
pub mod msg;
pub mod query;
pub mod state;
pub mod types;
mod contract_test;
mod mock;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
