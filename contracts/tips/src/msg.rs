use crate::contract::MAX_TIPS_HISTORY_SIZE;
use crate::error::ContractError;
use crate::state::{StateServiceFee, StateTip};
use cosmwasm_std::{Addr, Coin, Decimal, Uint64};
use cosmwasm_schema::{cw_serde, QueryResponses};

/// Fees required to execute [`ExecuteMsg::SendTip`].
#[cw_serde]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum ServiceFee {
    /// Represents a fixed fee that the sender needs to pay in order to send the tip.
    Fixed {
        /// Coins that the sender needs to pay.
        amount: Vec<Coin>,
    },
    /// Represents a percentage that the sender needs to pay in order to send the tip.
    Percentage {
        /// Percentage value.
        value: Decimal,
    },
}

impl ServiceFee {
    pub fn validate(&self) -> Result<(), ContractError> {
        match self {
            ServiceFee::Fixed { amount } => {
                if amount.is_empty() {
                    return Err(ContractError::EmptyFixedFee {});
                }

                let zero_coin = amount.iter().find(|coin| coin.amount.is_zero());
                if let Some(coin) = zero_coin {
                    return Err(ContractError::ZeroFeeCoin {
                        denom: coin.denom.to_owned(),
                    });
                }
            }
            ServiceFee::Percentage { value } => {
                let one_hundred = Decimal::from_atomics(100u32, 0).unwrap();
                if value.ge(&one_hundred) || value.is_zero() {
                    return Err(ContractError::InvalidPercentageFee {});
                }
            }
        }

        Ok(())
    }
}

impl From<StateServiceFee> for ServiceFee {
    fn from(state_service_fees: StateServiceFee) -> Self {
        match state_service_fees {
            StateServiceFee::Fixed { amount } => ServiceFee::Fixed { amount },
            StateServiceFee::Percentage { value } => ServiceFee::Percentage { value },
        }
    }
}

#[cw_serde]
pub struct InstantiateMsg {
    /// Address of who will have the right to manage the contract.
    pub admin: String,
    /// Application which is deploying the contract.
    pub subspace_id: Uint64,
    /// Fee that the users need to pay to use the contract.
    /// If `None` no fees will be collected from the tipped amount.
    pub service_fee: Option<ServiceFee>,
    /// The number of records saved of a user tips history.
    pub tips_history_size: u32,
}

impl InstantiateMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.subspace_id.is_zero() {
            return Err(ContractError::InvalidSubspaceId {});
        }

        if let Some(service_fee) = &self.service_fee {
            service_fee.validate()?;
        }

        if self.tips_history_size > MAX_TIPS_HISTORY_SIZE {
            return Err(ContractError::InvalidTipsHistorySize {
                value: self.tips_history_size,
                max: MAX_TIPS_HISTORY_SIZE,
            });
        }

        Ok(())
    }
}

/// Enum that represents a tip target.
#[cw_serde]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum Target {
    /// Tip related to an user's post to show their support towards a specific content.
    ContentTarget {
        /// Post id.
        post_id: Uint64,
    },
    /// Tip for an user.
    UserTarget {
        /// Address of the user that will receive the tip.
        receiver: String,
    },
}

#[cw_serde]
pub enum ExecuteMsg {
    /// Sends a tip to an user or to the author of post.  
    SendTip {
        /// Tip target.
        target: Target,
        /// Amount from which fees will be calculated.
        amount: Vec<Coin>,
    },
    /// Updates the fee required to execute [`ExecuteMsg::SendTip`].
    UpdateServiceFee {
        /// New service fee required to execute [`ExecuteMsg::SendTip`].
        /// If `None` no fees will be collected from the tipped amount.
        new_fee: Option<ServiceFee>,
    },
    /// Updates the contract admin.
    UpdateAdmin {
        /// Address of the new contract admin.
        new_admin: String,
    },
    /// Updates the number of record saved in the tips history.
    UpdateSavedTipsHistorySize {
        /// New tips history size.
        new_size: u32,
    },
    /// Claims the fees paid to execute the contract.
    ClaimFees {
        /// Address to which fees will be sent.
        receiver: String,
    },
}

impl ExecuteMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        match self {
            ExecuteMsg::SendTip { target, .. } => match target {
                Target::ContentTarget { post_id } => {
                    if post_id.is_zero() {
                        Err(ContractError::InvalidPostId {})
                    } else {
                        Ok(())
                    }
                }
                Target::UserTarget { .. } => Ok(()),
            },
            ExecuteMsg::UpdateServiceFee { new_fee } => {
                if let Some(service_fee) = new_fee {
                    service_fee.validate()
                } else {
                    Ok(())
                }
            }
            ExecuteMsg::UpdateSavedTipsHistorySize { new_size } => {
                if *new_size > MAX_TIPS_HISTORY_SIZE {
                    Err(ContractError::InvalidTipsHistorySize {
                        value: *new_size,
                        max: MAX_TIPS_HISTORY_SIZE,
                    })
                } else {
                    Ok(())
                }
            }
            _ => Ok(()),
        }
    }
}

