use crate::types::PageRequest;
use cosmwasm_std::{Addr, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RelationshipsQuery {
    Relationships {
        user: Addr,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    },
    Blocks {
        user: Addr,
        subspace_id: Uint64,
        pagination: Option<PageRequest>,
    },
}
