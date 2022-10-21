use crate::error::ContractError;
use crate::msg::ServiceFee;
use crate::utils::{serialize_coins, sum_coins_sorted};
use cosmwasm_std::{Addr, Coin, Decimal};
use cw_storage_plus::{Item, Map};
use cosmwasm_schema::cw_serde;
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ops::{Div, Mul};

#[cw_serde]
#[allow(clippy::derive_partial_eq_without_eq)]
pub enum StateServiceFee {
    Fixed { amount: Vec<Coin> },
    Percentage { value: Decimal },
}

#[cw_serde]
pub struct Config {
    pub admin: Addr,
    pub subspace_id: u64,
    pub service_fee: Option<StateServiceFee>,
    pub tips_history_size: u32,
}

#[cw_serde]
#[allow(clippy::derive_partial_eq_without_eq)]
pub struct StateTip {
    /// Who sent this tip.
    pub sender: Addr,
    /// Who received this tip.
    pub receiver: Addr,
    /// If some means that this tip is referencing a post.
    pub post_id: u64,
    /// Tip amount.
    pub amount: Vec<Coin>,
    /// Counts how many references exist toward this tip.
    /// With our current implementation this value can be in [0, 3]
    /// since a tip can be inside an user sent tips history,
    /// an user received tips history and a post received tips history.
    pub ref_counter: u8,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const BLOCK_INDEX: Item<(u64, u32)> = Item::new("block_index");
pub const TIPS: Map<(u64, u32), StateTip> = Map::new("tips");
pub type TipHistory = VecDeque<(u64, u32)>;
pub const SENT_TIPS_HISTORY: Map<Addr, TipHistory> = Map::new("sent_tips_history");
pub const RECEIVED_TIPS_HISTORY: Map<Addr, TipHistory> = Map::new("received_tips_history");
pub const POST_TIPS_HISTORY: Map<u64, TipHistory> = Map::new("post_tips_history");

impl StateServiceFee {
    /// Computes the fees that the contract will holds and the coins that
    /// can be sent to the user.
    /// * `funds` - Coins sent from the user to the contract.
    /// * `tip_amount` - Coins from which to calculate the fees.
    pub fn check_fees(&self, funds: &[Coin], tip_amount: &[Coin]) -> Result<(), ContractError> {
        let funds = sum_coins_sorted(funds.to_vec())?;
        // Compute the fees
        let mut fee = match self {
            StateServiceFee::Fixed { amount } => amount.clone(),
            StateServiceFee::Percentage { value } => {
                let percentage_value = value.div(Decimal::from_atomics(100u32, 0).unwrap());
                tip_amount
                    .iter()
                    .map(|coin| Coin {
                        amount: coin.amount.mul(percentage_value),
                        denom: coin.denom.clone(),
                    })
                    .collect()
            }
        };

        // Put the tip amount inside the fees
        fee.extend(tip_amount.to_vec());
        let fee_plus_tips = sum_coins_sorted(fee)?;
        // Check fees + tips < funds
        for fee_plus_tip in fee_plus_tips.iter() {
            // Search the fee coin inside the funds sent to the contract
            let fund_coin_amount = funds
                .binary_search_by(|coin| coin.denom.cmp(&fee_plus_tip.denom))
                .map(|index| funds[index].amount)
                .map_err(|_| ContractError::InsufficientFunds {
                    requested: serialize_coins(&fee_plus_tips),
                    provided: serialize_coins(&funds),
                })?;

            // Ensure tip amount + fee <= provided funds
            if fee_plus_tip.amount > fund_coin_amount {
                return Err(ContractError::InsufficientFunds {
                    requested: serialize_coins(&fee_plus_tips),
                    provided: serialize_coins(&funds),
                });
            }
        }

        Ok(())
    }
}

impl TryFrom<ServiceFee> for StateServiceFee {
    type Error = ContractError;

