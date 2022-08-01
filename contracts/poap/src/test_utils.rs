use crate::msg::{EventInfo, InstantiateMsg};
use cosmwasm_std::Timestamp;
use cw721_base::InstantiateMsg as Cw721InstantiateMsg;

pub const INITIAL_BLOCK_TIME_SECONDS: u64 = 3600;
pub const EVENT_START_SECONDS: u64 = INITIAL_BLOCK_TIME_SECONDS + 3600;
pub const EVENT_END_SECONDS: u64 = EVENT_START_SECONDS + 3600;
pub const CREATOR: &str = "creator";
pub const ADMIN: &str = "admin";
pub const MINTER: &str = "minter";
pub const USER: &str = "user";
pub const EVENT_URI: &str = "ipfs://event-uri";

pub fn get_valid_init_msg(cw721_code_id: u64) -> InstantiateMsg {
    let start_time = Timestamp::from_seconds(EVENT_START_SECONDS);
    let end_time = Timestamp::from_seconds(EVENT_END_SECONDS);

    InstantiateMsg {
        admin: ADMIN.to_string(),
        minter: MINTER.to_string(),
        cw721_code_id: cw721_code_id.into(),
        cw721_initiate_msg: Cw721InstantiateMsg {
            name: "test-poap".to_string(),
            symbol: "poap".to_string(),
            minter: "".to_string(),
        },
        event_info: EventInfo {
            creator: CREATOR.to_string(),
            start_time,
            end_time,
            per_address_limit: 2,
            base_poap_uri: "ipfs://popap-uri".to_string(),
            event_uri: EVENT_URI.to_string(),
        },
    }
}
