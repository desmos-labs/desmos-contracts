use crate::error::ContractError;
use crate::msg::ServiceFee;
use crate::utils::sum_coins_sorted;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::convert::TryFrom;
use std::ops::{Div, Mul};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StateServiceFee {
    Fixed { amount: Vec<Coin> },
    Percentage { value: Decimal },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub subspace_id: u64,
    pub service_fee: Option<StateServiceFee>,
    pub tips_history_size: u32,
}

pub type SentTip = (Addr, Vec<Coin>, u64);
pub type ReceivedTip = (Addr, Vec<Coin>, u64);
pub type PostTip = (Addr, Vec<Coin>);

pub const CONFIG: Item<Config> = Item::new("config");
pub const SENT_TIPS_HISTORY: Map<Addr, VecDeque<SentTip>> = Map::new("sent_tips_history");
pub const RECEIVED_TIPS_HISTORY: Map<Addr, VecDeque<ReceivedTip>> =
    Map::new("received_tips_history");
pub const POST_TIPS_HISTORY: Map<u64, VecDeque<PostTip>> = Map::new("received_tips_history");

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
        // Check fees + tips < funds
        for fee_plus_tip in sum_coins_sorted(fee)?.drain(0..) {
            // Search the fee coin inside the funds sent to the contract
            let fund_coin_amount = funds
                .binary_search_by(|coin| coin.denom.cmp(&fee_plus_tip.denom))
                .map(|index| funds[index].amount)
                .map_err(|_| ContractError::InsufficientAmount {
                    denom: fee_plus_tip.denom.clone(),
                    requested: fee_plus_tip.amount,
                    provided: Uint128::zero(),
                })?;

            // Ensure tip amount + fee <= provided funds
            if fee_plus_tip.amount > fund_coin_amount {
                return Err(ContractError::InsufficientAmount {
                    denom: fee_plus_tip.denom,
                    requested: fee_plus_tip.amount,
                    provided: fund_coin_amount,
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
    use cosmwasm_std::{Coin, Decimal, Uint128};
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
    fn fixed_fees_insufficient_funds() {
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
            ContractError::InsufficientAmount {
                denom: "udsm".to_string(),
                requested: Uint128::new(fixed_fee_amount + tip_amount),
                provided: Uint128::new(fund_amount),
            },
            computed_fees
        );
    }

    #[test]
    fn fixed_fees_fee_coin_not_provided() {
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
            ContractError::InsufficientAmount {
                denom: "uatom".to_string(),
                requested: fixed_fee_amount.into(),
                provided: Uint128::zero()
            },
            computed_fees
        );
    }

    #[test]
    fn fixed_fees_fee_fund_coin_not_provided() {
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
            ContractError::InsufficientAmount {
                denom: "uatom".to_string(),
                requested: tip_amount.into(),
                provided: Uint128::zero(),
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
    fn percentage_state_service_fee_from_service_fee_invalid_percentage() {
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
    fn percentage_fees_insufficient_funds() {
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
            ContractError::InsufficientAmount {
                denom: "udsm".to_string(),
                requested: Uint128::new(tip_amount + 100),
                provided: Uint128::new(fund_amount),
            },
            computed_fees
        );
    }

    #[test]
    fn percentage_fee_fund_coin_not_provided() {
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
            ContractError::InsufficientAmount {
                denom: "uatom".to_string(),
                requested: Uint128::new(tip_amount + 100),
                provided: Uint128::zero(),
            },
            computed_fees
        );
    }
}
