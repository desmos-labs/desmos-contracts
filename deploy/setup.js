#!/usr/bin/env node

/* eslint-disable @typescript-eslint/naming-convention */
const { Random } = require("@cosmjs/crypto");
const { Bech32 } = require("@cosmjs/encoding");
const { coins } = require("@cosmjs/launchpad");
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { assertIsBroadcastTxSuccess, SigningStargateClient } = require("@cosmjs/stargate");

const rpcUrl = "http://localhost:26657"
const prefix = "desmos"
const faucet = {
  mnemonic: "middle flavor spoil chicken bright morning bus entry crane clean split dust palace gown loud march popular desert true express work neither term remove",
  address: "desmos1sp48xxyz4cqzs8ka3eqlavmpljtnu5wve6j3dg"
}

async function main() {
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(faucet.mnemonic, undefined, prefix);
  const client = await SigningStargateClient.connectWithSigner(rpcUrl, wallet);
  const recipient = Bech32.encode(prefix, Random.getBytes(20));
  const amount = coins(226644, "ucosm");
  const memo = "Ensure chain has my pubkey";
  const sendResult = await client.sendTokens(faucet.address, recipient, amount, memo);
  assertIsBroadcastTxSuccess(sendResult);
}

main().then(
  () => process.exit(0),
  (error) => {
    console.error(error);
    process.exit(1);
  },
);
