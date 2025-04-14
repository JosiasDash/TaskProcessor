// use tasks;
// mod tasks;
use std::io::{self, Write};
use serde_json::json;
// use crate::data::Axis;
use crate::data::Worker;
use crate::tasks;
use crate::data;
// use crate::utils;
use crate::utils::generate_id;
use crate::utils::get_status;

fn get_user_input(field: String) -> String {
    print!("Enter {}: ", field);
    io::stdout().flush().unwrap();
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Read data failed");
    if data.len() == 0 {
        println!("Error: invalid data {}", field);
        return get_user_input(field);
    }
    data.remove(data.len() - 1);
    return data;
}

fn email_prompt()-> serde_json::Value {
    let mut form = json!({});
    
    form["from"] = serde_json::Value::String(std::env::var("APP_EMAIL").expect("APP_EMAIL ENV VAR IS REQUIRED."));
    form["to"] = serde_json::Value::String(get_user_input(String::from("to >>> ")));
    form["subject"] = serde_json::Value::String(get_user_input(String::from("subject >>> ")));
    form["body"] = serde_json::Value::String(get_user_input(String::from("body >>> ")));
    return form;
}

fn launch_program_prompt()-> serde_json::Value {
    let mut form = json!({});

    form["bin"] = serde_json::Value::String(get_user_input(String::from("Binary path >>> ")));
    let args = get_user_input(String::from("Arguments >>>"));
    let args_array: Vec<&str> = args.split(" ").collect();
    let elems = args_array.iter().map(|v| serde_json::Value::String(String::from(*v))).collect();
    form["args"] = serde_json::Value::Array(elems);
    return form;
}

fn list_prompt() {
    let list = tasks::get_task_type();

    for elem in list.iter() {
        println!("\t{}", *elem);
    }
}

fn list_tasks(workers: &Vec<Worker>) {
    
    println!("TYPE\tSTATUS\tID");
    for worker in workers.iter() {
        println!("{}\t{}\t{}", worker.name, get_status(&worker.status), worker.id);
    }
}

fn manage_tasks(cmd: String) {
    let mut workers: Vec<data::Worker> = Vec::new();
    let mut _form = json!({});
    
    if cmd == "launch_program" {
        _form = launch_program_prompt();
        let result = tasks::create_task(String::from("program"), _form);
        let new_worker = Worker {
            name: String::from("Program"),
            status: data::Status::PENDING,
            thread: result,
            id: generate_id()
        };
        workers.push(new_worker);
    } else if  cmd == "send_email" {
        _form = email_prompt();
        let result = tasks::create_task(String::from("email"), _form);
        let new_worker = Worker {
            name: String::from("Email"),
            status: data::Status::PENDING,
            thread: result,
            id: generate_id()
        };
        workers.push(new_worker);
    } else if cmd == "tasks" {
        list_tasks(&workers);
    }
}

pub fn prompt() {
    loop {
        let data = get_user_input(String::from("prompt here> "));
        if data == "exit" || data == "Exit" {
            break;
        }
        if data == "list" {
            list_prompt();
        }
        manage_tasks(data);
    }
    println!("Bye 👋🏽");
}
