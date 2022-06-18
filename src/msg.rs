use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::models::{Status, Todo};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub owner: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    AddTodo {
        description: String,
    },
    UpdateTodo {
        id: u64,
        description: Option<String>,
        status: Option<Status>,
    },
    DeleteTodo {
        id: u64,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetTodoList {
        offset: Option<u64>,
        limit: Option<u64>,
    },
    GetTodo {
        id: u64
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TodoResponse {
    pub id: u64,
    pub description: String,
    pub status: Status,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct TodosResponse {
    pub todos: Vec<Todo>,
}
