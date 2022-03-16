use crate::types::PageRequest;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipsQuery {
    Relationships {
        subspace_id: Uint64,
        user: Option<Addr>,
        counterparty: Option<Addr>,
        pagination: Option<PageRequest>,
    },
    Blocks {
        subspace_id: Uint64,
        blocker: Option<Addr>,
        blocked: Option<Addr>,
        pagination: Option<PageRequest>,
    },
}
