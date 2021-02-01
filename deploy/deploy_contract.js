#!/usr/bin/env node

/* eslint-disable @typescript-eslint/naming-convention */
const { DirectSecp256k1HdWallet } = require("@cosmjs/proto-signing");
const { SigningCosmWasmClient } = require("@cosmjs/cosmwasm-stargate");
const fs = require("fs");

const endpoint = "http://localhost:26657"
const deployer = {
  mnemonic: "battle call once stool three mammal hybrid list sign field athlete amateur cinnamon eagle shell erupt voyage hero assist maple matrix maximum able barrel",
  address0: "desmos1k8u92hx3k33a5vgppkyzq6m4frxx7ewnlkyjrh",
};

const codeMeta = {
  source: "",
  builder: "cosmwasm/rust-optimizer:0.10.7",
};

const prefix = "desmos"

async function main() {
  const wallet = await DirectSecp256k1HdWallet.fromMnemonic(deployer.mnemonic, undefined, prefix);
  const client = await SigningCosmWasmClient.connectWithSigner(endpoint, wallet);

  const wasm = fs.readFileSync(".wasm");
  const uploadReceipt = await client.upload(deployer.address0, wasm, codeMeta, "Upload CW1 subkeys contract");
  console.info(`Upload succeeded. Receipt: ${JSON.stringify(uploadReceipt)}`);

  const initMsg = {
    admins: [alice.address0],
    mutable: true,
  };
  const label = "Subkey test";
  const { contractAddress } = await client.instantiate(deployer.address0, uploadReceipt.codeId, initMsg, label, {
    memo: `Create a CW1 instance for ${deployer.address0}`,
    admin: alice.address0,
  });
  await client.sendTokens(deployer.address0, contractAddress, [
    {
      amount: "1000",
      denom: "ucosm",
    },
  ]);
  console.info(`Contract instantiated for ${deployer.address0} subkey at ${contractAddress}`);
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
