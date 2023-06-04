use cosmwasm_std::{Addr, Api, StdResult, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit::permit::Permit;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SubmitNetWorth { networth: Uint128 },
    SetViewingKey { key: String },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    AllInfo { 
        addr: Addr,
        key: String,
    },
    AmIRichest {
        addr: Addr,
        key: String,
    },
    //
    // complete code here
    // 
}

impl QueryMsg {
    pub fn get_validation_params(&self) -> (/* complete code here */) {
        //
        // complete code here
        //
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryWithPermit {
    //
    // complete code here
    //
}

/// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryAnswer {
    AllInfo {
        richest: bool,
        networth: Uint128
    },
    AmIRichest {
        richest: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RichieRichPermissions {
    //
    // complete code here
    //
}