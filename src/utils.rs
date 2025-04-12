use uuid::{uuid, Uuid};

pub fn generate_id()-> String {

    let id = Uuid::new_v4();
    return id.to_string();
}