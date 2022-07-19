use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Timestamp};
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub minter: Addr,
    pub mint_enabled: bool,
    pub per_address_limit: u32,
    pub cw721_code_id: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct EventInfo {
    pub creator: Addr,
    pub start_time: Timestamp,
    pub end_time: Timestamp,
    pub base_poap_uri: String,
    pub event_uri: String,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const EVENT_INFO: Item<EventInfo> = Item::new("event_info");
pub const CW721_ADDRESS: Item<Addr> = Item::new("cw721_address");
pub const NEXT_POAP_ID: Item<u64> = Item::new("nex_poap_id");
pub const MINTER_ADDRESS: Map<Addr, u32> = Map::new("minter_address");

impl EventInfo {
    /// Checks if the event has already started.
    /// * `timestamp` - Reference time used to check if the event has started.
    pub fn is_started(&self, timestamp: &Timestamp) -> bool {
        &self.start_time <= timestamp
    }

    /// Checks if the event is ended.
    /// * `timestamp` - Reference time used to check if the event is ended.
    pub fn is_ended(&self, timestamp: &Timestamp) -> bool {
        timestamp >= &self.end_time
    }

    /// Checks if the event is in progress.
    /// * `timestamp` - Reference time used to check if the event is in progress.
    pub fn in_progress(&self, timestamp: &Timestamp) -> bool {
        self.is_started(timestamp) && !self.is_ended(timestamp)
    }
}

#[cfg(test)]
mod tests {
    use crate::state::EventInfo;
    use cosmwasm_std::{Addr, Timestamp};

    fn mock_event_info(start: u64, end: u64) -> EventInfo {
        EventInfo {
            creator: Addr::unchecked(""),
            start_time: Timestamp::from_seconds(start),
            end_time: Timestamp::from_seconds(end),
            base_poap_uri: "".to_string(),
            event_uri: "".to_string(),
        }
    }

    #[test]
    fn event_is_started() {
        let current_time = Timestamp::from_seconds(300);
        let event_info = mock_event_info(200, 400);

        assert!(event_info.is_started(&current_time));

        // Test edge case start time = current time
        assert!(event_info.is_started(&event_info.start_time));
    }

    #[test]
    fn event_not_started() {
        let current_time = Timestamp::from_seconds(300);
        let event_info = mock_event_info(350, 400);

        assert!(!event_info.is_started(&current_time));
    }

    #[test]
    fn event_is_ended() {
        let current_time = Timestamp::from_seconds(300);
        let event_info = mock_event_info(200, 250);

        assert!(event_info.is_ended(&current_time));

        // Test edge end time = current time
        assert!(event_info.is_ended(&event_info.end_time));
    }

    #[test]
    fn event_not_ended() {
        let current_time = Timestamp::from_seconds(300);
        let event_info = mock_event_info(200, 400);

        assert!(!event_info.is_ended(&current_time));
    }

    #[test]
    fn event_in_progress() {
        let current_time = Timestamp::from_seconds(150);
        let event_info = mock_event_info(100, 200);

        assert!(event_info.in_progress(&current_time));
        // Test edge case current time = start time
        assert!(event_info.in_progress(&event_info.start_time));
    }

    #[test]
    fn event_not_in_progress() {
        let current_time = Timestamp::from_seconds(500);
        let event_info = mock_event_info(100, 200);

        assert!(!event_info.in_progress(&current_time));
        // Test edge case current time = end time
        assert!(!event_info.in_progress(&event_info.end_time));
    }
}
