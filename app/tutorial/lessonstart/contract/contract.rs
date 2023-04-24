use cosmwasm_std::{
    entry_point, to_binary, Deps, DepsMut, Env,
    MessageInfo, Response, StdResult, Addr, Binary, StdError, Uint128, 
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
            //
            // complete code here
            // 
            ()
        },
        QueryMsg::AmIRichest { .. } => {
            //
            // complete code here
            // 
            ()
        },
        //
        // complete code here
        // 
    };

    to_binary( /* complete code here */ "placeholder")
}

fn permit_queries(deps: Deps, env: Env, permit: Permit /* add generic */, query: QueryWithPermit) -> (/* complete code here */) {
    // Validate permit content
    let contract_address = env.contract.address;
        //
        // complete code here
        // 

    // Permit validated! We can now execute the query.
        //
        // complete code here
        // 

}


pub fn try_submit_net_worth(
    deps: DepsMut,
    info: MessageInfo,
    networth: Uint128,
) -> Result<Response, ContractError> {
    // checks that account has not already submitted -- can only submit once
    match NetWorthStore::may_load(deps.storage, &info.sender) {
        Some(networth) => return Err(ContractError::AlreadySubmittedNetworth { networth }),
        None => {
            // saves submission for each address can view their submission
            NetWorthStore::save(deps.storage, &info.sender, networth)?;
        },
    }

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
    let may_networth = NetWorthStore::may_load(deps.storage, &addr);
    let networth = match may_networth {
        Some(x) => x,
        None => return Err(StdError::GenericErr { msg: "You have not submitted your networth".to_string() }),
    };

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
    use std::any::Any;

    use crate::state::Millionaire;

    use super::*;

    use cosmwasm_std::testing::{
        mock_env, mock_info, mock_dependencies,
        MockStorage, MockApi, MockQuerier
    };
    use cosmwasm_std::{coins, OwnedDeps, from_binary};

    fn init_helper() -> (
        StdResult<Response>, 
        OwnedDeps<MockStorage, MockApi, MockQuerier>,
    ) {
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
            let msg = ExecuteMsg::SubmitNetWorth {networth: Uint128::from(networth)};
            let info = mock_info(sender, &[]);
            let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();
            res_vec.push(res);
        }
        res_vec
    }

    fn assert_info(deps: Deps, acc: &str, exp_richest: bool, exp_networth: u128) {
        let res = query_all_info(deps, Addr::unchecked(acc)).unwrap();

        match res {
            QueryAnswer::AllInfo { richest, networth } => {
                assert_eq!(richest, exp_richest); assert_eq!(networth, Uint128::from(exp_networth));       
            },
            res => panic!("unexpected QueryAnswer type: {res:?}"),
        }
    }

    fn assert_info_vec(deps: Deps, acc_richest_networth: Vec<(&str, bool, u128)>) {
        for (acc, exp_richest, exp_networth) in acc_richest_networth {
            assert_info(deps, acc, exp_richest, exp_networth);
        }
    }

    fn assert_gen_err<T: Any>(result: StdResult<T>, err_str_includes: &str) -> bool {
        match result {
            Ok(_) => panic!("An error was expected, but no error could be extracted"),
            Err(err) => match err {
                StdError::GenericErr { msg, .. } => {
                    msg.contains(err_str_includes)
                },
                _ => panic!("Unexpected result"),
            },
        }
    }

    #[test]
    fn test_init_sanity() {
        let (res, deps) = init_helper();

        // we can call .unwrap() to assert this was a success
        assert_eq!(0, res.unwrap().messages.len());

        let state = state_read(deps.as_ref().storage).load().unwrap();
        assert_eq!(state, Outcome { richest: Millionaire { addr: Addr::unchecked(""), networth: Uint128::zero() } });
    }

    #[test]
    fn test_richest_sanity() {
        let (_, mut deps) = init_helper();
        submit_networth_helper(&mut deps, vec![("alice", 1), ("bob", 2)]);

        let alice_query_res = query_all_info(deps.as_ref(), Addr::unchecked("alice")).unwrap();
        let bob_query_res = query_all_info(deps.as_ref(), Addr::unchecked("bob")).unwrap();

        match alice_query_res {
            QueryAnswer::AllInfo { richest, networth } => {
                assert_eq!(richest, false); assert_eq!(networth, Uint128::one());       
            },
            res => panic!("unexpected QueryAnswer type: {res:?}"),
        }
        match bob_query_res {
            QueryAnswer::AllInfo { richest, networth } => {
                assert_eq!(richest, true); assert_eq!(networth, Uint128::from(2u128));       
            },
            res => panic!("unexpected QueryAnswer type: {res:?}"),
        }
    }

    #[test]
    fn test_richest_logic() {
        let (_, mut deps) = init_helper();
        submit_networth_helper(&mut deps, vec![("alice", 1), ("bob", 2)]);

        assert_info_vec(deps.as_ref(), vec![
            ("alice", false, 1), ("bob", true, 2)
        ]);

        // cannot resubmit
        let msg = ExecuteMsg::SubmitNetWorth { networth: Uint128::from(3u128) };
        let info = mock_info("alice", &[]);
        let res = execute(deps.as_mut(), mock_env(), info, msg);
        match res {
            Ok(_) => assert!(false, "expected an error, but got success"),
            Err(err) => match err {
                ContractError::Std(_) => assert!(false, "expected an ContractError, but got StdError"),
                ContractError::AlreadySubmittedNetworth { networth } => {
                    assert_eq!(networth, Uint128::one())
                },
            },
        }

        // tied networth => previous `richest` remains the same
        submit_networth_helper(&mut deps, vec![("carol", 2)]);
        assert_info_vec(deps.as_ref(), vec![
            ("alice", false, 1), ("bob", true, 2), ("carol", false, 2)
        ]);

        // new richest
        submit_networth_helper(&mut deps, vec![("dan", 3)]);
        assert_info_vec(deps.as_ref(), vec![
            ("alice", false, 1), ("bob", false, 2), ("carol", false, 2), ("dan", true, 3)
        ]);

        // new but not richest
        submit_networth_helper(&mut deps, vec![("eve", 2)]);
        assert_info_vec(deps.as_ref(), vec![
            ("alice", false, 1), ("bob", false, 2), ("carol", false, 2), ("dan", true, 3), ("eve", false, 2)
        ]);

        // executing set viewing key does not change state on submissions
        let setvk_msg = ExecuteMsg::SetViewingKey { key: "vka".to_string() };
        let info = mock_info("alice", &[]);
        execute(deps.as_mut(), mock_env(), info, setvk_msg).unwrap();
        assert_info_vec(deps.as_ref(), vec![
            ("alice", false, 1), ("bob", false, 2), ("carol", false, 2), ("dan", true, 3), ("eve", false, 2)
        ]);
    }

    #[test]
    fn test_vk_query() {
        let (_, mut deps) = init_helper();
        submit_networth_helper(&mut deps, vec![("alice", 1), ("bob", 2)]);

        // no vk set yet ----------------------
        // AllInfo
        let q_msg_all = QueryMsg::AllInfo { addr: Addr::unchecked("alice"), key: "vka".to_string() };
        let query_result = query(deps.as_ref(), mock_env(), q_msg_all.clone());
        assert_gen_err(query_result, "Wrong viewing key for this address or viewing key not set");

        // AmIRichest
        let q_msg_richest = QueryMsg::AmIRichest { addr: Addr::unchecked("alice"), key: "vka".to_string() };
        let query_result = query(deps.as_ref(), mock_env(), q_msg_richest.clone());
        assert_gen_err(query_result, "Wrong viewing key for this address or viewing key not set");

        // set vk ----------------------
        let setvk_msg = ExecuteMsg::SetViewingKey { key: "vka".to_string() };
        let info = mock_info("alice", &[]);
        execute(deps.as_mut(), mock_env(), info, setvk_msg).unwrap();

        // can view result with correct vk ----------------------
        // AllInfo
        let query_result = query(deps.as_ref(), mock_env(), q_msg_all);
        assert!(query_result.is_ok());
        let query_answer = from_binary::<QueryAnswer>(&query_result.unwrap()).unwrap();
        assert_eq!(query_answer, QueryAnswer::AllInfo { richest: false, networth: Uint128::one() });

        // AmIRichest
        let query_result = query(deps.as_ref(), mock_env(), q_msg_richest);
        assert!(query_result.is_ok());
        let query_answer = from_binary::<QueryAnswer>(&query_result.unwrap()).unwrap();
        assert_eq!(query_answer, QueryAnswer::AmIRichest { richest: false });

        // cannot view result with wrong vk ----------------------
        // AllInfo
        let q_msg_wrong_vk_all = QueryMsg::AllInfo { addr: Addr::unchecked("alice"), key: "vk_wrong".to_string() };
        let query_result = query(deps.as_ref(), mock_env(), q_msg_wrong_vk_all);
        assert_gen_err(query_result, "Wrong viewing key for this address or viewing key not set");

        // AmIRichest
        let q_msg_wrong_vk_richest = QueryMsg::AmIRichest { addr: Addr::unchecked("alice"), key: "vk_wrong".to_string() };
        let query_result = query(deps.as_ref(), mock_env(), q_msg_wrong_vk_richest);
        assert_gen_err(query_result, "Wrong viewing key for this address or viewing key not set");

        // cannot view result with "wrong address" ----------------------
        // AllInfo
        let q_msg_wrong_addr_all = QueryMsg::AllInfo { addr: Addr::unchecked("bob"), key: "vka".to_string() };
        let query_result = query(deps.as_ref(), mock_env(), q_msg_wrong_addr_all);
        assert_gen_err(query_result, "Wrong viewing key for this address or viewing key not set");
        
        // AmIRichest
        let q_msg_wrong_addr_richest = QueryMsg::AmIRichest { addr: Addr::unchecked("bob"), key: "vka".to_string() };
        let query_result = query(deps.as_ref(), mock_env(), q_msg_wrong_addr_richest);
        assert_gen_err(query_result, "Wrong viewing key for this address or viewing key not set");
        
    }
}