use std::fmt;
use std::fmt::{Formatter};
use std::str::FromStr;
use std::collections::HashMap;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Coin, Timestamp, Uint64};
use cw_storage_plus::{Item, Map};
use crate::error::ContractError;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// This string wrapper represent the contract genesis DTag
pub struct ContractDTag(String);

pub const CONTRACT_DTAG_STORE: Item<ContractDTag> = Item::new("contract_dtag");

/// Auction status represent the different status in which an auction can be
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum AuctionStatus {
    Active,
    Inactive,
}

impl fmt::Display for AuctionStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            AuctionStatus::Active => write!(f, "Active"),
            AuctionStatus::Inactive => write!(f, "Inactive"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
#[serde(untagged)]
pub enum DtagTransferStatus {
    Accepted,
    Refused,
}

impl FromStr for DtagTransferStatus {
    type Err = ContractError::UnknownDTagTransferStatus;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accept_dtag_transfer_request" => Ok(DtagTransferStatus::Accepted),
            "refuse_dtag_transfer_request" => Ok(DtagTransferStatus::Refused),
            _ => Err(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// Offer represent an auction offer
pub struct Offers(HashMap<Addr, Vec<Coin>>);

impl Offers {
    pub fn new() -> Offers {
        Offers(HashMap::new())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// Auction represent a dtag auction
pub struct Auction {
    pub dtag: String,
    pub starting_price: Uint64,
    pub max_participants: Uint64,
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
    pub status: AuctionStatus,
    pub offers: Offers,
    pub user: Addr,
}

impl Auction {
    pub fn new(
        dtag: String,
        starting_price: Uint64,
        max_participants: Uint64,
        start_time: Option<Timestamp>,
        end_time: Option<Timestamp>,
        auction_status: AuctionStatus,
        user: Addr,
    ) -> Auction {
        Auction {
            dtag,
            starting_price,
            max_participants,
            start_time,
            end_time,
            status: auction_status,
            offers: Offers::new(),
            user,
        }
    }

    pub fn activate(&mut self, time: Timestamp) {
        self.start_time = Some(time);
        // the actual end time is 2 days later the auction start. 2 days = 172800
        self.end_time = Some(time.plus_seconds(172800));
        self.status = AuctionStatus::Active;
    }

    pub fn add_offer(&mut self, user: Addr, offer: Vec<Coin>) {
        self.offers.0.insert(user, offer);
    }

    pub fn remove_offer(&mut self, user: Addr) -> Option<Vec<Coin>> {
        self.offers.0.remove(&user)
    }

    pub fn get_best_offer(&self) -> Option<(&Addr, &Vec<Coin>)> {
        self.offers.0.iter().max_by_key(| offer | offer.1[0].amount)
    }
}

pub const AUCTIONS_STORE: Map<&Addr, Auction> = Map::new("auctions");
pub const ACTIVE_AUCTION: Item<Auction> = Item::new("active_auction");


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// DtagTransferRecord represents a dtag transfer record
pub struct DtagTransferRecord {
    user: String,
}

impl DtagTransferRecord {
    pub fn new(user: String) -> DtagTransferRecord {
        DtagTransferRecord { user }
    }
}

pub const DTAG_TRANSFER_RECORD: Item<DtagTransferRecord> = Item::new("dtag_transfer_record");
