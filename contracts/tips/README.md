# Tips contract

Contract that allows sending tips between users as well tracking sent and received tips for each one of them.  
To easily interact with the contract you can use the `tips` script available [here](https://github.com/desmos-labs/contract-utils/tree/main/utils), 
otherwise you can take a look at the supported messages in the following sections.

## Instantiate Message
Allows to initialize the contract. This message has the following parameters:
* `admin`: Address of the user that controls the contract;
* `subspace_id`: Application which is deploying the contract;
* `service_fee`: Fee that the users need to pay to use the contract, can be percentage, fixed or 
`null` to signal that the contract shouldn't collect fees;
* `tips_history_size`: Number of records saved of a user tips history, can be also `0` to signal that the contract
shouldn't save any tip history.

An example of instantiate message with a percentage fee
```json
{
  "admin": "desmos1.....",
  "subspace_id": "1",
  "service_fee": {
    "percentage": {
      "value": "0.1"
    }
  },
  "tips_history_size": 10
}
```
If you prefer to have a fixed fee instead you can replace the `service_fee` object with something like this
```json
{
  "fixed": {
    "amount": [
      {
        "amount": "1000",
        "denom": "udsm"
      }
    ]
  }
}
```

## Execute Messages

### SendTip
Allows to send a tips to a user or to the author of a post. This message has the following parameters:
* `amount`: Tip amount;
* `target`: Who receives the tip, can be a user address or a post id to signal appreciation for a post created from a user.  

**NOTE**: In order to be able to send the tip you must provide a sufficient amount of coins through the `funds` field
of [MsgExecuteContract](https://github.com/CosmWasm/wasmd/blob/6a471a4a16730e371863067b27858f60a3996c91/proto/cosmwasm/wasm/v1/tx.proto#L74).

Here an example of message to send a tip toward a user post:
```json
{
  "send_tip": {
    "target": {
      "content_target": {
        "post_id": "1"
      }
    },
    "amount": [
      {
        "amount": "100000000",
        "denom": "udsm"
      }
    ]
  }
}
```
If you want to send the tip toward a specific user you can replace the target object with:
```json
{
  "user_target": {
    "receiver": "desmos1..."
  }
}
```

### UpdateServiceFee
Allows the contract admin to update the fees collected from the contract when a user want to send a tip.
This message has the following parameter:
* `new_fee`: Fee that is collected, can be percentage, fixed or `null` to signal that the contract shouldn't collect fees.

Here an example of message to update the service fees to fixed 1DSM:
```json
{
  "update_service_fee": {
    "new_fee": {
      "fixed": [
        {
          "amount": "1000000",
          "denom": "udsm"
        }
      ]
    }
  }
}
```

### UpdateAdmin
Allows the contract admin to update the contract admin. This message has the following parameter:
* `new_admin`: Address of the new admin.

Here an example of message to update the contract admin:
```json
{
  "update_admin": {
    "new_admin": "desmos1...."
  }
}
```

### UpdateSavedTipsHistorySize
Allows the contract admin to update the number of record saved in the tips history. 
This message has the following parameter:
* `new_size`: New tips history size, can be also `0` to signal that the contract shouldn't save any tip.

Here an example of message to update the contract tips history size to 10:

```json
{
  "update_saved_tips_history_size": {
    "new_size": 10
  }
}
```

### ClaimFees
Allows the contract admin to claim the fees paid from the users to execute the contract. 
This message has the following parameter:
* `receiver`: Address to which fees will be sent.

Here an example of message to claim the fees:
```json
{
  "claim_fees": {
    "receiver": "desmos1..."
  }
}
```

## Query Messages

### Config
Allows to query the current contract configurations.

Here the json message to query the configurations:
```json
{
  "config": {}
}
```

### UserReceivedTips
Allows to query a user's received tips. This message has the following parameter:
* `user`: Address of the user of interest.

Here an example of message to query the received tips:
```json
{
  "user_received_tips": {
    "user": "desmos1..."
  }
}
```

### UserSentTips
Allows to query the tips that a user has sent. This message has the following parameter:
* `user`: Address of the user of interest.

Here an example of message to query the tips sent from an user:
```json
{
  "user_sent_tips": {
    "user": "desmos1..."
  }
}
```

### PostReceivedTips
Allows to query the tips that has been sent toward a post. This message has the following parameter:
* `post_id`: Id of the post of interest.

Here an example of message to query the tips toward the post with id 42:
```json
{
  "post_received_tips": {
    "post_id": "42"
  }
}
```
