use cosmwasm_std::Coin;
use std::collections::btree_map::BTreeMap;

/// Iterates over the coins vector and merges the coins having the same `denom`.
/// * `coins` - Vector of coins to merge.
pub fn merge_coins(coins: Vec<Coin>) -> Vec<Coin> {
    if coins.len() <= 1 {
        return coins;
    }

    let mut map: BTreeMap<String, u128> = BTreeMap::new();
    for coin in coins {
        let value = map.get_mut(&coin.denom);

        if let Some(amount) = value {
            *amount += coin.amount.u128();
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

    coins
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
        ]);

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
        ]);

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
