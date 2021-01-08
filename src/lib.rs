pub mod contract;
pub mod error;
pub mod msg;
pub mod state;
mod types;
mod query;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
