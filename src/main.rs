// use json::JsonValue;
use serde_json::json;

// use tasks::*;
// use data::*;
// use utils::*;
// mod tasks;
// mod data;
// mod utils;
mod process;
// use std::sync::mpsc;

fn main() {
    let elem = json!({
        "path": ".",
    });
    let th = std::thread::spawn(move || process::clean_repository(elem));

    let _res  = th.join();
    // let id = utils::generate_id();
    // println!("Hello, world! {}", id);
}
