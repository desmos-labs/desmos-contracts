pub mod contract;
pub mod error;
pub mod msg;
pub mod state;

#[cfg(not(target_arch = "wasm32"))]
pub mod mock;

#[cfg(target_arch = "wasm32")]
cosmwasm_std::create_entry_points!(contract);
