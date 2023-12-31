#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Empty, BankQuery};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetGroupTypeResponse, InstantiateMsg, QueryMsg, ControllerMsg};
use crate::state::*;
use crate::executors as execute;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:sweet-cosmwasm";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
        live_status: if msg.live { LiveStatus::Alive } else { LiveStatus::default() },
        group_type: msg.group_type,
        expiry: Expiry::default(),
        recovery: RecoveryInfo::default(),
        credential: Credential::default(),
        version: VersionInfo::default(),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    /* TEMP:
    pub const STATE: Item<State> = Item::new("state");
    pub const MEMBERS: Item<Vec<Member>> = Item::new("members");
    pub const RULES: Item<Vec<Rule>> = Item::new("rules");
     */
    STATE.save(deps.storage, &state)?;
    
    for (idx, member) in msg.members.into_iter().enumerate() {
        // example: ADMINS.save(deps.storage, &admin, &env.block.time)?;
        // ! : production should check that size (mapIndexType) is acceptable
        MEMBERS.save(deps.storage, idx as mapIndexType, &member)?;
    }
    for (idx, rule) in msg.rules.into_iter().enumerate() {
        // ! : production should check that size (mapIndexType) is acceptable
        RULES.save(deps.storage, idx as mapIndexType, &rule)?;
    }

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("sender", info.sender)
        .add_attribute("expiry", "((expiry attribute: to do))")
        .add_attribute("recovery", "((recovery attribute: to do))")
        .add_attribute("pk", "((pk receipt attribute?: maybe to do))")
        .add_attribute("version", "((version attribute: to do))"))



}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    /* example:
    match msg {
        ExecuteMsg::Increment {} => execute::increment(deps),
        ExecuteMsg::Reset { count } => execute::reset(deps, info, count),
    } */

    match msg {
        ExecuteMsg::ControlMsg { control_msg, credential } => {
            execute::check_credentials(Sender::Controller, credential)?;
            execute::controller_msg_handler(deps, _env, info, control_msg)
        },
        ExecuteMsg::MemberMsg{ member_msg, idx, credential }  => {
            execute::check_credentials(Sender::Member(idx), credential)?;
            execute::member_msg_handler(deps, _env, info, idx, member_msg)
        },
    }
}

// QUERY stuff

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // EXAMPLE: QueryMsg::GetCountType {} => to_binary(&query::count(deps)?),
        QueryMsg::GetLiveStatus {} => to_binary(&query::get_live_status(deps)?),
        QueryMsg::GetGroupType {} => to_binary(&query::get_group_type(deps)?),
        QueryMsg::GetMembers {} => to_binary(&query::get_members(deps)?),
        QueryMsg::GetRules {} => to_binary(&query::get_rules(deps)?),
        QueryMsg::GetExpiry {} => to_binary(&query::get_expiry(deps)?),
        QueryMsg::GetRecoveryInfo {} => to_binary(&query::get_recovery_info(deps)?),
        QueryMsg::GetVersionInfo {} => to_binary(&query::get_version_info(deps)?),
        QueryMsg::GetBalances {} => to_binary(&query::get_balances(deps, env)?),

    }
}

// query handlers
pub mod query {
    use cosmwasm_std::QueryRequest;

    use crate::msg::{GetLiveStatusResponse, GetMembersResponse, GetRulesResponse, GetExpiryResponse, GetRecoveryInfoResponse, GetVersionInfoResponse, GetBalancesResponse};

    use super::*;

    /* example:
    pub fn count(deps: Deps) -> StdResult<GetCountResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetGroupTypeResponse { count: state.count })
    } */
    
    pub fn get_live_status(deps: Deps) -> StdResult<GetLiveStatusResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetLiveStatusResponse { live_status: state.live_status })
    }

    pub fn get_group_type(deps: Deps) -> StdResult<GetGroupTypeResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetGroupTypeResponse { group_type: state.group_type })
    }

    pub fn get_members(deps: Deps) -> StdResult<GetMembersResponse> {
        todo!(); // Z! 
        /* for ,,,  - but need the true index
            let member = MEMBERS.load(deps.storage, idx)?; */
        Ok(GetMembersResponse { members: todo!() })
    }
    
    pub fn get_rules(deps: Deps) -> StdResult<GetRulesResponse> {
        todo!(); // Z! 
        /* for ,,,  - but need the true index
            let member = MEMBERS.load(deps.storage, idx)?; */
        Ok(GetRulesResponse { rules: todo!() })
    }

    pub fn get_expiry(deps: Deps) -> StdResult<GetExpiryResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetExpiryResponse { expiry: state.expiry })
    }

    pub fn get_recovery_info(deps: Deps) -> StdResult<GetRecoveryInfoResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetRecoveryInfoResponse { recover_info: state.recovery })
    }

    pub fn get_version_info(deps: Deps) -> StdResult<GetVersionInfoResponse> {
        let state = STATE.load(deps.storage)?;
        Ok(GetVersionInfoResponse { version_info: state.version })
    }

    pub fn get_balances(deps: Deps, env: Env) -> StdResult<GetBalancesResponse> {
        /* deps.querier.query(&cosmwasm_std::QueryRequest::Bank(
            BankQuery::Balance {
                address: env.contract.address,
                denom: todo!(),
            }
        ))? */
        todo!()
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = todo!(); //InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetGroupType {}).unwrap();
        let value: GetGroupTypeResponse = from_binary(&res).unwrap();
        assert_eq!(17, 16);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = todo!(); //InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::MemberMsg { member_msg: crate::msg::MemberMsg::Test {  }, idx: 0u8, credential: Credential::CREDENTIAL_TO_BE_DEFINED }; //ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetGroupType {}).unwrap();
        let value: GetGroupTypeResponse = from_binary(&res).unwrap();
        assert_eq!(18,17);
    }

    #[test]
    fn reset() {
        /* let mut deps = mock_dependencies();

        let msg = todo!(); //InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetGroupType {}).unwrap();
        let value: GetGroupTypeResponse = from_binary(&res).unwrap(); */
        assert_eq!(5, 4);
    }
}
