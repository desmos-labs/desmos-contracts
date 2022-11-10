use crate::ContractError;
use cosmwasm_std::{Coin, OverflowError, OverflowOperation, StdError};
use std::collections::btree_map::BTreeMap;

/// Iterates over the coins vector and merges the coins having the same `denom`
/// and return them sorted by denom.
/// * `coins` - Vector of coins to merge.
pub fn sum_coins_sorted(coins: Vec<Coin>) -> Result<Vec<Coin>, ContractError> {
    if coins.len() <= 1 {
        return Ok(coins);
    }

    let mut map: BTreeMap<String, u128> = BTreeMap::new();
    for coin in coins {
        let value = map.get_mut(&coin.denom);

        if let Some(amount) = value {
            *amount = amount.checked_add(coin.amount.u128()).ok_or_else(|| {
                StdError::overflow(OverflowError {
                    operation: OverflowOperation::Add,
                    operand1: amount.to_string(),
                    operand2: coin.amount.to_string(),
                })
            })?;
        } else {
            map.insert(coin.denom, coin.amount.u128());
        }
    }

    let mut coins: Vec<Coin> = Vec::with_capacity(map.len());
    for (denom, amount) in map.into_iter() {
        coins.push(Coin {
            denom,
            amount: amount.into(),
        })
    }

    Ok(coins)
}

/// Serialize a slice of [`Coin`] into where each coin is separated by a "," (comma).
/// * `coins` - Coins slice to serialize.
pub fn serialize_coins(coins: &[Coin]) -> String {
    coins
        .iter()
        .map(Coin::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

#[cfg(test)]
mod tests {
    use crate::error::ContractError;
    use crate::utils::{serialize_coins, sum_coins_sorted};
    use cosmwasm_std::{Coin, OverflowError, OverflowOperation, StdError};

    #[test]
    fn test_coin_merge_duplicates_properly() {
        let merged = sum_coins_sorted(vec![
            Coin::new(100, "uatom"),
            Coin::new(3000, "udsm"),
            Coin::new(1000, "uosmo"),
            Coin::new(200, "udsm"),
            Coin::new(2000, "uatom"),
        ])
        .unwrap();

        assert_eq!(
            vec![
                Coin::new(2100, "uatom"),
                Coin::new(3200, "udsm"),
                Coin::new(1000, "uosmo")
            ],
            merged
        )
    }

    #[test]
    fn test_coin_merge_no_duplicates_properly() {
        let merged = sum_coins_sorted(vec![
            Coin::new(100, "uatom"),
            Coin::new(3000, "udsm"),
            Coin::new(1000, "uosmo"),
        ])
        .unwrap();

        assert_eq!(
            vec![
                Coin::new(100, "uatom"),
                Coin::new(3000, "udsm"),
                Coin::new(1000, "uosmo")
            ],
            merged
        )
    }

    #[test]
    fn test_coin_merge_overflow_error() {
        let overflow_err = sum_coins_sorted(vec![
            Coin::new(u128::MAX - 1, "uatom"),
            Coin::new(3000, "uatom"),
        ])
        .unwrap_err();

        assert_eq!(
            ContractError::Std(StdError::overflow(OverflowError {
                operation: OverflowOperation::Add,
                operand1: (u128::MAX - 1).to_string(),
                operand2: 3000.to_string(),
            })),
            overflow_err
        )
    }

    #[test]
    fn test_sort_coin_properly() {
        let merged = sum_coins_sorted(vec![
            Coin::new(100, "uosmo"),
            Coin::new(1000, "uatom"),
            Coin::new(1000, "udsm"),
        ])
        .unwrap();

        assert_eq!(
            vec![
                Coin::new(1000, "uatom"),
                Coin::new(1000, "udsm"),
                Coin::new(100, "uosmo")
            ],
            merged
        )
    }

    #[test]
    fn serialize_coins_properly() {
        assert_eq!(
            "100uatom,100udsm",
            serialize_coins(&[Coin::new(100, "uatom"), Coin::new(100, "udsm")])
        )
    }
}
