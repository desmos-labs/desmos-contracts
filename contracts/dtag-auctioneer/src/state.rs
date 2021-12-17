use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Storage;
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub static CONTRACT_DTAG_KEY: &[u8] = b"contract_dtag";
pub static DTAG_AUCTION_STATUS_KEY: &[u8] = b"dtag_request_record";

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

/// Auction status represent the different status in which an auction can be
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AuctionStatus {
    PendingTransferRequest,
    AcceptedTransferRequest,
    AuctionStarted,
    AuctionClosed
}

/// DtagAuctionRecord represents an auction and its status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DtagAuctionStatus {
    user: String,
    pub status: AuctionStatus
}

impl DtagAuctionStatus {
    pub fn new(user: String, status: AuctionStatus) -> Self {
        Self { user, status }
    }
}

/// Get a writable bucket
pub fn dtag_auction_records_store(storage: &mut dyn Storage) -> Bucket<DtagAuctionStatus> {
    bucket(storage, DTAG_AUCTION_STATUS_KEY)
}

/// Get a readable bucket
pub fn dtag_requests_records_read(storage: &dyn Storage) -> ReadonlyBucket<DtagAuctionStatus> {
    bucket_read(storage, DTAG_AUCTION_STATUS_KEY)
}
