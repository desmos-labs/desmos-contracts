use std::cmp::max;
use std::fmt;
use std::fmt::{Formatter, write};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Storage, Timestamp, Uint64};
use cosmwasm_storage::{
    bucket, bucket_read, singleton, singleton_read, Bucket, ReadonlyBucket, ReadonlySingleton,
    Singleton,
};

pub static CONTRACT_DTAG_KEY: &[u8] = b"contract_dtag";
pub static DTAG_TRANSFER_RECORD_KEY: &[u8] = b"dtag_request_record";
pub static AUCTION_KEY: &[u8] = b"auction";

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
#[serde(untagged)]
pub enum RecordStatus {
    PendingTransferRequest,
    AcceptedTransferRequest,
    AuctionStarted,
    AuctionClosed
}

impl fmt::Display for RecordStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            RecordStatus::PendingTransferRequest => write!(f, "PendingTransferRequest"),
            RecordStatus::AcceptedTransferRequest => write!(f, "AcceptedTransferRequest"),
            RecordStatus::AuctionStarted => write!(f, "AuctionStarted"),
            RecordStatus::AuctionClosed => write!(f, "AuctionClosed")
        }
    }
}

/// DtagAuctionRecord represents an auction and its status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DtagTransferRecord {
    user: String,
    pub status: RecordStatus
}

impl DtagTransferRecord {
    pub fn new(user: String, status: RecordStatus) -> DtagTransferRecord {
        DtagTransferRecord { user, status }
    }
}

/// Get a writable bucket
pub fn dtag_transfer_records_store(storage: &mut dyn Storage) -> Bucket<DtagTransferRecord> {
    bucket(storage, DTAG_TRANSFER_RECORD_KEY)
}

/// Get a readable bucket
pub fn dtag_transfer_records_read(storage: &dyn Storage) -> ReadonlyBucket<DtagTransferRecord> {
    bucket_read(storage, DTAG_TRANSFER_RECORD_KEY)
}

pub struct Auction {
    dtag: String,
    starting_price: Uint64,
    max_participants: Uint64,
    start_time: Option<Timestamp>,
    end_time: Option<Timestamp>,
    user: String,
}

impl Auction {
    pub fn new(
        dtag: String,
        starting_price: Uint64,
        max_participants: Uint64,
        start_time: Option<Timestamp>,
        end_time: Option<Timestamp>,
        user: String
    ) -> Auction {
        Auction {
            dtag,
            starting_price,
            max_participants,
            start_time,
            end_time,
            user
        }
    }
}

/// Get a writable state singleton
pub fn auction_store(storage: &mut dyn Storage) -> Singleton<Auction> {
    singleton(storage, AUCTION_KEY)
}

/// Get a read-only state singleton
pub fn auction_read(storage: &dyn Storage) -> ReadonlySingleton<Auction> {
    singleton_read(storage, AUCTION_KEY)
}
