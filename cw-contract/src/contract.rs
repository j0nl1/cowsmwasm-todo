#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::executions::{add_todo, change_status, delete_todo};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::queries::{query_list, query_todo};
use crate::state::{INDEX, OWNER};

const CONTRACT_NAME: &str = "crates.io:todo-list";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    OWNER.save(deps.storage, &info.sender)?;
    INDEX.save(deps.storage, &msg.index)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("index", msg.index.to_string())
        .add_attribute("owner", info.sender.to_string()))
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
        ExecuteMsg::ChangeStatus { id, status } => change_status(deps, info, id, status),
        ExecuteMsg::Delete { id } => delete_todo(deps, info, id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetTodo { id, addr } => to_binary(&query_todo(deps, id, addr)?),
        QueryMsg::GetList { addr } => to_binary(&query_list(deps, addr)?),
    }
}
