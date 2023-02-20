<script setup lang="ts">
import { onMounted, ref, reactive, computed } from 'vue'
import { Wallet, SecretNetworkClient, Permission, Permit } from "secretjs"
import { AminoWallet } from 'secretjs/dist/wallet_amino';

// Secret.js Client
let secretjs: SecretNetworkClient

const wallet = new Wallet(
  "grant rice replace explain federal release fix clever romance raise often wild taxi quarter soccer fiber love must tape steak together observe swap guitar"
)

// Get environment variables from .env
const localSecretUrl: string = import.meta.env.VITE_LOCALSECRET_GRPC
const secretBoxCode: number = import.meta.env.VITE_SECRET_BOX_CODE
const secretBoxHash: string = import.meta.env.VITE_SECRET_BOX_HASH
const secretBoxAddress: string = import.meta.env.VITE_SECRET_BOX_ADDRESS

console.log(`local gRPC = ${localSecretUrl}`)
console.log(`code id = ${secretBoxCode}`)
console.log(`contract hash = ${secretBoxHash}`)
console.log(`contract address = ${secretBoxAddress}`)

const count = ref(0)
const showApp = ref(true)

onMounted(async () => {
  window.addEventListener('scroll', handleScroll)

  // To create a signer secret.js client, also pass in a wallet
  console.log("Initializing Secret.js client ...")
  secretjs = await SecretNetworkClient.create({
    //grpcWebUrl: "http://localhost:9091",
    grpcWebUrl: localSecretUrl,
    chainId: "secretdev-1",
    wallet: wallet,
    walletAddress: wallet.address,
  })

  console.log(`Created client for wallet address: ${wallet.address}`)

  // count.value = await queryCounter()
})


// Smart contract interface -------------------------------

const handleSubmitNetworth = async (
  networth: string
) => {
  const tx = await secretjs.tx.compute.executeContract(
  {
    sender: wallet.address,
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

const handleSetViewingKey = async (
  key: string,
) => {
  const tx = await secretjs.tx.compute.executeContract(
  {
    sender: wallet.address,
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

const handleQueryAllInfo = async (
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
  })) as AllInfoResponse | string;

  // if ('err"' in response) {
  //   throw new Error(
  //     `Query failed with the following err: ${JSON.stringify(response)}`
  //   )
  // }

  return response
}

const handleQueryAmIRichest = async (
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
  })) as AmIRichestResponse | string;

  // if ('err"' in response) {
  //   throw new Error(
  //     `Query failed with the following err: ${JSON.stringify(response)}`
  //   )
  // }

  return response
}

async function handleQueryAllInfoWithPermit(
  account: Account,
) {
  const permit = await handleGeneratePermit(account, ["all_info"]);

  const msg = { with_permit: {
    permit,
    query: { all_info: { }}
  }};

  const response = (await secretjs.query.compute.queryContract({
    contractAddress: secretBoxAddress,
    codeHash: secretBoxHash,
    query: msg,
  })) as AllInfoResponse;

  return response;
}

async function handleQueryAmIRichestWithPermit(
  account: Account,
) {
  const permit = await handleGeneratePermit(account, ["am_i_richest"]);

  const msg = { with_permit: {
    permit,
    query: { am_i_richest: { }}
  }};

  const response = (await secretjs.query.compute.queryContract({
    contractAddress: secretBoxAddress,
    codeHash: secretBoxHash,
    query: msg,
  })) as AmIRichestResponse;

  return response;
}


// TS types and helper functions

async function handleGeneratePermit(
  account: Account,
  permits: CustomPermission[],
): Promise<Permit> {
  const { secretjs } = account;
  const permit = await secretjs.utils.accessControl.permit.sign(
    account.address,
    "secret-4",
    "permitname",
    [secretBoxAddress],
    // @ts-ignore
    permits, // ["owner"],
    false,
  );
  return permit;
}

type AllInfoResponse = { 
  richest: boolean,
  networth: number, 
}

type AmIRichestResponse = { 
  richest: boolean,
}

type CustomPermission = "all_info" | "am_i_richest" 

type Account = {
  address: string;
  mnemonic: string;
  walletAmino: AminoWallet;
  walletProto: Wallet;
  secretjs: SecretNetworkClient;
};


// --------------------------------------------------------

const props = defineProps<{
  title: string
}>()

function isLight() {
  return localStorage.getItem('theme') === 'light'
}

function isDark() {
  return localStorage.getItem('theme') === 'dark'
}

function handleScroll() {
  if (showApp) {
    // To collapse App when user scrolls:
    // showApp.value = false
  }
}

// -------------------

