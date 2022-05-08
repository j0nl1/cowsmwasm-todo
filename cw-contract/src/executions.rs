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

#[cfg(test)]
mod tests {
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use crate::contract::{execute, instantiate};
    use crate::models::{Status, Todo};
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use crate::state::TODOS;

    #[test]
    fn add_todo() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { index: 0 };
        let info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let description = String::from("Improve tests");

        let msg = ExecuteMsg::AddTodo {
            description: description.clone(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        assert_eq!(true, TODOS.has(&deps.storage, (info.sender, 0)));
    }

    #[test]
    fn delete_todo() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { index: 0 };
        let info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let id = 0;
        let todo = Todo {
            description: String::from("Improve tests"),
            status: Status::OPEN,
        };

        let _res = TODOS.save(
            deps.as_mut().storage,
            (info.sender.clone(), id.clone()),
            &todo,
        );
        assert_eq!(
            true,
            TODOS.has(&deps.storage, (info.sender.clone(), id.clone()))
        );

        let msg = ExecuteMsg::Delete { id: id.clone() };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        assert_eq!(false, TODOS.has(&deps.storage, (info.sender, id)));
    }

    #[test]
    fn change_todo() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { index: 0 };
        let info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let id = 0;
        let todo = Todo {
            description: String::from("Improve tests"),
            status: Status::OPEN,
        };

        let _res = TODOS.save(
            deps.as_mut().storage,
            (info.sender.clone(), id.clone()),
            &todo,
        );
        assert_eq!(
            true,
            TODOS.has(&deps.storage, (info.sender.clone(), id.clone()))
        );

        let status = 2;

        let msg = ExecuteMsg::ChangeStatus {
            id: id.clone(),
            status: status.clone(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let todo_ref = TODOS.key((info.sender, id));
        let todo_loaded = todo_ref.load(&deps.storage).unwrap();

        assert_eq!(todo_loaded.status, Status::from(&status))
    }
}
