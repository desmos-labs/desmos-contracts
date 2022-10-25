use crate::state::{PendingTips, MAX_CONFIGURABLE_PENDING_TIPS};
use crate::ContractError;
use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub max_pending_tips: u32,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Message to send a tip to another user.
    SendTip { application: String, handle: String },
    /// Message that allow a user to claim their tips.
    ClaimTips {},
    /// Message to update the max pending tips that an user can have.
    UpdateMaxPendingTips { value: u32 },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Query the pending tips of an user.
    #[returns(QueryPendingTipsResponse)]
    UserPendingTips { user: String },
}

/// Response to [QueryMsg::UserPendingTips].
#[cw_serde]
pub struct QueryPendingTipsResponse {
    pub tips: PendingTips,
}

impl InstantiateMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.max_pending_tips == 0 || self.max_pending_tips > MAX_CONFIGURABLE_PENDING_TIPS {
            return Err(ContractError::InvalidMaxPendingTipsValue {
                value: self.max_pending_tips,
                max: MAX_CONFIGURABLE_PENDING_TIPS,
            });
        }

        Ok(())
    }
}

impl ExecuteMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        match self {
            ExecuteMsg::SendTip {
                application,
                handle: handler,
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
            ExecuteMsg::UpdateMaxPendingTips { value } => {
                if *value == 0 || *value > MAX_CONFIGURABLE_PENDING_TIPS {
                    Err(ContractError::InvalidMaxPendingTipsValue {
                        value: *value,
                        max: MAX_CONFIGURABLE_PENDING_TIPS,
                    })
                } else {
                    Ok(())
                }
            }
        }
    }
}
