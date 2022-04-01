use cosmwasm_std::{DepsMut, MessageInfo, Response};

use crate::error::ContractError;
use crate::helpers::get_id;
use crate::models::{Status, Todo};
use crate::state::TODOS;

pub fn add_todo(
    deps: DepsMut,
    info: MessageInfo,
    description: String,
) -> Result<Response, ContractError> {
    let data = Todo {
        description,
        status: Status::OPEN,
    };
    let id = get_id(deps.storage)?;
    TODOS.save(deps.storage, (info.sender, id), &data)?;
    Ok(Response::new()
        .add_attribute("method", "try_add")
        .add_attribute("todo_id", "1"))
}

pub fn change_status(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    status: u8,
) -> Result<Response, ContractError> {
    let todo = TODOS.key((info.sender, id));
    todo.update(deps.storage, |x| -> Result<_, ContractError> {
        let updated_todo = Todo {
            description: x.unwrap().description,
            status: Status::from(&status),
        };
        Ok(updated_todo)
    })?;
    Ok(Response::new())
}

pub fn delete_todo(deps: DepsMut, info: MessageInfo, id: u64) -> Result<Response, ContractError> {
    let todo = TODOS.key((info.sender, id));
    todo.remove(deps.storage);
    Ok(Response::new())
}
