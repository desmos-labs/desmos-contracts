use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Storage;
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub static CONTRACT_DTAG_KEY: &[u8] = b"contract_dtag";
pub static DTAG_REQUESTS_RECORDS_KEY: &[u8] = b"dtag_request_record";

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub contract_dtag: String,
}

/// Get a writable state singleton
pub fn state_store(storage: &mut dyn Storage) -> Singleton<State> {
    singleton(storage, CONTRACT_DTAG_KEY)
}

/// Get a read-only state singleton
pub fn state_read(storage: &dyn Storage) -> ReadonlySingleton<State> {
    singleton_read(storage, CONTRACT_DTAG_KEY)
}

/// DtagRequestRecord represents a request made from the contract to the user that wants to sell his dtag
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct DtagRequestRecord {
    user: String
}

impl DtagRequestRecord {
    pub fn new(user: String) -> Self {
        Self { user }
    }
}

/// Get a writable bucket
pub fn dtag_requests_records_store(storage: &mut dyn Storage) -> Bucket<DtagRequestRecord> {
    bucket(storage, DTAG_REQUESTS_RECORDS_KEY)
}

/// Get a readable bucket
pub fn dtag_requests_records_read(storage: &dyn Storage) -> ReadonlyBucket<DtagRequestRecord> {
    bucket_read(storage, DTAG_REQUESTS_RECORDS_KEY)
}
