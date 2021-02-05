#!/usr/bin/env node

/* eslint-disable @typescript-eslint/naming-convention */
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { SigningCosmWasmClient } = require("@cosmjs/cosmwasm-stargate");
const fs = require("fs");
const {coins} = require("@cosmjs/launchpad");
const {stringToPath} = require("@cosmjs/crypto");
const {GasPrice} = require("@cosmjs/launchpad");

const rpcUrl = "http://localhost:26657"
const prefix = "desmos"
const tokenDenom = "stake"
const hdPath = stringToPath("m/44'/852'/0'/0/0")
const deployer = {
  mnemonic: "moment lady correct fortune ask churn car organ faculty escape salt team vendor solar beach vicious suffer reopen curve utility grief spoil pave plastic",
  address: "desmos1p9k9h8z5hs2mhgkvfqykhg6654d7dcxr736v6f"
}

const codeMeta = {
  source: "",
  builder: "cosmwasm/rust-optimizer:0.10.7",
};

async function main() {
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(deployer.mnemonic, hdPath, prefix);
  const gas = GasPrice.fromString("200stake")
  const options = {
    gasPrice: gas,
    gasLimits: {upload: 1500000}
  }
  const client = await SigningCosmWasmClient.connectWithSigner(rpcUrl, wallet, options);

  const wasm = fs.readFileSync("desmos_contracts.wasm");
  const uploadReceipt = await client.upload(deployer.address, wasm, codeMeta, "Upload desmos posts filter contract");
  console.info(`Upload succeeded. Receipt: ${JSON.stringify(uploadReceipt)}`);

  const initMsg = {
    reports_limit: 2
  };
  const label = "Desmos posts filter tests";
  const { contractAddress } = await client.instantiate(deployer.address, uploadReceipt.codeId, initMsg, label, {
    memo: `Create a posts filter contract instance for ${deployer.address}`,
    admin: deployer.address,
  });
  await client.sendTokens(deployer.address, contractAddress, [
    {
      amount: "1000",
      denom: tokenDenom,
    },
  ]);
  console.info(`Contract instantiated for ${deployer.address} at ${contractAddress}`);
}

main().then(
  () => {
    console.info("All done, let the coins flow.");
    process.exit(0);
  },
  (error) => {
    console.error(error);
    process.exit(1);
  },
);
