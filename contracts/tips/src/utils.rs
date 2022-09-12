use crate::error::ContractError;
use cosmwasm_std::{Coin, OverflowError, OverflowOperation, StdError};
use std::collections::btree_map::BTreeMap;

/// Iterates over the coins vector and merges the coins having the same `denom`.
/// * `coins` - Vector of coins to merge.
pub fn merge_coins(coins: Vec<Coin>) -> Result<Vec<Coin>, ContractError> {
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

#[cfg(test)]
mod tests {
    use crate::utils::merge_coins;
    use cosmwasm_std::Coin;

    #[test]
    fn test_coin_merge_duplicates() {
        let merged = merge_coins(vec![
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
    fn test_coin_merge_no_duplicates() {
        let merged = merge_coins(vec![
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
}
