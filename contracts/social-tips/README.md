# Social tips contract

Contract that allows to send tokens to another user through their centralized application handle.  
To easily interact with the contract you can use the `social-tips` script available [here](https://github.com/desmos-labs/contract-utils/tree/main/utils),
otherwise you can take a look at the supported messages in the following sections.

## Instantiate Message

Allows to initialize the contract.  
This message has the following parameters:
* `admin`: Address of the user that controls the contract;
* `max_pending_tips`: Maximum number of pending tips that a user can have associated to his centralized application;
* `max_sent_pending_tips`: Maximum allowed number of tips that the contracts can collect from a single sender.

Here an example message to instantiate the contract:
```json
{   
    "admin": "desmos1......",
    "max_pending_tips": 10,
    "max_sent_pending_tips": 5
}
```

## Execute Messages

### SendTip

Allows to send a tip to a user through their centralized application handle.  
This message have the following parameters:
* `application`: Centralized application name;
* `handle`: User handle in the provided centralized application;
* `owner_index`: Optional index of the address to which the tip will be sent in case the user have linked the centralized application
to multiple addresses.

**NOTE**: The tip amount must be provided through the `funds` field of 
[MsgExecuteContract](https://github.com/CosmWasm/wasmd/blob/6a471a4a16730e371863067b27858f60a3996c91/proto/cosmwasm/wasm/v1/tx.proto#L74).

Here an example message to send a tip to a user:
```json
{
  "application": "twitter",
  "handle": "DesmosNetwork"
}
```

Here an example message to send a tip to the second linked address of the provided centralized application handle:
```json
{
  "send_tip": {
    "application": "twitter",
    "handle": "DesmosNetwork",
    "owner_index": 1
  }
}
```

### ClaimTips

Allows a user to claim their pending tips in case someone have sent it before the user have linked their centralized
application handle to the Desmos profile.  

Here an example message to claim the pending tips:
```json
{
  "claim_tips": {}
}
```

### UpdateAdmin

Allows the contract admin to update the contract admin.   
This message have the following parameter:
* `new_admin`: Address of the new admin.

Here an example message to update the contract admin:
```json
{
  "update_admin": {
    "new_admin": "desmos1...."
  }
}
```

### UpdateMaxPendingTips

Allows the contract admin to update the maximum number of pending tips that a user can have associated to his centralized application.  
This message have the following parameter:
* `value`: Maximum number of pending tips that a user can have associated to his centralized application.

Here an example message to update the maximum number of pending tips that a user can have associated to his centralized application:
```json
{
  "update_max_pending_tips": {
    "value": 10
  }
}
```

### UpdateMaxSentPendingTips

Allows the contract admin to update the maximum allowed number of tips that the contracts can collect from a single sender.  
This message have the following parameter:
* `value`: Maximum allowed number of tips that the contracts can collect from a single sender.

Here an example message to update the maximum allowed number of tips that the contracts can collect from a single sender:
```json
{
  "update_max_pending_tips": {
    "value": 5
  }
}
```

### RemovePendingTip

Allows a user to remove a tip that hasn't been collected from the receiver.  
This message has the following parameters:
* `application`: Name of the centralized application;
* `handle`: User handle;

Here an example message to remove an unclaimed tip sent to the **DesmosNetwork** twitter handle.
```json
{
  "remove_pending_tip": {
    "application": "twitter",
    "handle": "DesmosNetwork"
  }
}
```

## Query Messages

#### UserPendingTips

Allows a user to query the tips that can be collected from a user.  
This message have the following parameter:
* `user`: Address of the user of interest.

Here an example message to query the pending tips of a user:
```json
{
  "user_pending_tips": {
    "user": "desmos1..."
  }
}
```

Response:
```json
{
  "tips": [
    {
      "sender": "desmos1...",
      "amount": [{
        "amount": "10000",
        "denom": "udsm"
      }]
    }
  ]
}
```

#### UnclaimedSentTips

Allows a user to query the tips that has sent that aren't be claimed.  
This message have the following parameter:
* `user`: Address of the user of interest.

Here an example message to query the unclaimed tips sent from a user:
```json
{
  "unclaimed_sent_tips": {
    "user": "desmos1..."
  }
}
```

Response:
```json
{
  "tips": [
    {
      "sender": "desmos1...",
      "amount": [{
        "amount": "10000",
        "denom": "udsm"
      }]
    }
  ]
}
```

### Config

Allows to query the current contract configurations.  
Here the json message to query the configurations:
```json
{
  "config": {}
}
```

Response:
```json
{
  "admin": "desmos1...",
  "max_pending_tips": 10,
  "max_sent_pending_tips": 5
}
```
