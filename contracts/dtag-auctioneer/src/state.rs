use std::fmt;
use std::fmt::{Formatter, write};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp, Uint64};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ContractDTag(String);

pub const CONTRACT_DTAG_STORE: Item<ContractDTag> = Item::new("contract_dtag");

/// Auction status represent the different status in which an auction can be
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum AuctionStatus {
    Active,
    Inactive,
    Pending,
}

impl fmt::Display for AuctionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuctionStatus::Active => write!(f, "Active"),
            AuctionStatus::Inactive => write!(f, "Inactive"),
            AuctionStatus::Pending => write!(f, "Pending"),
        }
    }
}

pub struct Auction {
    dTag: String,
    starting_price: Uint64,
    max_participants: Uint64,
    start_time: Option<Timestamp>,
    end_time: Option<Timestamp>,
    auction_status: AuctionStatus,
    user: String,
}

impl Auction {
    pub fn new(
        dTag: String,
        starting_price: Uint64,
        max_participants: Uint64,
        start_time: Option<Timestamp>,
        end_time: Option<Timestamp>,
        auction_status: AuctionStatus,
        user: String,
    ) -> Auction {
        Auction {
            dTag,
            starting_price,
            max_participants,
            start_time,
            end_time,
            auction_status,
            user,
        }
    }
}

pub const AUCTIONS_STORE: Map<(&Addr, &AuctionStatus), Auction> = Map::new("auctions");

/// DtagAuctionRecord represents an auction and its status
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DtagTransferRecord {
    user: String,
}

impl DtagTransferRecord {
    pub fn new(user: String) -> DtagTransferRecord {
        DtagTransferRecord { user }
    }
}

pub const DTAG_TRANSFER_RECORD: Item<DtagTransferRecord> = Item::new("dtag_transfer_record");
