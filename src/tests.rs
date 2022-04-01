#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{
        mock_dependencies, mock_dependencies_with_balance, mock_env, mock_info,
    };
    use cosmwasm_std::{coins, from_binary};

    use crate::contract::{execute, instantiate, query};
    use crate::models::{Status, Todo};
    use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, TodosResponse};
    use crate::state::TODOS;

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balance(&coins(2, "token"));

        let msg = InstantiateMsg { index: 0 };
        let info = mock_info("creator", &coins(1000, "token"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
    }

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
        assert_eq!(
            todos.todos,
            vec![(0, open_todo), (1, closed_todo)]
        );
    }
}
