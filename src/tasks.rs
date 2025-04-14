use json::JsonValue::Null;
// use json::{object, JsonValue};
use serde_json;
use crate::data;
use std::thread::{self, JoinHandle};
use crate::process;
// Give TASK TYPE LIST
pub fn get_task_type()-> Vec<String> {
    let mut arr = Vec::new();
    arr.push(String::from("launch_program"));
    arr.push(String::from("send_email"));

    return arr;
}

// Task factory
pub fn create_task(task_type: String, form: serde_json::Value)-> std::thread::JoinHandle<()> {
    if task_type == "program" {
        let task = std::thread::spawn(|| process::launch_program(form));
        return task;
    } else {
        let task = std::thread::spawn(|| process::send_email(form));
        return task;
    }
}
