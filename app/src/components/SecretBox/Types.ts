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
} 

export type FormRow = {
  headerText: string
  onFunction: () => Promise<void>,
  inputs: FormRowInput[],
  buttonText: string,
}

export type FormRowInput = {
  field: keyof UserInputs,
  placeholderText: string,
}
