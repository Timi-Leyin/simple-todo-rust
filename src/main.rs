use chrono::prelude::*;
use std::io::*;
use uuid::Uuid;
fn main() {
    // SIMPLE TODO LIST APPLICATION
    struct TodoType {
        uuid: Option<Uuid>,
        item: String,
        completed: bool,
        created_at: DateTime<chrono::Utc>,
        updated_at: DateTime<chrono::Utc>,
    }
    struct Todo {
        todos: Vec<TodoType>,
    }

    impl Todo {
        fn init(initial_value: Vec<TodoType>) -> Self {
            Self {
                todos: initial_value,
            }
        }

        fn add(&mut self, item: String) {
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

        fn get_all(&self) {
            let data = &self.todos;
            for ele in data.iter().enumerate() {
                println!("{}: {}", ele.0, ele.1.item);
            }
        }
    }

    let todos = Vec::new();
    let mut todo = Todo::init(todos);
    let mut running = true;

    while running {
        println!("Hey There !!!!");
        println!("1. Add item");
        println!("2. Get all items");
        println!("3. Exit");
        let mut response = String::new();
        stdin().read_line(&mut response).expect("ERROR OCCURRED!!!");
        response = response.trim().to_string();

        if response == "1" {
            println!("What do you want to do?");
            let mut new_todo = String::new();
            stdin().read_line(&mut new_todo).expect("ERROR OCCURRED!!!");
            new_todo = new_todo.trim().to_string();
            todo.add(new_todo);
        }
        else if response == "2" {
            todo.get_all();
        }
        else if response == "3" {
          println!("Exiting ...");
          running = false;
        }
        else{
            println!("Invalid Choice, Choose - 1, 2 or 3");

        }
    }
}
