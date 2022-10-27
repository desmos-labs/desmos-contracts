use crate::state::{PendingTip, MAX_CONFIGURABLE_PENDING_TIPS, MAX_CONFIGURABLE_SENT_PENDING_TIPS};
use crate::ContractError;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub max_pending_tips: u16,
    pub max_sent_pending_tips: u16,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Message to send a tip to another user.
    SendTip { application: String, handle: String },
    /// Message that allow a user to claim their tips.
    ClaimTips {},
    /// Message that allows the current admin to update the contract admin.
    UpdateAdmin { new_admin: String },
    /// Message that allows the current admin to update the max pending tips that
    /// can be associated to a centralize application.
    UpdateMaxPendingTips { value: u16 },
    /// Message that allows the current admin to update the max pending tips that
    /// can be sent from a user.
    UpdateMaxSentPendingTips { value: u16 },
    /// Message to remove an unclaimed pending tip.
    RemovePendingTip { application: String, handle: String },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Query the pending tips of an user.
    #[returns(QueryPendingTipsResponse)]
    UserPendingTips { user: String },
    /// Message to query the unclaimed tips sent from an user.
    #[returns(QueryUnclaimedSentTipsResponse)]
    UnclaimedSentTips { user: String },
    /// Message to query the contract configurations.
    #[returns(QueryConfigResponse)]
    Config {},
}

/// Response to [QueryMsg::UserPendingTips].
#[cw_serde]
pub struct QueryPendingTipsResponse {
    pub tips: Vec<PendingTip>,
}

/// Response to [QueryMsg::UnclaimedTips].
#[cw_serde]
pub struct QueryUnclaimedSentTipsResponse {
    pub tips: Vec<PendingTip>,
}

/// Response to [QueryMsg::UnclaimedTips].
#[cw_serde]
pub struct QueryConfigResponse {
    pub admin: Addr,
    pub max_pending_tips: u16,
    pub max_sent_pending_tips: u16,
}

impl InstantiateMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.max_pending_tips == 0 || self.max_pending_tips > MAX_CONFIGURABLE_PENDING_TIPS {
            return Err(ContractError::InvalidMaxPendingTipsValue {
                value: self.max_pending_tips,
                max: MAX_CONFIGURABLE_PENDING_TIPS,
            });
        }

        if self.max_sent_pending_tips == 0
            || self.max_sent_pending_tips > MAX_CONFIGURABLE_SENT_PENDING_TIPS
        {
            return Err(ContractError::InvalidMaxSentPendingTipsValue {
                value: self.max_sent_pending_tips,
                max: MAX_CONFIGURABLE_SENT_PENDING_TIPS,
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
            ExecuteMsg::UpdateAdmin { .. } => Ok(()),
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
            ExecuteMsg::UpdateMaxSentPendingTips { value } => {
                if *value == 0 || *value > MAX_CONFIGURABLE_SENT_PENDING_TIPS {
                    Err(ContractError::InvalidMaxSentPendingTipsValue {
                        value: *value,
                        max: MAX_CONFIGURABLE_SENT_PENDING_TIPS,
                    })
                } else {
                    Ok(())
                }
            }
            ExecuteMsg::RemovePendingTip {
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
        }
    }
}
