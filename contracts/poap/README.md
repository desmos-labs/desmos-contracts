# POAP Contract

Contract that allows users to mint POAP nft.

## How to build

Before build the contract make sure you have:
1. Docker installed on your system;
2. The cargo `cargo-run-script` binary. 
This binary can be installed with the `cargo install cargo-run-script` command.

To build the contract from withing the contract directory run:
```shell
cargo optimize
```

This will build the contract and store the compiled wasm code in the `artifacts` directory 
located in the workspace root.

## Instantiate Message

Allows to initialize the contract. This message has the following parameters:
* `name`: Name of the POAP contract;
* `symbol`: Symbol of the POAP contract;
* `metadata_uri`: The URI where users can view the associated metadata for the POAPs, ideally following the ERC-721 metadata scheme in a JSON file;
* `admin`: Who controls the contract. If not set will be used the address of who is instantiating the contract;
* `minter`: Address that is allowed to mint tokens on behalf of other users. If not set will be used the address of who is instantiating the contract;
* `is_transferable`: Specifies whether each POAP can be transferred from one user to another;
* `is_mintable`: Indicates whether users can mint the POAPs;
* `mint_start_time`: Identifies the timestamp at which the minting of the POAP will be enabled. If not set, the minting is always enabled;
* `mint_end_time`: Identifies the timestamp at which the minting of the POAP will be disabled. If not set, the minting will never end.

Here an example message to instantiate the contract:
```json
{
    "name": "poap-nft",
    "symbol": "poap",
    "metadata_uri": "ipfs://poap_metadata",
    "admin": "desmos1......",
    "minter": "desmos1......",
    "is_transferable": true,
    "is_mintable": true
}
```

## Execute messages

This contract extends the `cw721-base` contract and so inherit all the `cw721`.
You can take a look [here](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md#messages) to see the `cw721` messages.

### Mint

Mint a new POAP for the caller. This message has the following parameters:
* `extension`: The POAP extension.

Here an example message to mint a POAP:
```json
{
  "mint": {}
}
```

### MintTo

Mint a new POAP for the provided users, can only be called from the contract minter.
This message have the following parameters:
* `users`: List of users for whom the POAP will be mined;
* `extension`: The POAP extension.

Here an example message to mint a POAP to two users:
```json
{
  "mint_to": {
    "users": [
      "desmos1....",
      "desmos1..."
    ]
  }
}
```

### Burn

Burn a POAP the sender has access to.
This message have the following parameters:
* `token_id`: Id of the POAP that will be burned.

Here an example message to burn a POAP:
```json
{
  "burn": {
    "token_id": "1"
  }
}
```

### UpdateMinter

Allow to update the user with the mint permissions, can only be called from the contract admin.
This message have the following parameters:
* `minter`: Address of the new minter.

Here an example message to update the contract minter:
```json
{
  "update_minter": {
    "minter": "desmos1..."
  }
}
```

### SetMintable

Sets if the users can mint their POAP, can only be called from the contract admin.
This message have the following parameters:
* `mintable`: Boolean value that determines whether users can mint a POAP.

Here an example message to update the POAP mintability:
```json
{
  "set_mintable": {
    "mintable": false
  }
}
```

### SetTransferable

Sets if the users can transfer their POAP, can only be called from the contract admin.
This message have the following parameters:
* `transferable`: Boolean value that determines whether users transfer their POAP.

Here an example message to update the POAP transferability:
```json
{
  "set_transferable": {
    "transferable": false
  }
}
```

### SetMintStartEndTime

Sets the time period of when the POAP can be minted from the users, can only be called from the contract admin.
This message have the following parameters:
* `start_time`: Identifies the timestamp at which the minting of the POAP will be enabled in nanoseconds since 1970-01-01T00:00:00Z. If not set, the minting is always enabled;
* `end_time`: Identifies the timestamp at which the minting of the POAP will be disabled in nanoseconds since 1970-01-01T00:00:00Z. If not set, the minting will never end.

Here an example message that allow the POAP to be minted from the 2023-08-01T00:00:00Z to 2023-08-07T00:00:00Z:
```json
{
  "set_transferable": {
    "start_time": "1690848000000000000",
    "end_time": "1691366400000000000"
  }
}
```

## Query messages

This contract extends the `cw721-base` contract and so inherit all the `cw721`.
You can take a look [here](https://github.com/CosmWasm/cw-nfts/blob/main/packages/cw721/README.md#queries) to see the `cw721` messages.

### Minter

Allows to query the contract minter.

Here an example message to query the contract minter:
```json
{
    "minter": {}
}
```

Response:
```json
{
  "minter": "desmos1..."
}
```

### IsMintable

Allows to query if the POAP can be minted.

Here an example message to query the POAP mintability:
```json
{
    "is_mintable": {}
}
```

Response:
```json
{
  "mintable": true
}
```

### IsTransferable

Allows to query if the POAP can be transferred.

Here an example message to query the POAP mintability:
```json
{
    "is_transferable": {}
}
```

Response:
```json
{
  "transferable": true
}
```

### MintStartEndTime

Allows to query the POAP mint period.

Here an example message to query the POAP mint period:
```json
{
    "mint_start_end_time": {}
}
```

Response:
```json
{
  "start_time": "1690848000000000000",
  "end_time": "1691366400000000000"
}
```