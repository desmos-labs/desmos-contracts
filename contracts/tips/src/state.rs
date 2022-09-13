use crate::error::ContractError;
use crate::msg::ServiceFee;
use crate::utils;
use cosmwasm_std::{Addr, Coin, Decimal, Uint128};
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, MultiIndex, UniqueIndex};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
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
    pub tips_record_threshold: u32,
}

/// Represents the tip that is saved inside the contract state
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct StateTip {
    /// Address of who sent the tip.
    pub sender: Addr,
    /// Address of who received the tip.
    pub receiver: Addr,
    /// Id of the post to which the tip refers.
    pub post_id: u64,
    /// Tip amount.
    pub amount: Vec<Coin>,
    /// Block height at which the tip took place.
    pub block_height: u64,
    /// Index to avoid collision when more then one tip is received in the same block.
    pub block_height_index: u64,
}

pub struct StateTipIndex<'a> {
    /// Index to map a sender with block height and block height index to a [`StateTip`] to
    /// have the tips sent from an user sorted by sent time.
    pub sender: UniqueIndex<'a, (Addr, (u64, u64)), StateTip, String>,
    /// Index to map a receiver with block height and block height index to a [`StateTip`] to
    /// have the tips received from an user sorted by sent time.
    pub receiver: UniqueIndex<'a, (Addr, (u64, u64)), StateTip, String>,
    /// Index to map a post id to a list of [`StateTip`].
    pub post_id: MultiIndex<'a, u64, StateTip, String>,
}

impl IndexList<StateTip> for StateTipIndex<'_> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<StateTip>> + '_> {
        let vec: Vec<&dyn Index<StateTip>> = vec![&self.sender, &self.receiver, &self.post_id];
        Box::new(vec.into_iter())
    }
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const BLOCK_HEIGHT_INDEX: Item<(u64, u64)> = Item::new("block_height_index");

pub fn tips_record<'a>() -> IndexedMap<'a, String, StateTip, StateTipIndex<'a>> {
    IndexedMap::new(
        "tips_record",
        StateTipIndex {
            sender: UniqueIndex::new(
                |tip| {
                    (
                        tip.sender.clone(),
                        (tip.block_height, tip.block_height_index),
                    )
                },
                "tips_record__sender",
            ),
            receiver: UniqueIndex::new(
                |tip| {
                    (
                        tip.receiver.clone(),
                        (tip.block_height, tip.block_height_index),
                    )
                },
                "tips_record__receiver",
            ),
            post_id: MultiIndex::new(|tip| tip.post_id, "tips_record", "tips_record__post_id"),
        },
    )
}

impl StateServiceFee {
    /// Computes the fees that the contract will holds and the coins that
    /// can be sent to the user.
    /// * `coins` - Coins from which to calculate the fees.
    pub fn compute_fees(&self, coins: Vec<Coin>) -> Result<(Vec<Coin>, Vec<Coin>), ContractError> {
        let received_coins = utils::merge_coins(coins)?;
        match self {
            StateServiceFee::Fixed { amount } => {
                StateServiceFee::compute_fixed_service_fees(amount, received_coins)
            }
            StateServiceFee::Percentage { value } => {
                StateServiceFee::compute_percentage_service_fees(value, received_coins)
            }
        }
    }

    fn compute_fixed_service_fees(
        fee_coins: &[Coin],
        received_coins: Vec<Coin>,
    ) -> Result<(Vec<Coin>, Vec<Coin>), ContractError> {
        let mut fees: Vec<Coin> = vec![];
        let mut to_user: Vec<Coin> = vec![];

        for coin in received_coins {
            let fee_option = fee_coins
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

                // Update the fees array
                fees.push(fee.clone());

                let to_user_coin = Coin::new(coin.amount.u128() - fee.amount.u128(), &coin.denom);
                if !to_user_coin.amount.is_zero() {
                    to_user.push(to_user_coin);
                }
            } else {
                to_user.push(coin);
            }
        }
        // Ensure that we have processed all the fees
        if fee_coins.len() > fees.len() {
            for fee in fee_coins {
                let fee_found = fees.iter().any(|user_fee| user_fee.denom.eq(&fee.denom));
                if !fee_found {
                    return Err(ContractError::FeeCoinNotProvided {
                        denom: fee.denom.to_owned(),
                    });
                }
            }
        }

