use uuid::Uuid;
use crate::data;

pub fn generate_id()-> String {

    let id = Uuid::new_v4();
    return id.to_string();
}

pub fn get_status(status: &data::Status) -> String {
    match status {
        data::Status::PENDING => return String::from("pending"),
        data::Status::SUCCESS => return String::from("success"),
        data::Status::CANCELED => return String::from("canceled"),
        data::Status::FAILED => return String::from("failed"),
    };
}
