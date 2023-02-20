import { Wallet, SecretNetworkClient, Permission, Permit } from "secretjs"
import { AminoWallet } from 'secretjs/dist/wallet_amino';


// =========================================
// Contract interface
// =========================================
type AllInfoResponse = { 
  richest: boolean,
  networth: number, 
}

type AmIRichestResponse = { 
  richest: boolean,
}

export type AllInfoResult = AllInfoResponse | string
export type AmIRichestResult = AmIRichestResponse | string
export type QueryResult = AllInfoResult | AmIRichestResult

export type CustomPermission = "all_info" | "am_i_richest" | ""

// export type Account = {
//   address: string;
//   mnemonic: string;
//   walletAmino: AminoWallet;
//   walletProto: Wallet;
//   secretjs: SecretNetworkClient;
// };


// =========================================
// App UI 
// =========================================

export type UserInputs = {
  networth: string,
  viewingKey: string,
  permitName: string,
  permission: CustomPermission,
  queryAddr: string,
  queryKey: string,
  permitId: number,
} 

export type FormRow = {
  headerText: string
  inputs: FormInput[],
  buttons: FormButton<SecretNetworkClient>[],
}

export type FormInput = {
  field: keyof UserInputs,
  placeholderText: string,
}

export type FormButton<T> = {
  onFunction: (acc: T) => Promise<void>,  
  buttonText: string,
}
