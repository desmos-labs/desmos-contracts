use crate::state::{PendingTip, MAX_CONFIGURABLE_PENDING_TIPS, MAX_CONFIGURABLE_SENT_PENDING_TIPS};
use crate::ContractError;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub max_pending_tips: u16,
    pub max_sent_pending_tips: u16,
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Message to send a tip to another user by application handle.
    SendTip {
        application: String,
        handle: String,
        owner_index: Option<Uint64>,
    },
    /// Message that allows a user to claim their pending tips.
    ClaimTips {},
    /// Message that allows the current admin to update the contract admin.
    UpdateAdmin { new_admin: String },
    /// Message that allows the current admin to update the max pending tips that
    /// can be associated to a centralized application.
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
    /// Query the pending tips of a user.
    #[returns(QueryPendingTipsResponse)]
    UserPendingTips { user: String },
    /// Message to query the unclaimed tips sent from a user.
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
                handle,
                ..
            } => {
                if application.is_empty() {
                    return Err(ContractError::InvalidApplication {});
                }

                if handle.is_empty() {
                    return Err(ContractError::InvalidUserHandle {});
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
                handle,
            } => {
                if application.is_empty() {
                    return Err(ContractError::InvalidApplication {});
                }

                if handle.is_empty() {
                    return Err(ContractError::InvalidUserHandle {});
                }

                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use crate::state::{MAX_CONFIGURABLE_PENDING_TIPS, MAX_CONFIGURABLE_SENT_PENDING_TIPS};
    use crate::ContractError;

    #[test]
    fn instantiate_with_zero_max_pending_tips_error() {
        let error = InstantiateMsg {
            max_pending_tips: 0,
            max_sent_pending_tips: 10,
            admin: None,
        }
        .validate()
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxPendingTipsValue {
                value: 0,
                max: MAX_CONFIGURABLE_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn instantiate_with_bigger_max_pending_tips_error() {
        let error = InstantiateMsg {
            max_pending_tips: MAX_CONFIGURABLE_PENDING_TIPS + 1,
            max_sent_pending_tips: 10,
            admin: None,
        }
        .validate()
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxPendingTipsValue {
                value: MAX_CONFIGURABLE_PENDING_TIPS + 1,
                max: MAX_CONFIGURABLE_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn instantiate_with_zero_max_sent_pending_tips_error() {
        let error = InstantiateMsg {
            max_pending_tips: 5,
            max_sent_pending_tips: 0,
            admin: None,
        }
        .validate()
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxSentPendingTipsValue {
                value: 0,
                max: MAX_CONFIGURABLE_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn instantiate_with_bigger_max_sent_pending_tips_error() {
        let error = InstantiateMsg {
            max_pending_tips: 10,
            max_sent_pending_tips: MAX_CONFIGURABLE_SENT_PENDING_TIPS + 1,
            admin: None,
        }
        .validate()
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxSentPendingTipsValue {
                value: MAX_CONFIGURABLE_SENT_PENDING_TIPS + 1,
                max: MAX_CONFIGURABLE_SENT_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn send_tip_with_empty_application_error() {
        let error = ExecuteMsg::SendTip {
            application: "".to_string(),
            handle: "handle".to_string(),
            owner_index: None,
        }
        .validate()
        .unwrap_err();

        assert_eq!(ContractError::InvalidApplication {}, error);
    }

    #[test]
    fn send_tip_with_empty_handle_error() {
        let error = ExecuteMsg::SendTip {
            application: "application".to_string(),
            handle: "".to_string(),
            owner_index: None,
        }
        .validate()
        .unwrap_err();

        assert_eq!(ContractError::InvalidUserHandle {}, error);
    }

    #[test]
    fn update_max_pending_tips_with_zero_error() {
        let error = ExecuteMsg::UpdateMaxPendingTips { value: 0 }
            .validate()
            .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxPendingTipsValue {
                value: 0,
                max: MAX_CONFIGURABLE_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn update_max_pending_tips_bigger_than_max_value_error() {
        let error = ExecuteMsg::UpdateMaxPendingTips {
            value: MAX_CONFIGURABLE_PENDING_TIPS + 1,
        }
        .validate()
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxPendingTipsValue {
                value: MAX_CONFIGURABLE_PENDING_TIPS + 1,
                max: MAX_CONFIGURABLE_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn update_max_sent_pending_tips_with_zero_error() {
        let error = ExecuteMsg::UpdateMaxSentPendingTips { value: 0 }
            .validate()
            .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxSentPendingTipsValue {
                value: 0,
                max: MAX_CONFIGURABLE_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn update_max_sent_pending_tips_with_not_bigger_value_error() {
        let error = ExecuteMsg::UpdateMaxSentPendingTips {
            value: MAX_CONFIGURABLE_SENT_PENDING_TIPS + 1,
        }
        .validate()
        .unwrap_err();

        assert_eq!(
            ContractError::InvalidMaxSentPendingTipsValue {
                value: MAX_CONFIGURABLE_SENT_PENDING_TIPS + 1,
                max: MAX_CONFIGURABLE_SENT_PENDING_TIPS
            },
            error
        );
    }

    #[test]
    fn remove_pending_tip_with_empty_application_error() {
        let error = ExecuteMsg::RemovePendingTip {
            application: "".to_string(),
            handle: "handle".to_string(),
        }
        .validate()
        .unwrap_err();

        assert_eq!(ContractError::InvalidApplication {}, error);
    }

    #[test]
    fn remove_pending_tip_with_empty_handle_error() {
        let error = ExecuteMsg::RemovePendingTip {
            application: "application".to_string(),
            handle: "".to_string(),
        }
        .validate()
        .unwrap_err();

        assert_eq!(ContractError::InvalidUserHandle {}, error);
    }
}
