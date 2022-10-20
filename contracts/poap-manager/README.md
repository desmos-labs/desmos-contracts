# POAP manager contract

The controller contract of the [POAP contract](../poap/README.md) that allows users who has a Desmos profile to mint POAP NFTs.
To easily interact with the contract you can use the `poap-manager` script available [here](https://github.com/desmos-labs/contract-utils/tree/main/utils), 
otherwise you can take a look at the supported messages in the following sections.

## Instantiate Message
Allows to initialize the contract. This message has the following parameters:
* `admin`: Address of who will have the right to administer the contract;
* `poap_code_id`: Id of the POAP contract to be initialized together with this contract;
* `poap_instantiate_msg`: Initialization [message](../poap/README.md#instantiate_message) that will be sent to the POAP contract;

Here an example message to instantiate the contract:
```json
{   
    "admin": "desmos1......",
    "poap_code_id": "1",
    "poap_instantiate_msg": {
        "admin": "desmos1......",
        "minter": "poap_manager_contract_address",
        "cw721_code_id": "2",
        "cw721_instantiate_msg": {
            "name": "poap_nft",
            "symbol": "poap",
            "minter": "poap_contract_address"
        },
        "event_info": {
            "creator": "desmos1......",
            "start_time": "2022-12-31T10:00:00Z",
            "end_time": "2022-12-31T19:00:00Z",
            "per_address_limit": 1,
            "poap_uri": "ipfs://poap.info"
        }
    }
}
```

## Execute Messages

### Claim
Allows users who have a Desmos profile to claim a POAP token during the event if the mint 
has been enabled.

Here an example message to claim a POAP:
```json
{
    "claim": {}
}
```

### MintTo
Allows the admin to mint a POAP token to a recipient during the event if mint was enabled. This message has the following parameter:
* `recipient`: Address who will receive the minted token.

Here an example message to mint a POAP to a user:
```json
{
    "mint_to": {
        "recipient": "desmos1......"
    }
}
```

### UpdateAdmin
Allows the contract's admin to transfer the admin rights to another user. This message has the following parameter:
* `new_admin`: Address of the new admin that will control this contract.

Here an example message to update the contract admin:
```json
{
    "update_admin": {
        "new_admin": "desmos1......"
    }
}
```

## Query Messages

### Config
Allows to query the config of the contract.

Here an example message to query the config:
```json
{
    "config": {}
}
```

Response:
```json
{
    "admin": "desmos1......",
    "poap_code_id": "1",
    "poap_contract_address": "desmos1......"
}
```