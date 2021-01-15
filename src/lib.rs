pub mod contract;
mod contract_test;
pub mod custom_query;
pub mod error;
mod mock;
pub mod msg;
pub mod state;
pub mod types;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
