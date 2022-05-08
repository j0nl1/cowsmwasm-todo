use crate::{models::Todo, msg::TodosResponse, state::TODOS};
use cosmwasm_std::{Addr, Deps, Order, StdResult};

pub fn query_todo(deps: Deps, id: u64, addr: Addr) -> StdResult<Todo> {
    let todo = TODOS.key((addr, id));
    let state = todo.load(deps.storage)?;
    Ok(state)
}

pub fn query_list(deps: Deps, addr: Addr) -> StdResult<TodosResponse> {
    let todos: StdResult<Vec<_>> = TODOS
        .prefix(addr)
        .range(deps.storage, None, None, Order::Ascending)
        .collect();
    Ok(TodosResponse { todos: todos? })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    use crate::contract::query;
    use crate::models::{Status, Todo};
    use crate::msg::{QueryMsg, TodosResponse};
    use crate::state::TODOS;

    #[test]
    fn get_todo() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &coins(1000, "token"));

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

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetTodo {
                addr: info.sender,
                id: id.clone(),
            },
        )
        .unwrap();
        let value: Todo = from_binary(&res).unwrap();
        assert_eq!(value, todo);
    }

    #[test]
    fn get_list() {
        let mut deps = mock_dependencies();
        let info = mock_info("creator", &coins(1000, "token"));

        let open_todo = Todo {
            description: String::from("OPEN"),
            status: Status::OPEN,
        };
        let closed_todo = Todo {
            description: String::from("CLOSED"),
            status: Status::CLOSED,
        };

        let _f1 = TODOS.save(deps.as_mut().storage, (info.sender.clone(), 0), &open_todo);
        let _f2 = TODOS.save(
            deps.as_mut().storage,
            (info.sender.clone(), 1),
            &closed_todo,
        );

        let res = query(
            deps.as_ref(),
            mock_env(),
            QueryMsg::GetList {
                addr: info.sender.clone(),
            },
        )
        .unwrap();
        let todos: TodosResponse = from_binary(&res).unwrap();
        assert_eq!(todos.todos, vec![(0, open_todo), (1, closed_todo)]);
    }
}
