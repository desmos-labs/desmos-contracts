use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Formatter;
use std::str::FromStr;

use crate::error::ContractError;
use cosmwasm_std::{Addr, Coin, Order, StdResult, Storage, Timestamp, Uint128, Uint64};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
/// This string wrapper represent the contract genesis DTag
pub struct ContractDTag(pub String);

pub const CONTRACT_DTAG_STORE: Item<ContractDTag> = Item::new("contract_dtag");

/// Auction status represent the different status in which an auction can be
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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
            _ => Err(ContractError::UnknownDTagTransferStatus {}),
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
    pub status: AuctionStatus,
    pub user: Addr,
}

impl Auction {
    pub fn new(
        dtag: String,
        starting_price: Uint128,
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
            user,
        }
    }

    pub fn activate(&mut self, time: Timestamp) {
        self.start_time = Some(time);
        // the actual end time is 2 days later the auction start. 2 days = 172800
        self.end_time = Some(time.plus_seconds(172800));
        self.status = AuctionStatus::Active;
    }

    pub fn add_offer(
        &self,
        storage: &mut dyn Storage,
        user: Addr,
        offer: Vec<Coin>,
    ) -> StdResult<()> {
        AUCTION_OFFERS_STORE.save(storage, &user, &offer)
    }

    pub fn remove_offer(&self, storage: &mut dyn Storage, user: Addr) {
        AUCTION_OFFERS_STORE.remove(storage, &user);
    }

    pub fn get_best_offer(&self, storage: &mut dyn Storage) -> StdResult<(Addr, Vec<Coin>)> {
        let best_offer = AUCTION_OFFERS_STORE
            .range(storage, None, None, Order::Ascending)
            .enumerate()
            .max_by_key(|(_, item)| item.as_ref().unwrap().1[0].amount)
            .unwrap()
            .1?;

        let key = String::from_utf8(best_offer.0)?;

        Ok((Addr::unchecked(key), best_offer.1))
    }
}

pub const AUCTIONS_STORE: Map<&Addr, Auction> = Map::new("auctions");
pub const ACTIVE_AUCTION: Item<Auction> = Item::new("active_auction");
pub const AUCTION_OFFERS_STORE: Map<&Addr, Vec<Coin>> = Map::new("auctions_offers");
