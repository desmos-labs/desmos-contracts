#!/usr/bin/env node

/* eslint-disable @typescript-eslint/naming-convention */
const {GasPrice} = require("@cosmjs/launchpad");
const { Random } = require("@cosmjs/crypto");
const { stringToPath } = require("@cosmjs/crypto")
const { Bech32 } = require("@cosmjs/encoding");
const { coins } = require("@cosmjs/launchpad");
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { assertIsBroadcastTxSuccess, SigningStargateClient } = require("@cosmjs/stargate");

const rpcUrl = "http://localhost:26657"
const prefix = "desmos"
const tokenDenom = "stake"
const hdPath = stringToPath("m/44'/852'/0'/0/0")
const faucet = {
  mnemonic: "jaguar harbor escape nasty charge intact common grow minute riot patient office quarter suffer solid light post brush snack decorate option arrange deer dinosaur",
  address: "desmos1r96w2wtk53e4k6mfldpjya6cgdl2ey026eqnvg"
}

async function main() {
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(faucet.mnemonic, hdPath, prefix);
  const gas = GasPrice.fromString("2000stake")
  const client = await SigningStargateClient.connectWithSigner(rpcUrl, wallet, {
    gasPrice: gas
  });
  const recipient = Bech32.encode(prefix, Random.getBytes(20));
  const amount = coins(226644, tokenDenom);
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
