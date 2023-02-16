use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, Addr, Binary, StdError, //StdError,
};
use secret_toolkit::{
    viewing_key::{ViewingKey, ViewingKeyStore}, 
    permit::{Permit, TokenPermissions}
};

use crate::{
    error::{ContractError}, 
    msg::QueryWithPermit
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
        QueryMsg::AllInfo { .. } => viewing_keys_queries(deps, msg),
        QueryMsg::WithPermit { permit, query } => permit_queries(deps, env, permit, query),
    };

    to_binary(&q_response?)
}

pub fn try_submit_net_worth(
    deps: DepsMut,
    info: MessageInfo,
    networth: u128,
) -> Result<Response, ContractError> {
    // saves submission for each address can view their latest submission -- will override existing if exists
    NetWorthStore::save(deps.storage, &info.sender, networth)?;

    // Compares networth with current highest -- update state if necessary
    // For simplicity, if networth is equal, the first Millionaire remains the richest
    // In a real-world contract, you'd want to have functionality to handle ties 
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

pub fn viewing_keys_queries(deps: Deps, msg: QueryMsg) -> StdResult<QueryAnswer> {
    let (addresses, key) = msg.get_validation_params(deps.api)?;

    for address in addresses {
        let result = ViewingKey::check(deps.storage, address.as_str(), key.as_str());
        if result.is_ok() {
            return match msg {
                QueryMsg::AllInfo { addr, .. } => query_all_info(deps, addr),
                QueryMsg::WithPermit { .. } => unreachable!("Trying to query with permit on a viewing key query function"),
            };
        }
    }

//todo!(make it contract error)
    Ok(QueryAnswer::ViewingKeyError {
        msg: "Wrong viewing key for this address or viewing key not set".to_string(),
    })
}

fn permit_queries(deps: Deps, env: Env, permit: Permit, query: QueryWithPermit) -> StdResult<QueryAnswer> {
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
            if !permit.check_permission(&TokenPermissions::Owner) {
                return Err(StdError::generic_err(format!(
                    "No permission to query balance, got permissions {:?}",
                    permit.params.permissions
                )));
            }

            query_all_info(deps, deps.api.addr_validate(&account)?)
        }
    }
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


#[cfg(test)]
mod tests {
    use super::*;

    use cosmwasm_std::testing::{mock_env, mock_info, mock_dependencies};
    use cosmwasm_std::coins;

    #[test]
    fn proper_instantialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

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
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg_alice = ExecuteMsg::SubmitNetWorth {networth: 1};
        let msg_bob = ExecuteMsg::SubmitNetWorth {networth: 2};

        let info = mock_info("alice", &[]);
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg_alice).unwrap();

        let info = mock_info("bob", &[]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg_bob).unwrap();

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