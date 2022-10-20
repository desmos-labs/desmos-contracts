# ADR 006: Advertise contract

## Changelog

- Oct 20, 2022: Initial draft;

## Status

DRAFT

## Abstract
This ADR defines the architecture of the Advertise contract. This contract allows users to advertise a post, then social network applications can query random advertisement posts from it.
Also, to be fair to the post which is advertised more tokens, this contract provide the system to make sure that the more tokens a post is advertised, the more visibility it has.

## Context
The Desmos based social network may want to have a advertise system to make money. Currently, building a advertise system on apps is not easy so we would like to build a tool for it.

## Decision
The idea is to implement a smart contract that allows users to advertise a post by using tokens. The contract will maintain a advertising pool based on [`SumTree`](https://medium.com/kleros/an-efficient-data-structure-for-blockchain-sortition-15d202af3247) to provide a way to get random posts with the query rate defined by how much token advertised. Say, Tim advertises `post 1` by `10dsm` then Tom advertises `post 2` by `90dsm`, then the query rate of the `post 1` would be `10%` and the `post 2` would be `90%`. In addition, advertisement has timeliness so it needs an expiration time.

The contract will take care of:
* allows users to advertise a post by the amount tokens larger the minimum required fees;
* returns random advertising posts;
* gives a way to remove expired advertising posts;
* allows the admin to claim the collected fees.

## Specifications

### Messages

#### Instantiate
```rust
pub struct InstantiateMsg {
    admin: String,
    min_fee_per_day: Coin,
}
```

* The `admin` identifies the user that controls the contract;
* The `min_fee_per_day` defines the minimum daily fee to advertise a post.

***NOTE***
The contract only support one type coin to the `min_fee_per_day` since it will calculate the Ads stakes by the amount of it.

#### Execute
```rust
enum ExecuteMsg {
    AdvertisePost{ post_id: Uint64, days: Option<u32> },
    RemoveExpiredAds{},
    ClaimFees{ recipient: String },
}
```

##### AdvertisePost
With `AdvertisePost` message, users can call the contract to advertise a post in a period.
Here we need to check that:
* The post of the given id exists;
* The funds is larger than the required minimum fee that is `min_fee_per_day * days`.

##### RemoveExpiredAds
With `RemoveExpiredAds` message, users can call the contract to remove expired advertisements.

##### ClaimFees
With `ClaimFees` message, the admin can call the contract to claim the collected fees to the recipient.

### Query
```rust
enum QueryMsg {
    Config{},
    RandomAds{ amount: Option<u32> },
    AdvertisementInfo{ post_id: Uint64 },
}
```

#### Config
The `Config` query returns the contract's configuration inside by `QueryConfigResponse`.

```rust
pub struct QueryConfigResponse {
  pub admin: Addr,
  pub subspace_id: Uint64,
  pub total_stakes: Uint64,
  pub min_fee_per_day: Coin,
}
```

#### RamndomAds
The `RandomAds` query returns the a random set of the advertising post ids by `QueryRandomAdsResponse`.

```rust
pub struct QueryRandomAdsResponse {
    ids: Vec<Uint64>,
}
```

#### AdInfo
The `AdInfo` query returns the advertisement information of the post by `QueryAdInfoResponse`.

```rust
pub struct QueryAdInfoResponse {
    stake: Uint64,
    expiration: Expiration,
}
```

**Note**
Expiration is the structure defined in [`cw_utils::Expiration`](https://docs.rs/cw-utils/latest/cw_utils/enum.Expiration.html).

## References
* SumTree: https://medium.com/kleros/an-efficient-data-structure-for-blockchain-sortition-15d202af3247
* Expiration: https://docs.rs/cw-utils/latest/cw_utils/enum.Expiration.html