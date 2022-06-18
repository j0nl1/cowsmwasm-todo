#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};
use cw2::set_contract_version;
use semver::Version;

use crate::error::ContractError;
use crate::executions::{add_todo, delete_todo, update_todo};
use crate::models::Config;
use crate::msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::queries::{query_todo, query_todo_list};
use crate::state::{CONFIG, INDEX};

const CONTRACT_NAME: &str = "crates.io:todo_list";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let owner = msg
        .owner
        .and_then(|addr_string| deps.api.addr_validate(addr_string.as_str()).ok())
        .unwrap_or(info.sender);

    let config = Config {
        owner: deps.api.addr_canonicalize(&owner.to_string())?,
    };

    CONFIG.save(deps.storage, &config)?;

    INDEX.save(deps.storage, &0u64)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", owner))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let cw_info = cw2::get_contract_version(deps.storage)?;
    let cver: Version = cw_info.version.parse()?;

    if cw_info.contract != CONTRACT_NAME {
        return Err(StdError::generic_err("Can only upgrade from same type").into());
    }

    if cver < CONTRACT_VERSION.parse()? {
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    }

    Ok(Response::default())
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::AddTodo { description } => add_todo(deps, info, description),
        ExecuteMsg::UpdateTodo {
            id,
            description,
            status,
        } => update_todo(deps, info, id, description, status),
        ExecuteMsg::DeleteTodo { id } => delete_todo(deps, info, id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTodo { id } => to_binary(&query_todo(deps, id)?),
        QueryMsg::GetTodoList { offset, limit } => {
            to_binary(&query_todo_list(deps, offset, limit)?)
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};

    use crate::contract::instantiate;
    use crate::msg::InstantiateMsg;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg {
            owner: Some("owner".to_string()),
        };
        let info = mock_info("creator", &coins(1000, "token"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(res.attributes[1].value, "owner".to_string());
    }
}
