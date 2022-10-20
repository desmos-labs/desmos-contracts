use crate::state::PendingTips;
use crate::ContractError;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Message to send a tip to another user.
    SendTip {
        application: String,
        handler: String,
    },
    /// Message that allow a user to claim their tips.
    ClaimTips {},
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Query the pending tips of an user.
    UserPendingTips { user: String },
}

/// Response to [QueryMsg::UserPendingTips].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryPendingTipsResponse {
    pub tips: PendingTips,
}

impl ExecuteMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        match self {
            ExecuteMsg::SendTip {
                application,
                handler,
            } => {
                if application.is_empty() {
                    return Err(ContractError::InvalidApplication {});
                }

                if handler.is_empty() {
                    return Err(ContractError::InvalidUserHandler {});
                }

                Ok(())
            }
            ExecuteMsg::ClaimTips {} => Ok(()),
        }
    }
}
