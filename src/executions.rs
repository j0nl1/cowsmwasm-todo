use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError};

use crate::error::ContractError;
use crate::helpers::ensure_is_owner;
use crate::models::{Status, Todo};
use crate::state::{TODOS, INDEX};

pub fn add_todo(
    deps: DepsMut,
    info: MessageInfo,
    description: String,
) -> Result<Response, ContractError> {
    ensure_is_owner(deps.as_ref(), &info.sender)?;
    let id = INDEX.update::<_, StdError>(deps.storage, |id| {
        Ok(id + 1)
    })?;
    let data = Todo {
        id,
        description,
        status: Status::Pending,
    };
    TODOS.save(deps.storage, id, &data)?;
    Ok(Response::new()
        .add_attribute("method", "add_todo")
        .add_attribute("id", id.to_string()))
}

pub fn update_todo(
    deps: DepsMut,
    info: MessageInfo,
    id: u64,
    description: Option<String>,
    status: Option<Status>,
) -> Result<Response, ContractError> {
    ensure_is_owner(deps.as_ref(), &info.sender)?;
    let todo = TODOS.load(deps.storage, id)?;

    let updated_todo = Todo {
        id,
        description: description.unwrap_or(todo.description),
        status: status.unwrap_or(todo.status),
    };

    TODOS.save(deps.storage, id, &updated_todo)?;

    Ok(Response::new()
        .add_attribute("method", "update_todo")
        .add_attribute("id", id.to_string()))
}

pub fn delete_todo(deps: DepsMut, info: MessageInfo, id: u64) -> Result<Response, ContractError> {
    ensure_is_owner(deps.as_ref(), &info.sender)?;
    TODOS.remove(deps.storage, id);
    Ok(Response::new()
        .add_attribute("method", "delete_todo")
        .add_attribute("id", &id.to_string()))
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins};
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};

    use crate::contract::{execute, instantiate};
    use crate::models::{Status, Todo};
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use crate::state::TODOS;

   
    const IMSG: InstantiateMsg = InstantiateMsg { owner: None };

    #[test]
    fn add_todo() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), IMSG).unwrap();

        let msg = ExecuteMsg::AddTodo {
            description: "Improve tests".to_string(),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        assert_eq!(true, TODOS.has(&deps.storage, 1));
    }

    #[test]
    fn delete_todo() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), IMSG).unwrap();

        let id = 0;
        let todo = Todo {
            id,
            description: "Improve tests".to_string(),
            status: Status::Pending,
        };

        let _res = TODOS.save(deps.as_mut().storage, id, &todo);
        assert_eq!(true, TODOS.has(&deps.storage, id));

        let msg = ExecuteMsg::DeleteTodo { id };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        assert_eq!(false, TODOS.has(&deps.storage, id));
    }

    #[test]
    fn edit_todo() {
        let mut deps = mock_dependencies();

        let info = mock_info("creator", &coins(1000, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), IMSG).unwrap();

        let id = 0;
        let todo = Todo {
            id,
            description: "Improve tests".to_string(),
            status: Status::Pending,
        };

        let _res = TODOS.save(deps.as_mut().storage, id, &todo);
        assert_eq!(true, TODOS.has(&deps.storage, id));

        

        let msg = ExecuteMsg::UpdateTodo {
            id,
            description: None,
            status: Some(Status::InProgress),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let todo_ref = TODOS.key(id);
        let todo_loaded = todo_ref.load(&deps.storage).unwrap();

        assert_eq!(todo_loaded.status, Status::InProgress);
    }
}
