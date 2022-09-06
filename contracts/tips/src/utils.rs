use cosmwasm_std::Coin;
use std::collections::btree_map::BTreeMap;

/// Iterates over the coins vector and merge the coins having the same `denom` value.
/// * `coins` - Vector that will be iterated.
pub fn merge_coins(coins: Vec<Coin>) -> Vec<Coin> {
    let mut map: BTreeMap<String, u128> = BTreeMap::new();
    for coin in coins {
        let value = map
            .get(&coin.denom)
            .map_or(coin.amount.u128(), |value| *value + coin.amount.u128());
        map.insert(coin.denom, value);
    }

    let mut coins: Vec<Coin> = Vec::with_capacity(map.len());
    for entry in map {
        coins.push(Coin {
            denom: entry.0,
            amount: entry.1.into(),
        })
    }

    coins
}
