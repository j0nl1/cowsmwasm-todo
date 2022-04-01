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
