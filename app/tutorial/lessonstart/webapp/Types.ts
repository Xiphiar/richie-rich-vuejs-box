import { SecretNetworkClient, } from "secretjs"


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


// =========================================
// App UI 
// =========================================

export type UserInputs = {
  networth: AccountLevelInputs<string>,
  viewingKey: AccountLevelInputs<string>,
  permitName: AccountLevelInputs<string>,
  permission: AccountLevelInputs<CustomPermission>,
  queryAddr: string,
  queryKey: string,
  permitId: number,
} 

type AccountLevelInputs<T> = {
  [key: string]: T;
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
  onFunction: (acc: T) => Promise<void>,// | (() => Promise<void>),
  buttonText: string,
}
