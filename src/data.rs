use std::thread::JoinHandle;
use json::JsonValue;

pub enum Status {
    PENDING,
    SUCCESS,
    CANCELED,
    FAILED
}

pub struct Worker {
    name: String,
    status: Status,
    thread: JoinHandle<(JsonValue)>,
    id: String,
}
