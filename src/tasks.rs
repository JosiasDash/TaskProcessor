use json::{object, JsonValue};
// Give TASK TYPE LIST
pub fn get_task_type()-> Vec<String> {
    let mut arr = Vec::new();
    arr.push(String::from("SET ALARM"));
    arr.push(String::from("LAUNCH PROGRAM"));
    arr.push(String::from("CLEAN REPOSITORY"));

    return arr;
}



// Task factory
pub fn create_task(task_type: String, form: JsonValue) {
    
}
