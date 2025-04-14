use json::JsonValue::Null;
// use json::{object, JsonValue};
use serde_json;
use crate::data::{self, Worker};
use std::sync::{Arc, Mutex};
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
pub fn create_task(task_type: String, form: serde_json::Value, worker: Arc<Mutex<Worker>>)-> std::thread::JoinHandle<()> {
    // let new_worker = Arc::new(Mutex::new(worker));
    if task_type == "program" {
        let task = std::thread::spawn(move || process::launch_program(form, worker));
        return task;
    } else {
        let task = std::thread::spawn(move || process::send_email(form));
        return task;
    }
}
