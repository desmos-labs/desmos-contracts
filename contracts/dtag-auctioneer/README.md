# DTag auctioneer contract

## Store, instantiate and Interact with the contract on Desmos chain
### Store the contract (if not stored before)
```bash
desmos tx wasm store <contract_name.wasm> --chain-id <chain_id> --from <key_name> --gas 1050000
```
### Instantiate the contract
First you need to get the `code_id` of the previously stored contract.
You can check it from the `tx_response` or alternatively by executing the following query:
```bash
desmos query wasm list-code
```
Response's example:
```bash
code_infos:
- creator: desmos1k8u92hx3k33a5vgppkyzq6m4frxx7ewnlkyjrh
  data_hash: 151EF9413F16C8953EE18FE527692B5DEA142EBF02027C3564852AC874844B7A
  id: 1
pagination: {}
```
After getting the contract's id you can now instantiate it by doing:
```bash
desmos tx wasm instantiate <code_id> '{"contract_dtag": "your_dtag_here"}' --label <contract_name> --from <key_name> --chain-id <chain_id> --amount <amount>
```

### Interact with the contract
```bash
desmos tx wasm execute <contract_address> '{"ask_me_for_dtag_transfer_request": {}}' --from <key_name> --chain-id <chain_id>
```

```bash
desmos query wasm contract-state smart <contract_address> '' --chain-id <chai_id>
```
