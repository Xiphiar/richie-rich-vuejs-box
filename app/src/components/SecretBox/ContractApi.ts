import { onMounted, ref, reactive, computed } from 'vue'
import { Wallet, SecretNetworkClient, Permission, Permit } from "secretjs"
import type { 
  UserInputs, FormRow, FormRowInput,
  CustomPermission,
  QueryResult, AllInfoResult, AmIRichestResult,
} from './Types'

// Get environment variables from .env
const localSecretUrl: string = import.meta.env.VITE_LOCALSECRET_GRPC
const secretBoxCode: number = import.meta.env.VITE_SECRET_BOX_CODE
const secretBoxHash: string = import.meta.env.VITE_SECRET_BOX_HASH
const secretBoxAddress: string = import.meta.env.VITE_SECRET_BOX_ADDRESS

console.log(`local gRPC = ${localSecretUrl}`)
console.log(`code id = ${secretBoxCode}`)
console.log(`contract hash = ${secretBoxHash}`)
console.log(`contract address = ${secretBoxAddress}`)

// const addrs: string[] = [
//   import.meta.env.ALICE_ADDRESS,
//   import.meta.env.BOB_ADDRESS,
//   import.meta.env.CAROL_ADDRESS ,
// ]

const mnemonics = [
  "grant rice replace explain federal release fix clever romance raise often wild taxi quarter soccer fiber love must tape steak together observe swap guitar",
  "jelly shadow frog dirt dragon use armed praise universe win jungle close inmate rain oil canvas beauty pioneer chef soccer icon dizzy thunder meadow",
  "chair love bleak wonder skirt permit say assist aunt credit roast size obtain minute throw sand usual age smart exact enough room shadow charge",
  "word twist toast cloth movie predict advance crumble escape whale sail such angry muffin balcony keen move employ cook valve hurt glimpse breeze brick",
]
// secret1ap26qrlp8mcq2pg6r47w43l0y8zkqm8a450s03
// secret1fc3fzy78ttp0lwuujw7e52rhspxn8uj52zfyne
// secret1ajz54hz8azwuy34qwy9fkjnfcrvf0dzswy0lqq
// secret1ldjxljw7v4vk6zhyduywh04hpj0jdwxsmrlatf


export const initSecretjsClient = async () => {
  let accounts: SecretNetworkClient[] = [];
  for (const mnemonic of mnemonics) {
    const wallet = new Wallet(mnemonic)
    let secretjs = await SecretNetworkClient.create({
      //grpcWebUrl: "http://localhost:9091",
      grpcWebUrl: localSecretUrl,
      chainId: "secretdev-1",
      wallet: wallet,
      walletAddress: wallet.address,
    })
    accounts.push(secretjs)
    console.log(`Created client for wallet address: ${secretjs.address}`)
  } 
  return accounts
}


// Smart contract interface -------------------------------

export const handleSubmitNetworth = async (
  secretjs: SecretNetworkClient,
  networth: string
) => {
  const tx = await secretjs.tx.compute.executeContract(
  {
    sender: secretjs.address,
    contractAddress: secretBoxAddress,
    codeHash: secretBoxHash,
    msg: {
      submit_net_worth: { networth },
    },
  },
  {
    gasLimit: 1_000_000,
  })

  console.log("Submitted networth")
  // count.value = await queryCounter()
}

export const handleSetViewingKey = async (
  secretjs: SecretNetworkClient,
  key: string,
) => {
  const tx = await secretjs.tx.compute.executeContract(
  {
    sender: secretjs.address,
    contractAddress: secretBoxAddress,
    codeHash: secretBoxHash,
    msg: {
      set_viewing_key: { key },
    },
  },
  {
    gasLimit: 1_000_000,
  })

  console.log("Viewing key set")
  // count.value = await queryCounter()
}

export const handleQueryAllInfo = async (
  secretjs: SecretNetworkClient,
  addr: string,
  key: string,
) => {
  const response = (await secretjs.query.compute.queryContract({
    contractAddress: secretBoxAddress,
    codeHash: secretBoxHash,
    query: { all_info: {
      addr,
      key,
    } },
  })) as AllInfoResult

  // if ('err"' in response) {
  //   throw new Error(
  //     `Query failed with the following err: ${JSON.stringify(response)}`
  //   )
  // }

  return response
}

export const handleQueryAmIRichest = async (
  secretjs: SecretNetworkClient,
  addr: string,
  key: string,
) => { 
  const response = (await secretjs.query.compute.queryContract({
    contractAddress: secretBoxAddress,
    codeHash: secretBoxHash,
    query: { am_i_richest: {
      addr,
      key,
    } },
  })) as AmIRichestResult

  // if ('err"' in response) {
  //   throw new Error(
  //     `Query failed with the following err: ${JSON.stringify(response)}`
  //   )
  // }

  return response
}

export async function handleQueryAllInfoWithPermit(
  secretjs: SecretNetworkClient,
  permit: Permit,
) {
  // const permit = await handleGeneratePermit(secretjs, ["all_info"]);

  const msg = { with_permit: {
    permit,
    query: { all_info: { }}
  }};

  const response = (await secretjs.query.compute.queryContract({
    contractAddress: secretBoxAddress,
    codeHash: secretBoxHash,
    query: msg,
  })) as AllInfoResult

  return response;
}

export async function handleQueryAmIRichestWithPermit(
  secretjs: SecretNetworkClient,
  permit: Permit,
) {
  // const permit = await handleGeneratePermit(secretjs, ["am_i_richest"]);

  const msg = { with_permit: {
    permit,
    query: { am_i_richest: { }}
  }};

  const response = (await secretjs.query.compute.queryContract({
    contractAddress: secretBoxAddress,
    codeHash: secretBoxHash,
    query: msg,
  })) as AmIRichestResult

  return response;
}

export async function handleGeneratePermit(
  account: SecretNetworkClient,
  permitName: string,
  permissions: CustomPermission[],
): Promise<Permit> {
  const permit = await account.utils.accessControl.permit.sign(
    account.address,
    "secret-4",
    permitName,
    [secretBoxAddress],
    // @ts-ignore
    permissions, // ["owner"],
    false,
  );
  return permit;
}
