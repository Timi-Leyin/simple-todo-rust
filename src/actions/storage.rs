use super::TodoType;
use std::{fs, str::FromStr};

use chrono::Utc;
use uuid::Uuid;
pub struct Storage {
    path: String,
}

impl Storage {
    pub fn new(path: String) -> Self {
        Self { path }
    }

    pub fn parse(&self, content: &String) {
        // println!("PARSING ... {content}");
        let rows = content.split("\n").enumerate();
        let mut parsed_todos:Vec<TodoType> = Vec::new()  ;
        for (index, row) in rows {
            if index > 0 {
                let raw_cols = &row.to_string();
                let cols = raw_cols.split(",").enumerate();
                for (col_index, col) in cols {
                    let mut todo = TodoType {
                        uuid: Some(Uuid::new_v4()),
                        item:  String::new(),
                        completed: false,
                        created_at: Utc::now(),
                        updated_at: Utc::now(),
                    };
                    if col_index == 0 {
                        todo.uuid = Some(Uuid::new_v4());
                    } else if col_index == 1 {
                        todo.item = col.to_string();
                    } else if col_index == 2 {
                        todo.completed = col.to_string() == "true";
                    } else if col_index == 3 {
                        todo.created_at = Utc::now();
                    } else if col_index == 4 {
                        todo.created_at = Utc::now();
                    }
                    parsed_todos.push(todo);
                }
           
            }
            print!("FROM FILE {:?}", parsed_todos);
        }
        // TODO:
        // 1. Get each line;
        // 2. Get the top line
        // 3. Split the row by "," to get each row
        // 4. group the columns approprialty
        // 5. Return as a struct
    }

    pub fn read(&self) -> String {
        let f = fs::read_to_string(&self.path).expect("AN ERROR OCCURRED");
        return f;
    }

    pub fn insert(self, value: String) {}
}
