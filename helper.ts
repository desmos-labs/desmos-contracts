/*
 * This is a set of helpers meant for use with @cosmjs/cli
 * With these you can easily use the desmos contracts without worrying about forming messages and parsing queries.
 *
 * Usage: npx @cosmjs/cli --init https://github.com/bragaz/wasm-test-contract/tree/master/helper.ts
 *
 * Create a client:
 *   const client = await useOptions(defaultOptions).setup(password);
 *   await client.getAccount()
 *
 * Get the mnemonic:
 *   await useOptions(defaultOptions).recoverMnemonic(password)
 *
 * If you want to use this code inside an app, you will need several imports from https://github.com/CosmWasm/cosmjs
 */

interface Options {
  readonly httpUrl: string
  readonly networkId: string
  readonly feeToken: string
  readonly gasPrice: number
  readonly bech32prefix: string
}

const defaultOptions: Options = {
  httpUrl: 'https://lcd.desmos.com',
  networkId: 'morpheus',
  feeToken: 'udaric',
  gasPrice: 0.01,
  bech32prefix: 'desmos',
}

const buildFeeTable = (feeToken: string, gasPrice: number): FeeTable => {
  const stdFee = (gas: number, denom: string, price: number) => {
    const amount = Math.floor(gas * price)
    return {
      amount: [{ amount: amount.toString(), denom: denom }],
      gas: gas.toString(),
    }
  }

  return {
    upload: stdFee(1500000, feeToken, gasPrice),
    init: stdFee(500000, feeToken, gasPrice),
    migrate: stdFee(500000, feeToken, gasPrice),
    exec: stdFee(200000, feeToken, gasPrice),
    send: stdFee(80000, feeToken, gasPrice),
    changeAdmin: stdFee(80000, feeToken, gasPrice),
  }
}

const buildWallet = (mnemonic: string): Promise<Secp256k1Wallet> => {
  return Secp256k1Wallet.fromMnemonic(mnemonic, makeCosmoshubPath(0), defaultOptions.bech32prefix);
}

const randomAddress = async (): Promise<string> => {
  const mnemonic = Bip39.encode(Random.getBytes(16)).toString()
  return mnemonicToAddress(mnemonic)
}

const mnemonicToAddress = async (
  mnemonic: string
): Promise<string> => {
  const wallet = await buildWallet(mnemonic);
  const [{ address }] = await wallet.getAccounts()
  return address
}

const getAttibute = (
  logs: readonly logs.Log[],
  key: string
): string | undefined =>
  logs[0].events[0].attributes.find((x) => x.key == key)?.value

const hitFaucet = async (
  faucetUrl: string,
  address: string,
  denom: string
): Promise<void> => {
  const r = await axios.post(faucetUrl, { denom, address })
  console.log(r.status)
  console.log(r.data)
}

const connect = async (
  mnemonic: string,
  opts: Partial<Options>
): Promise<{
  client: SigningCosmWasmClient
  address: string
}> => {
  const options: Options = { ...defaultOptions, ...opts }
  const feeTable = buildFeeTable(options.feeToken, options.gasPrice)
  const wallet = await buildWallet(mnemonic)
  const [{ address }] = await wallet.getAccounts()

  const client = new SigningCosmWasmClient(
    options.httpUrl,
    address,
    wallet,
    feeTable
  )
  return { client, address }
}

interface OptionalData {
  readonly key: string,
  readonly value: string,
}

interface Attachment {
  readonly uri: string,
  readonly mime_type: string,
  readonly tags: string[],
}

interface PollAnswer {
  readonly id: string,
  readonly text: string,
}

interface PollData {
  readonly question: string,
  readonly provided_answers: PollAnswer[],
  readonly end_date: string,
  readonly allows_multiple_answers: boolean,
  readonly allows_answer_edits: boolean,
}

interface Post {
  readonly post_id: string,
  readonly parent_id: string,
  readonly message: string,
  readonly created: string,
  readonly last_edited: string,
  readonly allows_comments: boolean,
  readonly subspace: string,
  readonly optional_data: OptionalData[],
  readonly attachments: Attachment[],
  readonly poll_data: PollData[],
  readonly creator: string,
}

interface PostQueryResponse {
  readonly posts: Post[]
}

interface InitMsg {
  readonly reports_limit: number,
}

interface PostsFilterContractInstance {
  readonly contractAddress: string,

  // queries
  getFilteredPosts: (reports_limit: number) => Promise<PostQueryResponse>

  // actions
  editReportsLimit: (reports_limit: number) => Promise<string>
}

interface Contract {
  // upload a code blob and returns codeId
  upload: () => Promise<number>

  // instantiates a filterPosts contract
  // codeId must come from a previous deploy
  // label is the public name of the contract in listing
  // if you set admin, you can run migration on this contract (likely client.senderAddress)
  instantiate: (codeId: number, initMsg: InitMsg, label: string, admin?: string) => Promise<PostsFilterContractInstance>

  use: (contractAddress: string) => PostsFilterContractInstance
}

const filterPostsContract = (client: SigningCosmWasmClient, metaSource: string, builderSource: string, contractSource: string): Contract => {
  const use = (contractAddress: string): PostsFilterContractInstance => {
    const getFilteredPosts = async (reports_limit: number): Promise<PostQueryResponse> => {
      return await client.queryContractSmart(contractAddress, {get_filtered_posts: {reports_limit}});
    }

    const editReportsLimit = async (reports_limit: number): Promise<string> => {
      const result = await client.execute(contractAddress, {edit_reports_limit: {reports_limit}});
      return result.transactionHash;
    }

    return {
      contractAddress,
      getFilteredPosts,
      editReportsLimit,
    };
  }

  const downloadWasm = async (url: string): Promise<Uint8Array> => {
    const r = await axios.get(url, { responseType: 'arraybuffer' })
    if (r.status !== 200) {
      throw new Error(`Download error: ${r.status}`)
    }
    return r.data
  }

  const upload = async (): Promise<number> => {
    const meta = {
      source: metaSource,
      builder: builderSource
    };
    const wasm = await downloadWasm(contractSource);
    const result = await client.upload(wasm, meta);
    return result.codeId;
  }

  const instantiate = async (codeId: number, initMsg: InitMsg, label: string, admin?: string): Promise<PostsFilterContractInstance> => {
    const result = await client.instantiate(codeId, initMsg, label, { memo: `Init ${label}`, admin});
    return use(result.contractAddress);
  }

  return { upload, instantiate, use };
}

// Example:
// const mnemonic = "<mnemonic phrase>"
// const result = connect(mnemonic, defaultOptions)
// const metaSourcePath = "https://github.com/bragaz/wasm-test-contract/tree/v0.2.2"
// const optimizerPath = "cosmwasm/rust-optimizer:0.10.7"
// const sourceUrl = "https://github.com/bragaz/wasm-test-contract/releases/download/v0.2.2/my_first_contract.wasm"
// const resolvedResult = await result
// const faucetUrl = "https://desmos.faucet.com
// hitFaucet(defaultFaucetUrl, resolvedResult.address, defaultOptions.feeToken)
// const factory = filterPostsContract(resolvedResult.client, metaSourcePath, optimizerPath, sourceUrl)
// const codeId = await factory.upload();
// const contract = await factory.instantiate(codeId, reports_limit: 5)
// contract.contractAddress -> 'desmos1w8efgymkdqafech2c0y40hgvxa23tmmgsmuz66'
//
// OR
//
// const filterPostsContract = factory.use(contract.contractAddress)


