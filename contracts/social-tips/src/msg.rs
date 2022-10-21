use crate::state::PendingTips;
use crate::ContractError;
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    /// Message to send a tip to another user.
    SendTip {
        application: String,
        handler: String,
    },
    /// Message that allow a user to claim their tips.
    ClaimTips {},
}

#[cw_serde]
pub enum QueryMsg {
    /// Query the pending tips of an user.
    UserPendingTips { user: String },
}

/// Response to [QueryMsg::UserPendingTips].
#[cw_serde]
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
