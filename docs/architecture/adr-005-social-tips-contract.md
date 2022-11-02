# ADR 005: Social tips contract

## Changelog

- Oct 10, 2022: Initial draft;
- Oct 12, 2022: Second review;
- Oct 27, 2022: Third review;

## Status
DRAFT

## Abstract
This ADR defines the architecture of the Social Tips contract. This contract allows sending tips to a user
using their know **handle** of a centralized application.  
The handle can be the @handle of Twitter, the user's nickname on Discord or even an e-mail address.  
Additionally, users may want to send tips to other users that don't have a Desmos profile this contract
can collect those tips and allow the users to later claim it.

# Context
Desmos based applications may want to leverage on the web3 features and one of the most popular one is for sure
the possibility to tip users. Currently, it's very easy to send tokens to a user knowing is address,
but it's more difficult to send tokens to a user knowing only his identity on a centralized application.

## Decision
The idea here is to write a contract called `Social Tips`, that allows to send tokens to another user 
through their centralized application handle. Also, if the receiver haven't linked yet his 
centralized application handle to their Desmos profile, this contract allows them to claim their tips later after having
proved that they own such identity on the centralized application.

## Specifications
Here below the specifications for the contract's messages:

### Messages

#### Instantiate
```rust
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub max_pending_tips: u16,
    pub max_sent_pending_tips: u16,
}
```

* The `admin` identifies the user that controls the contract, if it is `None` the contract will pick the address of who sent the transaction;
* The `max_pending_tips` identifies the maximum number of pending tips that a user can have associated to his centralized application;
* The `max_sent_pending_tips` identifies the maximum allowed number of tips that the contracts can collect from a single sender.

#### Execute
```rust
enum ExecuteMsg {
  SendTip { 
      amount: Vec<Coin>, 
      application: String,
      handle: String,
  },
  ClaimTips {},
  UpdateAdmin { new_admin: String },
  UpdateMaxPendingTips { value: u32 },
  UpdateMaxSentPendingTips { value: u32 },
  RemovePendingTip { application: String, handle: String },
}
```

##### SendTip
With the `SentTip` message the user can send a tip to another user though the handle of a linked centralized application.
The `MessageInfo` fields contains:
* the funds necessary to cover the fees plus the tip amount specified inside the `funds` field
* the sender identified by the `sender` field

**NOTE** If the user has already sent a tip to the same centralized application reference, the contracts collects
only the last one then sending the previous tip amount back to the user.

#### ClaimTips
With the `ClaimTips` message the user can claim their pending tips that has been sent to him before proving that
owns the identities to which the tips refer.

##### UpdateAdmin
With the `UpdateAdmin` message, the current admin can update the contract's admin.

### UpdateMaxPendingTips
With the `UpdateMaxPendingTips` message, the current admin can change the amount of pending tips that
can be associated to a centralized application user.

### UpdateMaxSentPendingTips
With the `UpdateMaxPendingTips` message, the current admin can change the amount of tips that
the contract collects for a specific sender address.

### RemovePendingTip
With the `RemovePendingTip` message, the user can cancel a tip that haven't been claimed yet.

### Query
```rust
enum QueryMsg {
  UserPendingTips { user: String }, 
  UnclaimedSentTips { user: String },
  Config {},  
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
    pub block_height: u64,
}
```

##### UnclaimedSentTips
With the `UnclaimedSentTips` message, the user can query the unclaimed tips that has been sent.
The returned tips are provided as follows:
```rust
struct QueryUnclaimedSentTipsResponse {
    pub tips: Vec<Tip>,
}

struct Tip {
    pub sender: Addr,
    pub amount: Vec<Coin>,
    pub block_height: u64,
}
```

### Config
With the  `Config{}` message, a user can query the current contract's configurations.
The returned configuration are provided as follows:
```rust
pub struct QueryConfigResponse {
  pub admin: Addr,
  pub max_pending_tips: u16,
  pub max_sent_pending_tips: u16,
}
```