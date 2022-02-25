
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{types::DesmosRoute};

/// DesmosQueryRouter is an override of QueryRequest::Custom to access desmos-specific modules
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub struct DesmosQueryRouter<T> {
    pub route: DesmosRoute,
    pub query_data: T,
}