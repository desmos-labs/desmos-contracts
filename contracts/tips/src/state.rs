use crate::error::ContractError;
use crate::msg::ServiceFee;
use crate::utils;
use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum StateServiceFees {
    Fixed { amount: Vec<Coin> },
    Percentage { value: u128, decimals: u32 },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub admin: Addr,
    pub subspace_id: u64,
    pub service_fee: StateServiceFees,
    pub tips_record_threshold: u32,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const TIPS_RECORD: Map<(u64, Addr, Addr), Vec<Coin>> = Map::new("tips_record");

impl From<ServiceFee> for StateServiceFees {
    fn from(service_fees: ServiceFee) -> Self {
        match service_fees {
            ServiceFee::Fixed { amount } => StateServiceFees::Fixed {
                amount: utils::merge_coins(amount),
            },
            ServiceFee::Percentage { value, decimals } => StateServiceFees::Percentage {
                value: value.u128(),
                decimals,
            },
        }
    }
}

impl StateServiceFees {
    /// Computes the fees that the contract will holds and the coins that
    /// can be sent to the user.
    /// * `amount` - Coins from which to calculate the fees.
    pub fn compute_fees(
        &self,
        received_coins: Vec<Coin>,
    ) -> Result<(Vec<Coin>, Vec<Coin>), ContractError> {
        let received_coins = utils::merge_coins(received_coins);
        let mut fees: Vec<Coin> = vec![];
        let mut to_user: Vec<Coin> = vec![];

        match self {
            StateServiceFees::Fixed { amount } => {
                let mut received_coins_iter = received_coins.iter();
                for fee in amount {
                    // Find the coin that have the same denom of the current fee coin
                    let received_coin = received_coins_iter
                        .find(|received_coin| received_coin.denom.eq(&fee.denom))
                        .ok_or(ContractError::FeeCoinNotProvided {
                            denom: fee.denom.to_owned(),
                        })?;

                    // Ensure that the received amount is >= of the fee to pay
                    if received_coin.amount.u128() < fee.amount.u128() {
                        return Err(ContractError::InsufficientFee {
                            denom: received_coin.denom.to_owned(),
                            provided: received_coin.amount,
                            requested: fee.amount,
                        });
                    }

                    to_user.push(Coin::new(
                        received_coin.amount.u128() - fee.amount.u128(),
                        &received_coin.denom,
                    ));
                    fees.push(Coin::new(fee.amount.u128(), &received_coin.denom));
                }
            }
            StateServiceFees::Percentage { value, decimals } => {
                for coin in received_coins {
                    let decimal_factor = 10_u128.pow(*decimals);
                    let coin_fee: u128 =
                        (coin.amount.u128() * *value) / (100_u128 * decimal_factor);

                    to_user.push(Coin::new(coin.amount.u128() - coin_fee, coin.denom.clone()));
                    fees.push(Coin::new(coin_fee, coin.denom));
                }
            }
        }

        Ok((fees, to_user))
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ContractError;
    use crate::state::StateServiceFees;
    use cosmwasm_std::{Coin, Uint128};

    #[test]
    fn fixed_fees_insufficient_amount() {
        let requested_amount = 20000;
        let provided_amount = 1000;

        let service_fees = StateServiceFees::Fixed {
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

        let service_fees = StateServiceFees::Fixed {
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
        let fees = vec![
            Coin::new(requested_amount, "uatom"),
            Coin::new(requested_amount, "udsm"),
        ];

        let service_fees = StateServiceFees::Fixed {
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
                Coin::new(provided_amount - requested_amount, "uatom"),
                Coin::new(provided_amount - requested_amount, "udsm"),
            ],
            computed_fees.1
        );
    }

    #[test]
    fn percentage_fees_valid() {
        let three_percent = StateServiceFees::Percentage {
            value: 3300000,
            decimals: 6,
        };

        let mut computed_fees = three_percent
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
