use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
    collections::HashMap,
    fs::{read_to_string, write},
    path::Path,
};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Todo {
    pub status: Status,
    pub task: String,
    pub id: u8,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone, Copy)]
pub enum Status {
    Completetd,
    Pending,
}

pub fn get_all_todos() -> HashMap<String, Vec<Todo>> {
    let file_path = Path::new("todos.json");
    let todos: Vec<Todo> = vec![];
    let data: HashMap<String, Vec<Todo>> = HashMap::from([(String::from("todos"), todos)]);
    if file_path.exists() {
        let file_str = read_to_string(file_path).unwrap();
        let data: HashMap<String, Vec<Todo>> = from_str(&file_str).unwrap();
        return data;
    } else {
        let json_str = to_string(&data).unwrap();
        write(file_path, json_str).unwrap()
    }
    data
}

pub fn add_todo(task: String) {
    let mut todos_data = get_all_todos();
    let mut empty_vector: Vec<Todo> = vec![];
    let todos = todos_data.get_mut("todos").unwrap_or(&mut empty_vector);
    let todos_length: u8 = todos.len().try_into().unwrap_or(0);
    let todo = Todo {
        id: todos_length + 1,
        task: task.clone(),
        status: Status::Pending,
    };
    todos.push(todo);
    save_todos(todos_data);
}

pub fn save_todos(todos_data: HashMap<String, Vec<Todo>>) {
    let file_path = Path::new("todos.json");
    let json_str = to_string(&todos_data).unwrap();
    write(file_path, json_str).unwrap()
}

pub fn update_todo(id: u8, new_task: Option<String>, is_completed: Option<bool>) {
    let mut todos_data = get_all_todos();
    let mut empty_vector: Vec<Todo> = vec![];
    let todos = todos_data.get_mut("todos").unwrap_or(&mut empty_vector);

    todos.iter_mut().for_each(|todo| {
        if id == todo.id {
            match is_completed.clone() {
                Some(value) => {
                    if value {
                        todo.status = Status::Completetd
                    } else {
                        todo.status = Status::Pending
                    }
                }
                None => {}
            }

            match new_task.clone() {
                Some(value) => todo.task = value,
                None => (),
            }

            println!("{todo:?}")
        }
    });

    save_todos(todos_data);
}

pub fn delete_todo(id: u8) {
    let mut todos_data = get_all_todos();
    let mut empty_todos: Vec<Todo> = vec![];
    let todos = todos_data.get_mut("todos").unwrap_or(&mut empty_todos);
    let index = id - 1;
    todos.remove(index.into());

    save_todos(todos_data);
}

pub fn get_todo(id: u8) -> Todo {
    let todos_data = get_all_todos();
    let todos = todos_data.get("todos").unwrap();
    let todo = todos.iter().find(|todo| todo.id == id).unwrap().clone();
    println!("{todo:?}");
    todo
}
