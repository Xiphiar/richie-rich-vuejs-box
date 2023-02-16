use cosmwasm_std::{Addr, Api, StdResult};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use secret_toolkit::permit::Permit;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SubmitNetWorth { networth: u128 },
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
    WithPermit {
        permit: Permit<RichieRichPermissions>,
        query: QueryWithPermit,
    },
}

impl QueryMsg {
    pub fn get_validation_params(&self, api: &dyn Api) -> StdResult<(Addr, String)> {
        match self {
            Self::AllInfo { addr, key } => {
                let address = api.addr_validate(addr.as_str())?;
                Ok((address, key.clone()))
            }
            Self::AmIRichest { addr, key } => {
                let address = api.addr_validate(addr.as_str())?;
                Ok((address, key.clone()))
            },
            Self::WithPermit { .. } => panic!("This query type does not require authentication"),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum QueryWithPermit {
    AllInfo {  },
    AmIRichest {  },
}

/// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum QueryAnswer {
    AllInfo {
        richest: bool,
        networth: u128
    },
    AmIRichest {
        richest: bool,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum RichieRichPermissions {
    AllInfo,
    AmIRichest,
}