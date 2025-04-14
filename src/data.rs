use std::thread::JoinHandle;

pub enum Status {
    PENDING,
    SUCCESS,
    FAILED
}

pub struct Worker {
    pub name: String,
    pub status: Status,
    pub thread: Option<JoinHandle<()>>,
    pub id: String,
    pub log: String
}
