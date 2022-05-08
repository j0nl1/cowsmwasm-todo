use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError};
use std::convert::TryFrom;

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
    TODOS.save(deps.storage, (&info.sender, id), &data)?;
    Ok(Response::new()
        .add_attribute("action", "add_todo")
        .add_attribute("id", id.to_string()))
}

pub fn edit_todo(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    todo: Todo,
) -> Result<Response, ContractError> {
    TODOS.update(
        deps.storage,
        (&info.sender, id),
        |_| -> Result<Todo, ContractError> {
            Ok(Todo {
                description: todo.description,
                status: todo.status,
            })
        },
    )?;
    Ok(Response::new()
        .add_attribute("action", "edit_todo")
        .add_attribute("todo_id", id.to_string()))
}

pub fn change_status(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    status: u8,
) -> Result<Response, ContractError> {
    TODOS.update(
        deps.storage,
        (&info.sender, id),
        |todo| -> Result<Todo, ContractError> {
            match todo {
                Some(mut t) => {
                    t.status = Status::try_from(&status)?;
                    Ok(t)
                }
                None => Err(ContractError::Std(StdError::not_found("todo"))),
            }
        },
    )?;
    Ok(Response::new()
        .add_attribute("action", "change_status")
        .add_attribute("todo_id", &id.to_string())
        .add_attribute("status", &status.to_string())
    )
}

pub fn delete_todo(deps: DepsMut, info: MessageInfo, id: u64) -> Result<Response, ContractError> {
    TODOS.remove(deps.storage, (&info.sender, id));
    Ok(Response::new().add_attribute("action", "delete_todo").add_attribute("todo_id", &id.to_string()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use std::convert::TryFrom;

    use crate::contract::{execute, instantiate};
    use crate::models::{Status, Todo};
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use crate::state::TODOS;

    const IMSG: InstantiateMsg = InstantiateMsg {};

    #[test]
    fn add_todo() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), IMSG).unwrap();

        let description = "Improve tests".to_string();

        let msg = ExecuteMsg::AddTodo {
            description: description.clone(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        assert_eq!(true, TODOS.has(&deps.storage, (&info.sender, 0)));
    }

    #[test]
    fn delete_todo() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), IMSG).unwrap();

        let id = 0;
        let todo = Todo {
            description: "Improve tests".to_string(),
            status: Status::OPEN,
        };

        let _res = TODOS.save(deps.as_mut().storage, (&info.sender, id.clone()), &todo);
        assert_eq!(true, TODOS.has(&deps.storage, (&info.sender, id.clone())));

        let msg = ExecuteMsg::Delete { id: id.clone() };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        assert_eq!(false, TODOS.has(&deps.storage, (&info.sender, id)));
    }

    #[test]
    fn change_todo() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), IMSG).unwrap();

        let id = 0;
        let todo = Todo {
            description: "Improve tests".to_string(),
            status: Status::OPEN,
        };

        let _res = TODOS.save(deps.as_mut().storage, (&info.sender, id.clone()), &todo);
        assert_eq!(true, TODOS.has(&deps.storage, (&info.sender, id.clone())));

        let status = 2;

        let msg = ExecuteMsg::ChangeStatus {
            id: id.clone(),
            status: status.clone(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let todo_ref = TODOS.key((&info.sender, id));
        let todo_loaded = todo_ref.load(&deps.storage).unwrap();

        assert_eq!(todo_loaded.status, Status::try_from(&status).unwrap());
    }
}