    fn try_from(service_fees: ServiceFee) -> Result<Self, ContractError> {
        service_fees.validate()?;

        match service_fees {
            ServiceFee::Fixed { amount } => Ok(StateServiceFee::Fixed {
                amount: sum_coins_sorted(amount)?,
            }),
            ServiceFee::Percentage { value } => Ok(StateServiceFee::Percentage { value }),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ContractError;
    use crate::msg::ServiceFee;
    use crate::state::StateServiceFee;
    use cosmwasm_std::{Coin, Decimal};
    use std::convert::TryFrom;

    #[test]
    fn fixed_state_service_fee_from_service_fee_properly() {
        let fees = vec![Coin::new(1000, "udsm")];
        let service_fee = ServiceFee::Fixed {
            amount: fees.clone(),
        };

        let state_service_fee = StateServiceFee::try_from(service_fee).unwrap();
        match state_service_fee {
            StateServiceFee::Fixed { amount } => {
                assert_eq!(fees, amount)
            }
            StateServiceFee::Percentage { .. } => {
                panic!("ServiceFee::Fixed should be converted to StateServiceFee::Fixed")
            }
        }
    }

    #[test]
    fn fixed_fees_insufficient_funds_error() {
        let fixed_fee_amount = 2000;
        let tip_amount = 1000;
        let fund_amount = 2500;

        let service_fees = StateServiceFee::Fixed {
            amount: vec![Coin::new(fixed_fee_amount, "udsm")],
        };

        let funds = vec![Coin::new(fund_amount, "udsm")];
        let tips = vec![Coin::new(tip_amount, "udsm")];
        let computed_fees = service_fees.check_fees(&funds, &tips).unwrap_err();

        assert_eq!(
            ContractError::InsufficientFunds {
                requested: "3000udsm".to_string(),
                provided: "2500udsm".to_string(),
            },
            computed_fees
        );
    }

    #[test]
    fn fixed_fees_fee_coin_not_provided_error() {
        let fixed_fee_amount = 20000;
        let tip_amount = 100000;

        let service_fees = StateServiceFee::Fixed {
            amount: vec![
                Coin::new(fixed_fee_amount, "udsm"),
                Coin::new(fixed_fee_amount, "uatom"),
            ],
        };

        let computed_fees = service_fees
            .check_fees(
                &vec![Coin::new(fixed_fee_amount + tip_amount, "udsm")],
                &vec![Coin::new(tip_amount, "udsm")],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::InsufficientFunds {
                requested: "20000uatom,120000udsm".to_string(),
                provided: "120000udsm".to_string()
            },
            computed_fees
        );
    }

    #[test]
    fn fixed_fees_fee_fund_coin_not_provided_error() {
        let fixed_fee_amount = 20000;
        let tip_amount = 100000;

        let service_fees = StateServiceFee::Fixed {
            amount: vec![Coin::new(fixed_fee_amount, "udsm")],
        };

        let computed_fees = service_fees
            .check_fees(
                &vec![Coin::new(fixed_fee_amount + tip_amount, "udsm")],
                &vec![
                    Coin::new(tip_amount, "udsm"),
                    Coin::new(tip_amount, "uatom"),
                ],
            )
            .unwrap_err();

        assert_eq!(
            ContractError::InsufficientFunds {
                requested: "100000uatom,120000udsm".to_string(),
                provided: "120000udsm".to_string(),
            },
            computed_fees
        );
    }

    #[test]
    fn fixed_fees_check_properly() {
        let fixed_fee_amount = 20000;
        let tip_amount = 100000;

        let service_fees = StateServiceFee::Fixed {
            amount: vec![
                Coin::new(fixed_fee_amount, "udsm"),
                Coin::new(fixed_fee_amount, "uatom"),
            ],
        };

        service_fees
            .check_fees(
                &vec![
                    Coin::new(fixed_fee_amount + tip_amount, "udsm"),
                    Coin::new(fixed_fee_amount, "uatom"),
                ],
                &vec![Coin::new(tip_amount, "udsm")],
            )
            .unwrap();
    }

    #[test]
    fn percentage_state_service_fee_from_service_fee_invalid_percentage_error() {
        // Service fees at 100%
        let service_fee = ServiceFee::Percentage {
            value: Decimal::from_atomics(100u32, 0).unwrap(),
        };

        let error = StateServiceFee::try_from(service_fee).unwrap_err();
        assert_eq!(ContractError::InvalidPercentageFee {}, error);
    }

    #[test]
    fn percentage_state_service_fee_from_service_fee_properly() {
        let service_fee = ServiceFee::Percentage {
            value: Decimal::one(),
        };

        let state_service_fee = StateServiceFee::try_from(service_fee).unwrap();
        match state_service_fee {
            StateServiceFee::Fixed { .. } => {
                panic!("ServiceFee::Percentage should be converted to StateServiceFee::Percentage")
            }
            StateServiceFee::Percentage { value } => {
                assert_eq!("1", value.to_string());
            }
        }
    }

    #[test]
    fn percentage_fees_insufficient_funds_error() {
        let tip_amount = 1000;
        let fund_amount = 1099;

        // Fee at 10%
        let service_fees = StateServiceFee::Percentage {
            value: Decimal::from_atomics(10u128, 0u32).unwrap(),
        };

        let tips = vec![Coin::new(tip_amount, "udsm")];
        let funds = vec![Coin::new(fund_amount, "udsm")];
        let computed_fees = service_fees.check_fees(&funds, &tips).unwrap_err();

        assert_eq!(
            ContractError::InsufficientFunds {
                requested: "1100udsm".to_string(),
                provided: "1099udsm".to_string(),
            },
            computed_fees
        );
    }

    #[test]
    fn percentage_fee_fund_coin_not_provided_error() {
        let tip_amount = 1000;
        let fund_amount = 1100;

        // Fee at 10%
        let service_fees = StateServiceFee::Percentage {
            value: Decimal::from_atomics(10u128, 0u32).unwrap(),
        };

        let tips = vec![
            Coin::new(tip_amount, "udsm"),
            Coin::new(tip_amount, "uatom"),
        ];
        let funds = vec![Coin::new(fund_amount, "udsm")];
        let computed_fees = service_fees.check_fees(&funds, &tips).unwrap_err();

        assert_eq!(
            ContractError::InsufficientFunds {
                requested: "1100uatom,1100udsm".to_string(),
                provided: "1100udsm".to_string(),
            },
            computed_fees
        );
    }
}
