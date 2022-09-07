use crate::error::ContractError;
use crate::msg::ServiceFee;
use crate::utils;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::vec_deque::VecDeque;
use std::convert::TryFrom;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StateServiceFee {
    Fixed { amount: Vec<Coin> },
    Percentage { value: u128, decimals: u32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub subspace_id: u64,
    pub service_fee: StateServiceFee,
    pub tips_record_threshold: u32,
}

pub const CONFIG: Item<Config> = Item::new("config");
/// Tips record key where:
/// 0. Tip sender address
/// 1. Tip receiver address
/// 2. Tip post, if 0 means that is not referencing a post.
pub type TipsRecordKey = (Addr, Addr, u64);
pub const TIPS_RECORD: Map<TipsRecordKey, Vec<Coin>> = Map::new("tips_record");
// Keeps the keys of TIPS_RECORD ordered by insertion time, first oldest, last newest.
pub const TIPS_KEY_LIST: Item<VecDeque<TipsRecordKey>> = Item::new("tips_key_list");

impl StateServiceFee {
    /// Computes the fees that the contract will holds and the coins that
    /// can be sent to the user.
    /// * `coins` - Coins from which to calculate the fees.
    pub fn compute_fees(&self, coins: Vec<Coin>) -> Result<(Vec<Coin>, Vec<Coin>), ContractError> {
        let received_coins = utils::merge_coins(coins);
        let mut fees: Vec<Coin> = vec![];
        let mut to_user: Vec<Coin> = vec![];

        match self {
            StateServiceFee::Fixed { amount } => {
                for coin in received_coins {
                    let fee_option = amount
                        .iter()
                        .find(|fee_coin| fee_coin.denom.eq(&coin.denom));
                    // This coin is present inside the service fees.
                    if let Some(fee) = fee_option {
                        // Return error if the provided amount is not enough.
                        if fee.amount.u128() > coin.amount.u128() {
                            return Err(ContractError::InsufficientFee {
                                denom: coin.denom.to_owned(),
                                provided: coin.amount,
                                requested: fee.amount,
                            });
                        }

                        // Update the and the coins to send to the user
                        fees.push(fee.clone());

                        let to_user_coin =
                            Coin::new(coin.amount.u128() - fee.amount.u128(), &coin.denom);
                        if !to_user_coin.amount.is_zero() {
                            to_user.push(to_user_coin);
                        }
                    } else {
                        to_user.push(coin);
                    }
                }
                // Ensure that we have processed all the fees
                if amount.len() > fees.len() {
                    for fee in amount {
                        let coin_found = fees
                            .iter()
                            .find(|user_fee| user_fee.denom.eq(&fee.denom))
                            .is_some();
                        if !coin_found {
                            return Err(ContractError::FeeCoinNotProvided {
                                denom: fee.denom.to_owned(),
                            });
                        }
                    }
                }
            }
            StateServiceFee::Percentage { value, decimals } => {
                let decimal_factor = 10_u128.pow(*decimals);
                for coin in received_coins {
                    let coin_fee: u128 =
                        (coin.amount.u128() * *value) / (100_u128 * decimal_factor);

                    to_user.push(Coin::new(coin.amount.u128() - coin_fee, coin.denom.clone()));
                    if coin_fee > 0 {
                        fees.push(Coin::new(coin_fee, coin.denom));
                    }
                }
            }
        }

        Ok((fees, to_user))
    }
}

impl TryFrom<ServiceFee> for StateServiceFee {
    type Error = ContractError;

    fn try_from(service_fees: ServiceFee) -> Result<Self, ContractError> {
        match service_fees {
            ServiceFee::Fixed { amount } => Ok(StateServiceFee::Fixed {
                amount: utils::merge_coins(amount),
            }),
            ServiceFee::Percentage { value, decimals } => {
                let percent_value = value.u128() / 10_u128.pow(decimals);
                if percent_value >= 100 {
                    Err(ContractError::InvalidPercentageFee {})
                } else {
                    Ok(StateServiceFee::Percentage {
                        value: value.u128(),
                        decimals,
                    })
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ContractError;
    use crate::state::StateServiceFee;
    use cosmwasm_std::{Coin, Uint128};

    #[test]
    fn fixed_fees_insufficient_amount() {
        let requested_amount = 20000;
        let provided_amount = 1000;

        let service_fees = StateServiceFee::Fixed {
            amount: vec![Coin::new(requested_amount, "udsm")],
        };

        let computed_fees = service_fees
            .compute_fees(vec![Coin::new(provided_amount, "udsm")])
            .unwrap_err();

        assert_eq!(
            ContractError::InsufficientFee {
                denom: "udsm".to_string(),
                provided: Uint128::new(provided_amount),
                requested: Uint128::new(requested_amount)
            },
            computed_fees
        );
    }

    #[test]
    fn fixed_fees_fee_coin_not_provided() {
        let requested_amount = 20000;
        let provided_amount = 100000;

        let service_fees = StateServiceFee::Fixed {
            amount: vec![
                Coin::new(requested_amount, "udsm"),
                Coin::new(requested_amount, "uatom"),
            ],
        };

        let computed_fees = service_fees
            .compute_fees(vec![Coin::new(provided_amount, "udsm")])
            .unwrap_err();

        assert_eq!(
            ContractError::FeeCoinNotProvided {
                denom: "uatom".to_string(),
            },
            computed_fees
        );
    }

    #[test]
    fn fixed_fees_fee_coin_valid() {
        let requested_amount = 20000;
        let provided_amount = 100000;
        let fees = vec![Coin::new(requested_amount, "udsm")];

        let service_fees = StateServiceFee::Fixed {
            amount: fees.clone(),
        };

        let computed_fees = service_fees
            .compute_fees(vec![
                Coin::new(provided_amount, "uatom"),
                Coin::new(provided_amount, "udsm"),
            ])
            .unwrap();

        assert_eq!(fees, computed_fees.0);

        assert_eq!(
            vec![
                Coin::new(provided_amount, "uatom"),
                Coin::new(provided_amount - requested_amount, "udsm"),
            ],
            computed_fees.1
        );
    }

    #[test]
    fn zero_percentage_fees() {
        let zero_percent = StateServiceFee::Percentage {
            value: 1,
            decimals: 0,
        };

        let computed_fees = zero_percent
            .compute_fees(vec![
                Coin::new(1000, "udsm"),
                Coin::new(600, "uatom"),
                Coin::new(1000, "uosmo"),
            ])
            .unwrap();

        assert_eq!(Vec::<Coin>::new(), computed_fees.0);
    }

    #[test]
    fn percentage_fees_valid() {
        let three_percent = StateServiceFee::Percentage {
            value: 3300000,
            decimals: 6,
        };

        let computed_fees = three_percent
            .compute_fees(vec![
                Coin::new(100, "udsm"),
                Coin::new(600, "uatom"),
                Coin::new(1000, "uosmo"),
            ])
            .unwrap();

        assert_eq!(
            vec![
                Coin::new(19, "uatom"),
                Coin::new(3, "udsm"),
                Coin::new(33, "uosmo"),
            ],
            computed_fees.0
        );
        assert_eq!(
            vec![
                Coin::new(581, "uatom"),
                Coin::new(97, "udsm"),
                Coin::new(967, "uosmo"),
            ],
            computed_fees.1
        );
    }
}
