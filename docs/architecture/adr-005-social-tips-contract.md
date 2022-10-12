# ADR 005: Social tips contract

## Changelog

- Oct 10, 2022: Initial draft;
- Oct 12, 2022: Second review;

## Status
DRAFT

## Abstract
This ADR defines the architecture of the Social Tips contract. This contract allow sending tips to a user
using their know **handle** of a centralized application.  
The handle can be the @handle of Twitter, the user's nickname on Discord or even an e-mail address.  
Additionally, users may want to send tips to other users that don't have a Desmos profile this contract
can collect those tips and allow the users to later claim it.

# Context
Desmos based applications may want to leverage on the web3 features and one of the most popular one is for sure
the possibility to tip users. Currently, it's very easy to send token to a user knowing is address,
but it's more difficult to send tokens to a user knowing only his identity on a centralized application.

## Decision
The idea here is to write a contract called `Social Tips`, that allows to send tokens to another user 
through their centralized application handle. Also, if the receiver haven't linked yet his 
centralized application handle this contract allow them to claim their tips later after having
proved that they own such identity on the centralized application.

## Specifications
Here below the specifications for the contract's messages:

### Messages

#### Instantiate
```rust
struct InstantiateMsg {
  pub admin: String,
  pub subspace_id: u64,
}
```
* The `admin` identifies the user that controls the contract;
* The `subspace_id` identifies the application which is deploying the contract;

#### Execute
```rust
enum ExecuteMsg {
  SendTip { 
      amount: Vec<Coin>, 
      application: Stting,
      handle: String,
  },
  ClaimTips {},
  UpdateAdmin { new_admin: String }
}
```

##### SendTip
With the `SentTip` message the user can send a tip to another user though the handle of a linked centralized application.
The `MessageInfo` fields contains:
* the funds necessary to cover the fees plus the tip amount specified inside the `funds` field
* the sender identified by the `sender` field

#### ClaimTips
With the `ClaimTips` message the user can claim their pending tips that has been sent to him before proving that
owns the identities to which the tips refer.

##### UpdateAdmin
With the `UpdateAdmin` message the user can update the contract's admin.
The message should make sure that the user trying to make the edit is the actual contract admin.

### Query
```rust
enum QueryMsg {
  Config {},
  UserPendingTips { user: String }
}
```

##### Config
With the `Config` message the user can query the contract's configuration.
The returned configurations are as follows:
```rust
struct QueryConfigResponse {
    pub admin: String,
    pub subspace_id: u64,
}
```

##### UserPendingTips
With the `UserPendingTips` message the user can query the pending tips of the given `user`.
The returned tips are provided as follows:
```rust
struct QueryPendingTipsResponse {
    pub tips: Vec<Tip>,
}

struct Tip {
    pub sender: Addr,
    pub amount: Vec<Coin>,
}
```
