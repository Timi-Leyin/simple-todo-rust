use chrono::{DateTime, Utc};
use uuid::Uuid;
mod storage;
use storage::Storage;

use crate::constants::STORAGE_PATH;

#[derive(Debug)]
pub struct TodoType {
    uuid: Option<Uuid>,
    item: String,
    completed: bool,
    created_at: DateTime<chrono::Utc>,
    updated_at: DateTime<chrono::Utc>,
}

pub struct Todo {
    todos: Vec<TodoType>,
    storage: Storage
}

impl Todo {
    pub fn init(initial_value: Vec<TodoType>) -> Self {
        let st = Storage::new(STORAGE_PATH.to_string());
        let contents = &st.read();
        let parsed = &st.parse(&contents.to_string());
        Self {
            todos: initial_value,
            storage:st
        }
    }

    pub fn add(&mut self, item: String) {
        let uuid = Uuid::new_v4();
        let todo = TodoType {
            uuid: Some(uuid),
            item: item,
            completed: false,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        self.todos.push(todo);
        
        println!("Item added successfully");
    }

    pub fn get_all(&self) {
        let data = &self.todos;
        for ele in data.iter().enumerate() {
            println!("{}: {}", ele.0, ele.1.item);
        }
    }
}
