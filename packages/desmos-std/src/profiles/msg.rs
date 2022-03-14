use crate::profiles::models_app_links::{Data, TimeoutHeight};
use crate::profiles::models_chain_links::{ChainConfig, ChainLinkAddr, Proof};
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ProfilesMsg {
    SaveProfile {
        dtag: String,
        nickname: String,
        bio: String,
        profile_picture: String,
        cover_picture: String,
        creator: Addr,
    },
    DeleteProfile {
        creator: Addr,
    },
    RequestDtagTransfer {
        receiver: Addr,
        sender: Addr,
    },
    AcceptDtagTransferRequest {
        new_dtag: String,
        sender: Addr,
        receiver: Addr,
    },
    RefuseDtagTransferRequest {
        sender: Addr,
        receiver: Addr,
    },
    CancelDtagTransferRequest {
        receiver: Addr,
        sender: Addr,
    },
    LinkChainAccount {
        chain_address: ChainLinkAddr,
        proof: Proof,
        chain_config: ChainConfig,
        signer: Addr,
    },
    LinkApplication {
        sender: Addr,
        link_data: Data,
        call_data: String,
        source_port: String,
        source_channel: String,
        timeout_height: TimeoutHeight,
        timeout_timestamp: Uint64,
    },
}
