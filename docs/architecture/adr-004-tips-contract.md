# ADR 003: Remarkables Contract

## Changelog

- Aug 24, 2022: Initial draft.

## Status
DRAFTED

## Abstract
This ADR defines the architecture of the Tips contract. This contract manage sending and tracking of tips made within
Desmos based applications and exposes queries that allow to gather all users sent and received tips easily.

## Context
Desmos based applications may want to leverage on the web3 features and one of the most popular one is for sure the possibility to tip users. Currently, it's easy to integrate a feature that allow an application to send tokens as tips between users but very difficult to track those tips in an easy way.
Plus application creators might want to integrate some form of monetization that allows them to earn a little percentage when user send tips to each other.

## Decision
The idea here is to write a contract called `Tips`, that handles the sending and tracking of user's tips.
This contract allows sending tips between users as well tracking sent and received tips for each one of them.
The tips tracking should be limited to a maximum tracked record number that helps to keep the contract storage as small
as possible. Also, it doesn't make much sense to store all the tips record that a user received because each one of them will
most likely check only the latest received.
Additionally, the contract allows its admin to set a fee that users has to pay in order to be allowed to send a tip. This fee, if not set, is equal to zero.

## Specifications
Here below the specifications for the contract's messages:

### Messages

#### Instantiate
```rust
pub struct InstantiateMsg {
  pub admin: String,
  pub subspace_id: u64,
  pub service_fee: Vec<Coin>,
}
```

* The `admin` identifies the user that controls the contract;
* The `subspace_id` identifies the application which is deploying the contract;
* The `service_fee` identifies a fee that the users need to pay to use the contract.

#### Execute
```rust
pub enum ExecuteMsg{
  SendTip{},
  UpdateServiceFee{},
  UpdateAdmin{},
}
```

### Query
```rust
pub enum QueryMsg {
  /// Return a ConfigResponse containing the configuration info of the contract
  Config{},
  ReceivedTips{},
  SentTips{}
}
```

#### Config
The `Config{}` query returns the contract's configuration inside a `ConfigResponse`.
```rust
pub struct QueryConfigResponse {
  pub admin: Addr,
  pub subspace_id: u64,
  pub service_fee: Vec<Coin>,
}
```

