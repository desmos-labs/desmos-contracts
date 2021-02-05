#!/usr/bin/env node

/* eslint-disable @typescript-eslint/naming-convention */
const {GasPrice} = require("@cosmjs/launchpad");
const { stringToPath } = require("@cosmjs/crypto")
const { coins } = require("@cosmjs/launchpad");
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { assertIsBroadcastTxSuccess, SigningStargateClient } = require("@cosmjs/stargate");

const rpcUrl = "http://localhost:26657"
const prefix = "desmos"
const tokenDenom = "stake"
const hdPath = stringToPath("m/44'/852'/0'/0/0")

// Genesis address
const faucet = {
  mnemonic: "jaguar harbor escape nasty charge intact common grow minute riot patient office quarter suffer solid light post brush snack decorate option arrange deer dinosaur",
  address: "desmos1r96w2wtk53e4k6mfldpjya6cgdl2ey026eqnvg"
}

// Your address
const deployer = {
  mnemonic: "moment lady correct fortune ask churn car organ faculty escape salt team vendor solar beach vicious suffer reopen curve utility grief spoil pave plastic",
  address: "desmos1p9k9h8z5hs2mhgkvfqykhg6654d7dcxr736v6f"
}

async function main() {
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(faucet.mnemonic, hdPath, prefix);
  const gas = GasPrice.fromString("2000stake")
  const client = await SigningStargateClient.connectWithSigner(rpcUrl, wallet, {gasPrice: gas});
  const amount = coins(1400000000, tokenDenom);
  const memo = "Sending tokens from faucet";
  const sendResult = await client.sendTokens(faucet.address, deployer.address, amount, memo);
  assertIsBroadcastTxSuccess(sendResult);
}

main().then(
  () => process.exit(0),
  (error) => {
    console.error(error);
    process.exit(1);
  },
);
