use crate::error::ContractError;
use crate::state::StateServiceFee;
use cosmwasm_std::{Addr, Coin, Uint128, Uint64};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// Fees required to execute [`ExecuteMsg::SendTip`].
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ServiceFee {
    /// Represents a fixed fee that is deducted from the tip.
    Fixed {
        /// Coins that are deducted from the tip.
        amount: Vec<Coin>,
    },
    /// Represents a percentage that is deducted from the tip.
    Percentage {
        /// Percentage value.
        value: Uint128,
        /// Percentage decimals.
        /// Example if this value is 3 means that the value should have 3 decimals numbers so
        /// if value is
        /// - 1 means 0,001%
        /// - 10 means 0,01%
        /// - 100 means 0,1%
        /// - 1000 means 1%
        /// - 10000 means 10%
        /// - 100000 means 100%
        decimals: u32,
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
            ServiceFee::Percentage { value, decimals } => {
                let percent_value = value.u128() / 10_u128.pow(*decimals);
                if percent_value >= 100 || percent_value == 0 {
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
            StateServiceFee::Percentage { value, decimals } => ServiceFee::Percentage {
                value: value.into(),
                decimals,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    /// Address of who will have the right to manage the contract.
    pub admin: String,
    /// Application which is deploying the contract.
    pub subspace_id: Uint64,
    /// Fee that the users need to pay to use the contract.
    /// If `None` no fees will be collected from the tipped amount.
    pub service_fee: Option<ServiceFee>,
    /// The number of records saved of a user tips history.
    pub saved_tips_threshold: u32,
}

impl InstantiateMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.subspace_id == Uint64::zero() {
            return Err(ContractError::InvalidSubspaceId {});
        }

        if let Some(service_fee) = &self.service_fee {
            service_fee.validate()?;
        }

        Ok(())
    }
}

/// Enum that represents a tip target.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Sends a tip to an user or to the author of post.  
    SendTip {
        /// Tip target.
        target: Target,
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
    /// Updates the number of tip records saved in the contract state.
    UpdateSavedTipsRecordThreshold {
        /// New tip records threshold.
        new_threshold: u32,
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
            ExecuteMsg::SendTip { target } => match target {
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
            _ => Ok(()),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Return a [`ConfigResponse`] containing the configuration info of the contract.
    Config {},
    /// Return a [`TipsResponse`] containing all the received tips of the user.
    UserReceivedTips { user: String },
    /// Return a [`TipsResponse`] containing all the sent tips from the user.
    UserSentTips { user: String },
    ///Return a [`TipsResponse`] containing all the tips associated with a given post.
    PostReceivedTips { post_id: Uint64 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct QueryConfigResponse {
    /// Address of the contract administrator.
    pub admin: Addr,
    /// Application that distributed the contract.
    pub subspace_id: Uint64,
    /// Fee required to execute [`ExecuteMsg::SendTip`].
    pub service_fee: Option<ServiceFee>,
    /// The number of records saved of a user tips history.
    pub saved_tips_record_threshold: u32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TipsResponse {
    pub tips: Vec<Tip>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Tip {
    pub sender: Addr,
    pub receiver: Addr,
    pub amount: Vec<Coin>,
}

#[cfg(test)]
mod tests {
    use crate::error::ContractError;
    use crate::msg::ServiceFee;
    use cosmwasm_std::{Coin, Uint128};

    #[test]
    fn fixed_service_fee_zero_fee_coin() {
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
    fn fixed_service_fee_empty_amount() {
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
    fn percentage_service_fee_zero_percentage() {
        let service_fee = ServiceFee::Percentage {
            value: Uint128::new(0),
            decimals: 6,
        };

        assert_eq!(
            ContractError::InvalidPercentageFee {},
            service_fee.validate().unwrap_err()
        );
    }

    #[test]
    fn percentage_service_fee_100_percentage() {
        let service_fee = ServiceFee::Percentage {
            value: Uint128::new(100),
            decimals: 0,
        };

        assert_eq!(
            ContractError::InvalidPercentageFee {},
            service_fee.validate().unwrap_err()
        );
    }

    #[test]
    fn percentage_service_fee_validate_properly() {
        let service_fee = ServiceFee::Percentage {
            value: Uint128::new(100),
            decimals: 2,
        };

        service_fee.validate().unwrap();
    }
}
