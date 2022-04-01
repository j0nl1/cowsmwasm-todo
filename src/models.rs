use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub enum Status {
    CLOSED = 0,
    OPEN = 1,
    COMPLETED = 2
}

impl Status {
    pub fn from(x: &u8) -> Status {
        match x {
            0 => Status::CLOSED,
            1 => Status::OPEN,
            2 => Status::COMPLETED,
            _ => Status::OPEN
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Todo {
    pub description: String,
    pub status: Status,
}