        Ok((fees, to_user))
    }

    fn compute_percentage_service_fees(
        value: &Decimal,
        received_coins: Vec<Coin>,
    ) -> Result<(Vec<Coin>, Vec<Coin>), ContractError> {
        let mut fees: Vec<Coin> = vec![];
        let mut to_user: Vec<Coin> = vec![];
        let percentage_value = value.div(Decimal::from_atomics(100u32, 0).unwrap());

        for coin in received_coins {
            // Safe to mul since we are percentage value is (0, 1) and coin amount can't overflow
            let coin_fee: Uint128 = coin.amount.mul(percentage_value);

            if coin_fee.u128() > 0 {
                to_user.push(Coin::new(
                    coin.amount.u128() - coin_fee.u128(),
                    coin.denom.clone(),
                ));
                fees.push(Coin::new(coin_fee.u128(), coin.denom));
            } else {
                to_user.push(Coin::new(coin.amount.u128() - coin_fee.u128(), coin.denom));
            }
        }

        Ok((fees, to_user))
    }
}

impl TryFrom<ServiceFee> for StateServiceFee {
    type Error = ContractError;

    fn try_from(service_fees: ServiceFee) -> Result<Self, ContractError> {
        service_fees.validate()?;

        match service_fees {
            ServiceFee::Fixed { amount } => Ok(StateServiceFee::Fixed {
                amount: utils::merge_coins(amount)?,
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
    fn fixed_fees_empty() {
        let provided_amount = 100000;

        let service_fees = StateServiceFee::Fixed { amount: vec![] };

        let (fees, to_user) = service_fees
            .compute_fees(vec![Coin::new(provided_amount, "udsm")])
            .unwrap();

        assert!(fees.is_empty());
        assert_eq!(vec![Coin::new(provided_amount, "udsm")], to_user);
    }

    #[test]
    fn fixed_fees_computes_properly() {
        let requested_amount = 20000;
        let provided_amount = 100000;
        let fees = vec![Coin::new(requested_amount, "udsm")];

        let service_fees = StateServiceFee::Fixed {
            amount: fees.clone(),
        };

        let (computed_fees, to_user) = service_fees
            .compute_fees(vec![
                Coin::new(provided_amount, "uatom"),
                Coin::new(provided_amount, "udsm"),
            ])
            .unwrap();

        assert_eq!(fees, computed_fees);

        assert_eq!(
            vec![
                Coin::new(provided_amount, "uatom"),
                Coin::new(provided_amount - requested_amount, "udsm"),
            ],
            to_user
        );
    }

    #[test]
    fn percentage_state_service_fee_from_service_fee_invalid_percentage() {
        // Service fees at 200%
        let service_fee = ServiceFee::Percentage {
            value: Decimal::from_atomics(200u32, 0).unwrap(),
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
    fn percentage_fees_zero() {
        let zero_percent = StateServiceFee::Percentage {
            value: Decimal::zero(),
        };
        let tips = vec![
            Coin::new(600000000, "uatom"),
            Coin::new(100000000, "udsm"),
            Coin::new(100000000, "uosmo"),
        ];

        let (computed_fees, to_user) = zero_percent.compute_fees(tips.clone()).unwrap();

        assert_eq!(Vec::<Coin>::new(), computed_fees);
        assert_eq!(tips, to_user);
    }

    #[test]
    fn percentage_fees_compute_properly() {
        let three_percent = StateServiceFee::Percentage {
            value: Decimal::from_atomics(3300000u32, 6).unwrap(),
        };

        let (computed_fees, to_user) = three_percent
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
            computed_fees
        );
        assert_eq!(
            vec![
                Coin::new(581, "uatom"),
                Coin::new(97, "udsm"),
                Coin::new(967, "uosmo"),
            ],
            to_user
        );
    }
}