#[cw_serde]
#[derive(QueryResponses)]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum QueryMsg {
    /// Returns a [`ConfigResponse`] containing the configuration info of the contract.
    #[returns(QueryConfigResponse)]
    Config {},
    /// Returns a [`TipsResponse`] containing all the received tips of the user.
    #[returns(TipsResponse)]
    UserReceivedTips { user: String },
    /// Returns a [`TipsResponse`] containing all the sent tips from the user.
    #[returns(TipsResponse)]
    UserSentTips { user: String },
    ///Returns a [`TipsResponse`] containing all the tips associated with a given post.
    #[returns(TipsResponse)]
    PostReceivedTips { post_id: Uint64 },
}

#[cw_serde]
pub struct QueryConfigResponse {
    /// Address of the contract administrator.
    pub admin: Addr,
    /// Application that distributed the contract.
    pub subspace_id: Uint64,
    /// Fee required to execute [`ExecuteMsg::SendTip`].
    pub service_fee: Option<ServiceFee>,
    /// The number of records saved of a user tips history.
    pub tips_history_size: u32,
}

#[cw_serde]
pub struct TipsResponse {
    pub tips: Vec<Tip>,
}

#[cw_serde]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct Tip {
    pub sender: Addr,
    pub receiver: Addr,
    pub amount: Vec<Coin>,
    pub post_id: Option<Uint64>,
    pub block_height: Uint64,
}

impl Tip {
    pub fn from_state_tip(state_tip: StateTip, block_height: u64) -> Self {
        Tip {
            sender: state_tip.sender,
            receiver: state_tip.receiver,
            amount: state_tip.amount,
            post_id: if state_tip.post_id > 0 {
                Some(state_tip.post_id.into())
            } else {
                None
            },
            block_height: block_height.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ContractError;
    use crate::msg::{ServiceFee, Tip};
    use crate::state::StateTip;
    use cosmwasm_std::{Addr, Coin, Decimal, Uint64};

    #[test]
    fn fixed_service_fee_zero_fee_coin_error() {
        let service_fee = ServiceFee::Fixed {
            amount: vec![Coin::new(0, "udsm")],
        };

        assert_eq!(
            ContractError::ZeroFeeCoin {
                denom: "udsm".to_string(),
            },
            service_fee.validate().unwrap_err()
        );
    }

    #[test]
    fn fixed_service_fee_empty_amount_error() {
        let service_fee = ServiceFee::Fixed { amount: vec![] };

        assert_eq!(
            ContractError::EmptyFixedFee {},
            service_fee.validate().unwrap_err()
        );
    }

    #[test]
    fn fixed_service_fee_validate_properly() {
        let service_fee = ServiceFee::Fixed {
            amount: vec![Coin::new(42, "udsm")],
        };

        service_fee.validate().unwrap();
    }

    #[test]
    fn percentage_service_fee_zero_percentage_error() {
        let service_fee = ServiceFee::Percentage {
            value: Decimal::from_atomics(0u32, 0).unwrap(),
        };

        assert_eq!(
            ContractError::InvalidPercentageFee {},
            service_fee.validate().unwrap_err()
        );
    }

    #[test]
    fn percentage_service_fee_100_percentage_error() {
        let service_fee = ServiceFee::Percentage {
            value: Decimal::from_atomics(100u32, 0).unwrap(),
        };

        assert_eq!(
            ContractError::InvalidPercentageFee {},
            service_fee.validate().unwrap_err()
        );
    }

    #[test]
    fn percentage_service_fee_validate_properly() {
        let service_fee = ServiceFee::Percentage {
            value: Decimal::from_atomics(25u32, 1).unwrap(),
        };

        service_fee.validate().unwrap();
    }

    #[test]
    fn tip_from_state_tip_properly() {
        let sender = Addr::unchecked("sender");
        let receiver = Addr::unchecked("receiver");

        assert_eq!(
            Tip {
                sender: sender.clone(),
                receiver: receiver.clone(),
                amount: vec![],
                post_id: Some(Uint64::new(32)),
                block_height: 1234u64.into()
            },
            Tip::from_state_tip(
                StateTip {
                    receiver,
                    sender,
                    amount: vec![],
                    ref_counter: 2,
                    post_id: 32
                },
                1234
            )
        )
    }
}
