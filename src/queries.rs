use cosmwasm_std::{Deps, Order, StdError, StdResult};
use cw_storage_plus::Bound;

use crate::{models::Todo, msg::TodosResponse, state::TODOS};

const MAX_LIMIT: u64 = 30;
const DEFAULT_LIMIT: u64 = 10;

pub fn query_todo(deps: Deps, id: u64) -> StdResult<Todo> {
    match TODOS.load(deps.storage, id) {
        Ok(todo) => Ok(todo),
        Err(_) => Err(StdError::not_found("todo")),
    }
}

pub fn query_todo_list(
    deps: Deps,
    offset: Option<u64>,
    limit: Option<u64>,
) -> StdResult<TodosResponse> {
    let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT);
    let min: u64 = offset.unwrap_or(0);
    let max: u64 = min + limit;

    let todos: StdResult<Vec<Todo>> = TODOS
        .range(
            deps.storage,
            Some(Bound::inclusive(min)),
            Some(Bound::inclusive(max)),
            Order::Ascending,
        )
        .take(limit as usize)
        .map(|i| i.map(|e| e.1))
        .collect();

    Ok(TodosResponse { todos: todos? })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::from_binary;
    use cosmwasm_std::testing::{mock_dependencies, mock_env};

    use crate::contract::query;
    use crate::models::{Status, Todo};
    use crate::msg::{QueryMsg, TodosResponse};
    use crate::state::TODOS;

    #[test]
    fn get_todo() {
        let mut deps = mock_dependencies();

        let id = 1;
        let todo = Todo {
            id: id,
            description: String::from("Improve tests"),
            status: Status::Pending,
        };

        let _res = TODOS.save(deps.as_mut().storage, id, &todo);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetTodo { id }).unwrap();
        let value: Todo = from_binary(&res).unwrap();
        assert_eq!(value, todo);
    }

    #[test]
    fn get_list() {
        let mut deps = mock_dependencies();

        let open_todo = Todo {
            id: 0,
            description: String::from("OPEN"),
            status: Status::Pending,
        };
        let completed_todo = Todo {
            id: 1,
            description: String::from("COMPLETED"),
            status: Status::Done,
        };

        let _f1 = TODOS.save(deps.as_mut().storage, 0, &open_todo);
        let _f2 = TODOS.save(deps.as_mut().storage, 1, &completed_todo);

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetTodoList {
                offset: None,
                limit: None,
            },
        )
        .unwrap();
        let todos: TodosResponse = from_binary(&res).unwrap();
        assert_eq!(todos.todos, vec![open_todo, completed_todo]);
    }
}
