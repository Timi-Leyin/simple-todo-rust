use std::io::stdin;
mod actions;
mod constants;
use actions::Todo;
fn main() {
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
        } else if response == "2" {
            todo.get_all();
        } else if response == "3" {
            println!("Exiting ...");
            running = false;
        } else {
            println!("Invalid Choice, Choose - 1, 2 or 3");
        }
    }
}
