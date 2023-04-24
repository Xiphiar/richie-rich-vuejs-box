<script setup lang="ts">
import { onMounted, ref, reactive, computed } from 'vue'
import { SecretNetworkClient, Permit } from "secretjs"
import { 
  handleGeneratePermit, handleQueryAllInfo, handleQueryAllInfoWithPermit,
  handleQueryAmIRichest, handleQueryAmIRichestWithPermit, handleSetViewingKey, handleSubmitNetworth,
  initSecretjsClient,
} from "./ContractApi"
import type { 
  UserInputs, FormRow,
  QueryResult,
} from './Types'


const showApp = ref(true)
let accounts: SecretNetworkClient[] = reactive([])

onMounted(async () => {
  window.addEventListener('scroll', handleScroll)

  // To create signer secret.js clients
  console.log("Initializing Secret.js client ...")
  accounts = await initSecretjsClient(accounts)
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

/** The form inputs and buttons for contract execute functions
 * and generate permit (ie: those that require signing with private key)
 */
const formExecRows = reactive<FormRow[]>([{
  headerText: "Execute functions",
  inputs: [{
      field: 'networth',
      placeholderText: "1000",
  }],
  buttons: [{
      onFunction: onSubmitNetworth, 
      buttonText: "Submit Networth",
  }]
  },
  {
    headerText: "",
    inputs: [{
        field: 'viewingKey',
        placeholderText: "my_viewing_key",
    }],
    buttons: [{
        onFunction: onSetViewingKey,  
        buttonText: "Set viewing key",
    }],
  },
  {
    headerText: "Generate permit",
    inputs: [{
        field: 'permitName',
        placeholderText: "Permit name",
      },
      {
        field: 'permission',
        placeholderText: '"all_info" or "am_i_richest"',
    }],
    buttons: [{
        onFunction: onGeneratePermit,  
        buttonText: "Sign permit",
    }]
  },
])

/** The form inputs and buttons for contract query functions */
const formQueryRows = reactive<FormRow[]>([{
    headerText: "Viewing key queries",
    inputs: [{
        field: 'queryAddr',
        placeholderText: "addr",
      },
      {
        field: 'queryKey',
        placeholderText: "viewing key",
    }],
    buttons: [{
        onFunction: onQueryAllInfo,  
        buttonText: "VK Query: All Info",
      },
      {
        onFunction: onQueryAmIRichest,  
        buttonText: "VK Query: Am I Richest",
    }],
  },
  {
  headerText: "Permit queries",
    inputs: [{
        field: 'permitId',
        placeholderText: "enter permit id number",
    }],
    buttons: [{
        onFunction: onQueryAllInfoWithPermit,  
        buttonText: "Permit Query: All Info",
      },
      {
        onFunction: onQueryAmIRichestWithPermit,  
        buttonText: "Permit Query: Am I Richest",
    }],
  },
])

let inputs: UserInputs = reactive({
  networth: {},
  viewingKey: {},
  permitName: {},
  permission: {},
  queryAddr: '',
  queryKey: '',
  permitId: 0,
})


let contractResponse = reactive({
  query: {
    richest: false,
    networth: '',
  } as QueryResult,

  permits: [{
    params: {
        permit_name: 'Unsigned permit',
        allowed_tokens: [''],
        chain_id: '',
        permissions: ['allowance'],  // to get around the secretjs fixed Permission type
    },
    signature: {
      pub_key: {
        type: '',
        value: '',
      },
      signature: '<blank>',
    },
  }] as Permit[],
})
// init contractResponse
contractResponse.query=''

async function onSubmitNetworth(acc: SecretNetworkClient) {
  // const acc = accounts[0]
  await handleSubmitNetworth(acc, inputs.networth[acc.address])
  inputs.networth[acc.address] = ''
}

async function onSetViewingKey(acc: SecretNetworkClient) {
  // const acc = accounts[0]
  await handleSetViewingKey(acc, inputs.viewingKey[acc.address])
  inputs.viewingKey[acc.address] = ''
}

async function onGeneratePermit(acc: SecretNetworkClient): Promise<void> {
  // const acc = accounts[0]
  const res = await handleGeneratePermit(acc, inputs.permitName[acc.address], [inputs.permission[acc.address]])
  inputs.permitName[acc.address] = ''
  inputs.permission[acc.address] = ''
  contractResponse.permits.push(res)
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

async function onQueryAllInfoWithPermit() {
  const acc = accounts[0]
  const res = await handleQueryAllInfoWithPermit(acc, contractResponse.permits[inputs.permitId])
  contractResponse.query = res
}

async function onQueryAmIRichestWithPermit() {
  const acc = accounts[0]
  const res = await handleQueryAmIRichestWithPermit(acc, contractResponse.permits[inputs.permitId])
  contractResponse.query = res
}


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
    <h1 class="text-xl font-bold mt-10">Account-level messages</h1>
    <div v-for="account in accounts">
      <h2 class="mt-4">Account: {{ account.address }}</h2>
      <div v-for="row in formExecRows" class="w-full justify-items-center mt-2 ml-4 mb-4">
        <p class="italic mb-2">{{ row.headerText }}</p>
        <form @submit.prevent=row.buttons[0].onFunction(account)>
          <div class="grid grid-cols-3 grid-flow-col h-full leading-none">
            <input v-for="input in row.inputs" 
              :class='row.inputs.length !== 2 
                ? "col-span-2 rounded-md ml-4 outline" 
                : "rounded-md ml-4 outline"'
              :placeholder=input.placeholderText
              v-model="//@ts-ignore implicit any for second field
                inputs[input.field][account.address]"
            >
            <button class="w-4/5 bg-box-yellow self-center px-1 py-1 rounded-md ml-4"> 
              {{ row.buttons[0].buttonText }} 
            </button>
          </div>
        </form>
      </div>
      <hr class="border-gray-300">
    </div>
      
    <h1 class="text-xl font-bold mt-5">Queries</h1>
    <div v-for="qrow in formQueryRows" class="w-full justify-items-center mt-4 ml-4 mb-4">
      <p class="italic mb-2">{{ qrow.headerText }}</p>
      <div class="grid grid-cols-3 grid-flow-col h-full leading-none">
        <input v-for="input in qrow.inputs" 
          class="rounded-md ml-4 outline"
          :placeholder=input.placeholderText
          v-model="inputs[input.field]"
        >
      </div>
      <div class="grid grid-cols-3 mt-2 mb-2">
        <button v-for="button in qrow.buttons"
          @click=button.onFunction
          class="w-4/5 col-start-3 bg-box-yellow self-center px-1 py-1 rounded-md ml-4 mt-1 mb-1"> 
          {{ button.buttonText }} 
        </button>
      </div>
    </div>

    <p class="font-semibold">Permits
      <span class="text-sm font-normal">(These are stored on the front-end client, not on-chain):</span>
    </p>
    <ol start="0" class="list-decimal list-inside text-left text-xs mb-6"> 
      <li v-for="(permit, id) in contractResponse.permits">
        Permit name: {{ permit.params.permit_name }}; Signature: {{ permit.signature.signature }}
      </li>
    </ol>

    <div class="rounded-md outline text-center ml-3 mt-10 mb-10 py-3 bg-yellow-50">
      <p class="font-semibold ">Query response:</p>
      <p>{{ contractResponse.query }}</p>
    </div>
    
    <!-- <hr>

    <div class="grid w-full justify-items-center mb-16">
      <button @click="" class="font-semibold text-[#4E4B66] dark:text-white border-2 border-[#4E4B66] dark:border-white px-8 py-3 rounded-md">New round</button>
    </div> -->
  </div>
</template>
