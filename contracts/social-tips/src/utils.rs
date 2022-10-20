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
