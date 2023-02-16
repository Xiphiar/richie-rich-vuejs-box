use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, Addr, Binary, StdError, 
};
use secret_toolkit::{
    viewing_key::{ViewingKey, ViewingKeyStore}, 
    permit::Permit
};

use crate::{
    error::{ContractError}, 
    msg::{QueryWithPermit, RichieRichPermissions}
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryAnswer};
use crate::state::{state, state_read, Outcome, NetWorthStore};

pub const PREFIX_REVOKED_PERMITS: &str = "revoked_permits";

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {

    let init_state = Outcome::init();
    // demonstates how to use Singleton
    state(deps.storage).save(&init_state)?;

    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::SubmitNetWorth { networth } => try_submit_net_worth(deps, info, networth),
        ExecuteMsg::SetViewingKey { key } => try_set_key(deps, info, key),
    }
}

#[entry_point]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let q_response = match msg {
        // There's some repeated code which can be moved into a separate function.
        // We've laid it our this way for clarity
        QueryMsg::AllInfo { .. } => {
            let (address, validated_key) = msg.get_validation_params(deps.api)?;
            let result = ViewingKey::check(deps.storage, address.as_str(), validated_key.as_str());
            match result.is_ok() {
                true => query_all_info(deps, address),
                false => Err(StdError::generic_err("Wrong viewing key for this address or viewing key not set")),
            }
        },
        QueryMsg::AmIRichest { .. } => {
            let (address, validated_key) = msg.get_validation_params(deps.api)?;
            let result = ViewingKey::check(deps.storage, address.as_str(), validated_key.as_str());
            match result.is_ok() {
                true => query_richest(deps, address),
                false => Err(StdError::generic_err("Wrong viewing key for this address or viewing key not set")),
            }
        },
        QueryMsg::WithPermit { permit, query } => permit_queries(deps, env, permit, query),
    };

    to_binary(&q_response?)
}

fn permit_queries(deps: Deps, env: Env, permit: Permit<RichieRichPermissions>, query: QueryWithPermit) -> StdResult<QueryAnswer> {
    // Validate permit content
    let contract_address = env.contract.address;

    let account = secret_toolkit::permit::validate(
        deps,
        PREFIX_REVOKED_PERMITS,
        &permit,
        contract_address.into_string(),
        None,
    )?;

    // Permit validated! We can now execute the query.
    match query {
        QueryWithPermit::AllInfo {} => {
            if !permit.check_permission(&RichieRichPermissions::AllInfo) {
                return Err(StdError::generic_err(format!(
                    "No permission to query, got permissions {:?}",
                    permit.params.permissions
                )));
            }

            query_all_info(deps, deps.api.addr_validate(&account)?)
        }
        QueryWithPermit::AmIRichest {  } => {
            if !permit.check_permission(&RichieRichPermissions::AmIRichest) {
                return Err(StdError::generic_err(format!(
                    "No permission to query, got permissions {:?}",
                    permit.params.permissions
                )));
            }

            query_richest(deps, deps.api.addr_validate(&account)?)
        }
    }
}


pub fn try_submit_net_worth(
    deps: DepsMut,
    info: MessageInfo,
    networth: u128,
) -> Result<Response, ContractError> {
    // saves submission for each address can view their latest submission -- will override existing if exists
    NetWorthStore::save(deps.storage, &info.sender, networth)?;

    // Compares networth with current highest, and update state if necessary
    // For simplicity, if networth is equal, the first Millionaire remains the richest
    let mut outcome = state(deps.storage).load()?;

    match networth > outcome.richest.networth {
        true => outcome.update_richest(info.sender, networth),
        false => (),
    }

    // save updated outcome on who's richest
    state(deps.storage).save(&outcome)?;

    Ok(Response::new())
}

pub fn try_set_key(deps: DepsMut, info: MessageInfo, key: String) -> Result<Response, ContractError> {
    ViewingKey::set(deps.storage, info.sender.as_str(), key.as_str());
    Ok(Response::new())
}

fn query_all_info(
    deps: Deps,
    addr: Addr,
) -> StdResult<QueryAnswer> {
    let outcome = state_read(deps.storage).load()?;
    let richest = outcome.richest.addr == addr;
    let networth = NetWorthStore::load(deps.storage, &addr);

    let resp = QueryAnswer::AllInfo { 
        richest,
        networth,
    };
        
    Ok(resp)
}

fn query_richest(
    deps: Deps,
    addr: Addr,
) -> StdResult<QueryAnswer> {
    let outcome = state_read(deps.storage).load()?;
    let richest = outcome.richest.addr == addr;

    let resp = QueryAnswer::AmIRichest {
        richest,
    };
        
    Ok(resp)
}


#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{
        mock_env, mock_info, mock_dependencies,
        MockStorage, MockApi, MockQuerier
    };
    use cosmwasm_std::{coins, OwnedDeps};

    fn init_helper() -> (StdResult<Response>, OwnedDeps<MockStorage, MockApi, MockQuerier>) {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(0, "coins"));
        let res = instantiate(deps.as_mut(), mock_env(), info, msg);
        (res, deps)
    }

    fn submit_networth_helper(
        deps: &mut OwnedDeps<MockStorage, MockApi, MockQuerier>,
        submissions: Vec<(&str, u128)>
    ) -> Vec<Response>  {
        let mut res_vec = vec![];
        for (sender, networth) in submissions {
            let msg = ExecuteMsg::SubmitNetWorth {networth};
            let info = mock_info(sender, &[]);
            let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
            res_vec.push(res);
        }
        res_vec
    }


    #[test]
    fn test_init_sanity() {
        let (res, deps) = init_helper();

        // we can call .unwrap() to assert this was a success
        assert_eq!(0, res.unwrap().messages.len());

        let res = query_all_info(deps.as_ref(), Addr::unchecked("alice")).unwrap();
        // behavior when querying an address that has not submitted anything:
        match res {
            QueryAnswer::AllInfo { richest, networth } => {
                assert_eq!(richest, false); assert_eq!(networth, 0);       
            },
            res => panic!("unexpected QueryAnswer type: {res:?}"),
        }
    }

    #[test]
    fn test_query_richest() {
        let (_, mut deps) = init_helper();
        let submissions = vec![("alice", 1), ("bob", 2)];
        submit_networth_helper(&mut deps, submissions);

        let alice_query_res = query_all_info(deps.as_ref(), Addr::unchecked("alice")).unwrap();
        let bob_query_res = query_all_info(deps.as_ref(), Addr::unchecked("bob")).unwrap();

        match alice_query_res {
            QueryAnswer::AllInfo { richest, networth } => {
                assert_eq!(richest, false); assert_eq!(networth, 1);       
            },
            res => panic!("unexpected QueryAnswer type: {res:?}"),
        }
        match bob_query_res {
            QueryAnswer::AllInfo { richest, networth } => {
                assert_eq!(richest, true); assert_eq!(networth, 2);       
            },
            res => panic!("unexpected QueryAnswer type: {res:?}"),
        }
    }
}