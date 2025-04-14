use std::thread::JoinHandle;
use serde_json;

pub enum Status {
    PENDING,
    SUCCESS,
    CANCELED,
    FAILED
}

pub struct Worker {
    pub name: String,
    pub status: Status,
    pub thread: Option<JoinHandle<()>>,
    pub id: String,
    pub log: String
}
