use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use cosmwasm_std::{Addr, Coin, Order, StdResult, Storage, Timestamp, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

use crate::error::ContractError;

pub const CONTRACT_DTAG_STORE: Item<ContractDTag> = Item::new("contract_dtag");
pub const INACTIVE_AUCTIONS_STORE: Map<&Addr, Auction> = Map::new("inactive_auctions");
pub const ACTIVE_AUCTION: Item<Auction> = Item::new("active_auction");
pub const AUCTION_BIDS_STORE: Map<&Addr, Vec<Coin>> = Map::new("auction_bids");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// This string wrapper represent the contract genesis DTag
pub struct ContractDTag(pub String);

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// The status of an on-chain dTag transfer request
pub enum DtagTransferStatus {
    Accepted,
    Refused,
}

impl FromStr for DtagTransferStatus {
    type Err = ContractError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "accept_dtag_transfer_request" => Ok(DtagTransferStatus::Accepted),
            "refuse_dtag_transfer_request" => Ok(DtagTransferStatus::Refused),
            _ => Err(ContractError::UnknownDTagTransferStatus { status: String::from(s) }),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// Auction represent a dtag auction
pub struct Auction {
    pub dtag: String,
    pub starting_price: Uint128,
    pub max_participants: Uint64,
    pub start_time: Option<Timestamp>,
    pub end_time: Option<Timestamp>,
    pub claim_time: Option<Timestamp>,
    pub creator: Addr,
}

impl Auction {
    pub fn new(
        dtag: String,
        starting_price: Uint128,
        max_participants: Uint64,
        start_time: Option<Timestamp>,
        end_time: Option<Timestamp>,
        claim_time: Option<Timestamp>,
        creator: Addr,
    ) -> Auction {
        Auction {
            dtag,
            starting_price,
            max_participants,
            start_time,
            end_time,
            claim_time,
            creator,
        }
    }

    /// activate make the auction active by calculating its end time
    pub fn activate(&mut self, time: Timestamp) {
        self.start_time = Some(time);
        // the actual end time is 2 days later the auction start. 2 days = 172800
        self.end_time = Some(time.plus_seconds(172800));
    }

    /// start_claim_time calculate a claim time period for the auction's dTag
    pub fn start_claim_time(&mut self) {
        self.claim_time = Some(self.end_time.unwrap().plus_nanos(86400)) // 1 day to claim dTag
    }

    /// add_bid add the bid associated with the given user to the bids store
    pub fn add_bid(&self, storage: &mut dyn Storage, user: &Addr, bid: Vec<Coin>) {
        let _ = AUCTION_BIDS_STORE.save(storage, user, &bid);
    }

    /// remove_bid remove the bid associated with the given user from the bids store
    pub fn remove_bid(&self, storage: &mut dyn Storage, user: &Addr) {
        AUCTION_BIDS_STORE.remove(storage, user);
    }

    /// count_bids count the number of bids in the bids store
    pub fn count_bids(&self, storage: &mut dyn Storage) -> u64 {
        AUCTION_BIDS_STORE
            .range(storage, None, None, Order::Ascending)
            .count() as u64
    }

    /// get_last_bid returns the last (and best) bid made to the auction
    pub fn get_last_bid(&self, storage: &mut dyn Storage) -> Result<(Addr, Vec<Coin>), ContractError> {
        let result = AUCTION_BIDS_STORE
            .range(storage, None, None, Order::Ascending)
            .last()
            .unwrap()?;
        Ok(result)
    }

    /// get_best_bid_amount returns the best bid amount (in our case it matches the last bid amount)
    pub fn get_best_bid_amount(&self, storage: &mut dyn Storage) -> Result<Uint128, ContractError> {
        Ok(self.get_last_bid(storage)?.1[0].amount)
    }

    /// get_all_bids returns all the bids made to the active auction
    pub fn get_all_bids(&self, storage: &mut dyn Storage) -> Result<Vec<(Addr, Vec<Coin>)>, ContractError> {
        let result: StdResult<Vec<_>> = AUCTION_BIDS_STORE
            .range(storage, None, None, Order::Ascending)
            .collect();
        Ok(result.unwrap())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
/// AuctionResponse represents the response for queries
pub struct AuctionResponse {
    pub auction: Auction,
    pub bids: Vec<(Addr, Vec<Coin>)>
}
