use crate::models::{Config, Todo};
use cw_storage_plus::{Item, Map};

pub const INDEX: Item<u64> = Item::new("index");

pub const TODOS: Map<u64, Todo> = Map::new("todos");

pub const CONFIG: Item<Config> = Item::new("config");
