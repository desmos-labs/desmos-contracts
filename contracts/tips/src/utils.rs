use cosmwasm_std::Coin;
use std::collections::HashMap;

/// Iterates over the coins vector and merge the coins having the same `denom` value.
/// * `coins` - Vector that will be iterated.
pub fn merge_coins(coins: Vec<Coin>) -> Vec<Coin> {
    let mut map: HashMap<String, u128> = HashMap::new();
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

    // Sort only when the target arch is not wasm32 to make this code deterministic.
    #[cfg(not(target_arch = "wasm32"))]
    {
        coins.sort_by_key(|coin| coin.denom.to_owned());
    }

    coins
}
