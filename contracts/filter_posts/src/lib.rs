pub mod contract;
pub mod custom_query;
pub mod error;
pub mod msg;
pub mod state;
pub mod types;

#[cfg(not(target_arch = "wasm32"))]
pub mod mock;

#[cfg(test)]
mod contract_test;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
