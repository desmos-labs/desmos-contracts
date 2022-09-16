# ADR 004: Tips Contract

## Changelog

- Aug 25, 2022: Initial draft;
- Aug 30, 2022: First review;
- Sept 1, 2022: Second review;
- Sept 5, 2022: Third review;
- Sept 5, 2022: Fourth review;
- Sept 14, 2022: Fifth Review;

## Status
IMPLEMENTED

## Abstract
This ADR defines the architecture of the Tips contract. This contract manages sending and tracking of tips made within
Desmos based applications and exposes queries that allow to gather all users sent and received tips easily.

## Context
Desmos based applications may want to leverage on the web3 features and one of the most popular one is for sure the possibility to tip users. 
Currently, it's easy to integrate a feature that allows sending tokens as tips between users, but it's very difficult to track those tips in an easy way.
Additionally, application developers might want to integrate some form of monetization that allows them to earn a little percentage when users send tips to each other.

## Decision
The idea here is to write a contract called `Tips`, that handles the sending and tracking of user's tips.
This contract allows sending tips between users as well tracking sent and received tips for each one of them.
The tips tracking should be limited to a maximum tracked record number that helps to keep the contract storage as small
as possible. Also, it doesn't make much sense to store all the tips record that a user received because each one of them will
most likely check only the latest received.
Additionally, the contract allows its admin to set a fee that users have to pay in order to be allowed to send a tip. This fee, if not set, defaults to zero.

## Specifications
Here below the specifications for the contract's messages:

### Messages

#### Instantiate
```rust
pub struct InstantiateMsg {
  pub admin: String,
  pub subspace_id: u64,
  pub service_fee: ServiceFee,
  pub tips_history_size: u32
}
```

* The `admin` identifies the user that controls the contract;
* The `subspace_id` identifies the application which is deploying the contract;
* The `service_fee` identifies a fee that the users need to pay to use the contract;
* The `tips_history_size` identifies the number of records saved of a user tips history.

The service fee can be set in two different ways, depending on the needs of the contract's admin.
It can be a `Fixed` or a `Percentage` fee.
* The `Fixed` one is just a fixed amount of tokens the admin requires to be paid as a fee.
* The `Percentage` one depends on the amount of the tip given by the tipper. The admin chooses a percentage to be deducted from the
tip and paid to the contract.

```rust
pub enum ServiceFee {
  Fixed { amount: Vec<Coin> },
  Percentage { value: Decimal }
}
```

#### Execute
```rust
pub enum ExecuteMsg {
  SendTip { amount: Vec<Coin>, target: Target },
  ClaimFees { recipient: String },
  UpdateServiceFee { new_fee: ServiceFee },
  UpdateAdmin { new_admin: String },
  UpdateSavedTipsHistorySize { new_size: u32 }
}
```

```rust
pub enum Target {
  ContentTarget { post_id: u64 },
  UserTarget { receiver: String }
}
```

##### SendTip
With the `SendTip` message the user can call the contract to send a tip to another user to show their support towards a specific content they made.
The `MessagInfo` fields contains:
* the funds necessary to cover the fees plus the tip amount specified inside the `funds` field
* the sender identified by the `sender` field

Ideally the contract should perform the following checks:
* If the tips is for a post check that the post associated with the given `post_id` exists.
* If there's any service fee, check that the service fee plus the tip amount specified inside the `ExecuteMsg::SendTip` are covered from `MessageInfo::founds`.

If the checks pass successfully, then the tip record can be saved.
The contract should keep track of:
1. User sent tips
2. User received tips
3. Tips sent towards a post  
 
For each of these informations the contract should not keep more tips than the configured `tips_history_size`.
If for one of these informations the amount of tips stored become greater then `tips_history_size` the contract should remove the oldest received tips.

##### ClaimFees
With the `ClaimFees` message the contract's admin can withdraw all the collected fees and send them to the given `recipient`.
The `recipient` can be a user or another contract.

##### UpdateServiceFee
With the `UpdateServiceFee` message the user can update the previously set service fee.
The message should make sure that the user tyring to make the edit is the actual contract admin.

##### UpdateAdmin
With the `UpdateAdmin` message the user can update the contract's admin.
The message should make sure that the user trying to make the edit is the actual contract admin.

##### UpdateSavedTipsHistorySize
With the `UpdateSavedTipsHistorySize` message the user can update the max number of tips that the contract keeps regarding:
1. Tips sent from a user
2. Tips received by a user
3. Tips sent towards a post

The message should make sure that the user trying to make the edit is the actual contract admin.

### Query
```rust
pub enum QueryMsg {
  /// Return a ConfigResponse containing the configuration info of the contract
  Config {},
  /// Return a TipsResponse containing all the received tips of the user
  UserReceivedTips { user: String },
  /// Return a TipsResponse containing all the sent tips from the user
  UserSentTips { user: String },
  ///Return a TipsResponse containing all the tips associated with a given post
  PostReceivedTips { post_id: u64 }
}
```

#### Config
The `Config{}` query returns the contract's configuration inside a `ConfigResponse`.
```rust
pub struct QueryConfigResponse {
  pub admin: Addr,
  pub subspace_id: u64,
  pub service_fee: Vec<Coin>,
  pub saved_tips_record_threshold: u64
}
```

#### UserReceivedTips
The `UserReceivedTips{ user }` query returns all the received tips of the given `user` inside a `TipsResponse`.


#### UserSentTips
The `UserSentTips{ user }` query returns all the tips sent by the given `user` inside a `TipsResponse`.


#### PostReceivedTips
The `PostReceivedTips{ post_id }` query returns all the tips associated with the given `post_id` inside a `TipsResponse`

```rust
pub struct TipsResponse {
  pub tips: Vec<Tip>
}
```

```rust
pub struct Tip {
  pub sender: Addr,
  pub receiver: Addr,
  pub amount: Vec<Coin>,
  pub post_id: Option<Uint64>
}
```