const formrows = reactive<FormRow[]>([{
  headerText: "hey",
    onFunction: onSubmitNetworth,  
    inputs: [{
      field: 'networth',
      placeholderText: "1000",
    }],
    buttonText: "Submit Networth",
  },
  {
    headerText: "hey",
    onFunction: onSetViewingKey,  
    inputs: [{
      field: 'viewingKey',
      placeholderText: "my_viewing_key",
    }],
    buttonText: "Set viewing key",
  },
  {
    headerText: "hey",
    onFunction: onQueryAllInfo,  
    inputs: [{
      field: 'queryAllInfoAddr',
      placeholderText: "addr",
    },
    {
      field: 'queryAllInfoKey',
      placeholderText: "viewing key",
    }],
    buttonText: "Query: All Info",
  },
  {
    headerText: "",
    onFunction: onQueryAllInfo,  
    inputs: [{
      field: 'queryAmIRichestAddr',
      placeholderText: "addr",
    },
    {
      field: 'queryAmIRichestKey',
      placeholderText: "viewing key",
    }],
    buttonText: "Query: Am I Richest",
  },
])

type UserInputs = {
  networth: string,
  viewingKey: string,
  queryAllInfoAddr: string,
  queryAllInfoKey: string,
  queryAmIRichestAddr: string,
  queryAmIRichestKey: string,
} 

type FormRow = {
  headerText: string
  onFunction: () => Promise<void>,
  inputs: FormRowInput[],
  buttonText: string,
}

type FormRowInput = {
  field: keyof UserInputs,
  placeholderText: string,
}

let inputs: UserInputs = reactive({
  networth: '',
  viewingKey: '',
  queryAllInfoAddr: '',
  queryAllInfoKey: '',
  queryAmIRichestAddr: '',
  queryAmIRichestKey: '',
})


let results = reactive({
  queryAllInfo: {
    richest: false,
    networth: '',
  } as unknown as AllInfoResponse | string,
})
// init result
results.queryAllInfo=''

async function onSubmitNetworth() {
  await handleSubmitNetworth(inputs.networth)
  inputs.networth = ''
}

async function onSetViewingKey() {
  await handleSetViewingKey(inputs.viewingKey)
  inputs.viewingKey = ''
}

async function onQueryAllInfo() {
  const res = await handleQueryAllInfo(inputs.queryAllInfoAddr, inputs.queryAllInfoKey)
  results.queryAllInfo = res
}

// if not connected to smart contract: ------------

// let submitted = {

// }

// ------------------------------------------------

</script>

<template>
  <div class="grid items-center grid-cols-2">
    <div class="flex pb-2 self-center">
      <img src="../assets/title_star.svg" alt="Richie Rich app">
      <h2 class="ml-2 text-2xl font-medium tracking-widest text-[#200E32] dark:text-white"></h2>
    </div>

    <img @click="showApp = false" class="justify-self-end cursor-pointer" v-if="showApp && isLight()" src="../assets/up.svg" alt="Hide application">
    <img @click="showApp = true" class="justify-self-end cursor-pointer" v-if="!showApp && isLight()" src="../assets/down.svg" alt="Show application">

    <img @click="showApp = false" class="justify-self-end cursor-pointer" v-if="showApp && isDark()" src="../assets/up_white.svg" alt="Hide application">
    <img @click="showApp = true" class="justify-self-end cursor-pointer" v-if="!showApp && isDark()" src="../assets/down_white.svg" alt="Show application">
  </div>

  <div v-if="showApp">
    <div v-for="row in formrows" class="w-full justify-items-center mt-4 mb-4">
      <p>{{ row.headerText }}</p>
      <form @submit.prevent=row.onFunction>
        <div class="grid grid-cols-3 grid-flow-col h-full text-xl leading-none">
          <input v-for="input in row.inputs" 
            :class='row.inputs.length !== 2 ? "col-span-2 rounded-md ml-4 outline" : "rounded-md ml-4 outline"'
            :placeholder=input.placeholderText
            v-model="inputs[input.field]"
          >
          <button class="w-4/5 bg-box-yellow self-center px-1 py-1 rounded-md ml-4"> 
            {{ row.buttonText }} 
          </button>
        </div>
      </form>
    </div>
    <p class="w-full">
      vmodel: {{ formrows[0].inputs[0].field }} <br>
      networth input: {{inputs.networth}}  
    </p>
    
    <p class="text-center mt-6 mb-6">
      <span class="font-semibold" >Query response: </span>{{ results.queryAllInfo }}
    </p>
    

    <div class="grid w-full justify-items-center mb-16">
      <button @click="" class="font-semibold text-[#4E4B66] dark:text-white border-2 border-[#4E4B66] dark:border-white px-8 py-3 rounded-md">New round</button>
    </div>
  </div>
</template>
