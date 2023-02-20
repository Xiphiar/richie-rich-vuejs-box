<script setup lang="ts">
import { onMounted, ref, reactive, computed } from 'vue'
import { Wallet, SecretNetworkClient, Permission, Permit } from "secretjs"
import { 
  handleGeneratePermit, handleQueryAllInfo, handleQueryAllInfoWithPermit,
  handleQueryAmIRichest, handleQueryAmIRichestWithPermit, handleSetViewingKey, handleSubmitNetworth,
  initSecretjsClient,
} from "./ContractApi"
import type { 
  UserInputs, FormRow, FormRowInput,
  CustomPermission, 
  QueryResult, AllInfoResult, AmIRichestResult,
} from './Types'


const showApp = ref(true)
let accounts: SecretNetworkClient[] = []

onMounted(async () => {
  window.addEventListener('scroll', handleScroll)

  // To create signer secret.js clients
  console.log("Initializing Secret.js client ...")
  accounts = await initSecretjsClient()
})

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
  headerText: "Execute functions",
    onFunction: onSubmitNetworth,  
    inputs: [{
      field: 'networth',
      placeholderText: "1000",
    }],
    buttonText: "Submit Networth",
  },
  {
    headerText: "",
    onFunction: onSetViewingKey,  
    inputs: [{
      field: 'viewingKey',
      placeholderText: "my_viewing_key",
    }],
    buttonText: "Set viewing key",
  },
  {
    headerText: "Generate permit",
    onFunction: onGeneratePermit,  
    inputs: [{
      field: 'permitName',
      placeholderText: "Permit name",
    },
    {
      field: 'permission',
      placeholderText: '"all_info" or "am_i_richest"',
    }],
    buttonText: "Sign permit",
  },
  {
    headerText: "Queries",
    onFunction: onQueryAllInfo,  
    inputs: [{
      field: 'queryAddr',
      placeholderText: "addr",
    },
    {
      field: 'queryKey',
      placeholderText: "viewing key",
    }],
    buttonText: "Query: All Info",
  },
  {
    headerText: "",
    onFunction: onQueryAmIRichest,  
    inputs: [{
      field: 'queryAddr',
      placeholderText: "addr",
    },
    {
      field: 'queryKey',
      placeholderText: "viewing key",
    }],
    buttonText: "Query: Am I Richest",
  },
])


let inputs: UserInputs = reactive({
  networth: '',
  viewingKey: '',
  permitName: '',
  permission: '',
  queryAddr: '',
  queryKey: '',
})


let contractResponse = reactive({
  query: {
    richest: false,
    networth: '',
  } as QueryResult,

  permit: {
    params: {
        permit_name: '',
        allowed_tokens: [''],
        chain_id: '',
        permissions: ['allowance'],  // to get around the secretjs fixed Permission type
    },
    signature: {
      pub_key: {
        type: '',
        value: '',
      },
      signature: '',
    },
  } as Permit,
})
// init contractResponse
contractResponse.query=''

async function onSubmitNetworth() {
  const acc = accounts[0]
  await handleSubmitNetworth(acc, inputs.networth)
  inputs.networth = ''
}

async function onSetViewingKey() {
  const acc = accounts[0]
  await handleSetViewingKey(acc, inputs.viewingKey)
  inputs.viewingKey = ''
}

async function onGeneratePermit(): Promise<void> {
  const acc = accounts[0]
  const res = await handleGeneratePermit(acc, inputs.permitName, [inputs.permission])
  inputs.permitName = ''
  inputs.permission = ''
  contractResponse.permit = res
}

async function onQueryAllInfo() {
  const acc = accounts[0]
  const res = await handleQueryAllInfo(acc, inputs.queryAddr, inputs.queryKey)
  contractResponse.query = res
}

async function onQueryAmIRichest() {
  const acc = accounts[0]
  const res = await handleQueryAmIRichest(acc, inputs.queryAddr, inputs.queryKey)
  contractResponse.query = res
}

// if not connected to smart contract: ------------

// let submitted = {

// }


</script>

<template>
  <div class="grid items-center grid-cols-2">
    <div class="flex pb-2 self-center">
      <img src="../../assets/title_star.svg" alt="Richie Rich app">
      <h2 class="ml-2 text-2xl font-medium tracking-widest text-[#200E32] dark:text-white"></h2>
    </div>

    <img @click="showApp = false" class="justify-self-end cursor-pointer" v-if="showApp && isLight()" src="../../assets/up.svg" alt="Hide application">
    <img @click="showApp = true" class="justify-self-end cursor-pointer" v-if="!showApp && isLight()" src="../../assets/down.svg" alt="Show application">

    <img @click="showApp = false" class="justify-self-end cursor-pointer" v-if="showApp && isDark()" src="../../assets/up_white.svg" alt="Hide application">
    <img @click="showApp = true" class="justify-self-end cursor-pointer" v-if="!showApp && isDark()" src="../../assets/down_white.svg" alt="Show application">
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
      <strong> Debug:-- </strong>
      vmodel: {{ formrows[0].inputs[0].field }} <br>
      networth input: {{inputs.networth}}  
    </p>
    
    <p class="text-center mt-6 mb-6">
      <span class="font-semibold">Permit: </span>
      {{ contractResponse.permit }}
    </p>

    <p class="text-center mt-6 mb-6">
      <span class="font-semibold">Query response: </span>{{ contractResponse.query }}
    </p>
    

    <div class="grid w-full justify-items-center mb-16">
      <button @click="" class="font-semibold text-[#4E4B66] dark:text-white border-2 border-[#4E4B66] dark:border-white px-8 py-3 rounded-md">New round</button>
    </div>
  </div>
</template>
