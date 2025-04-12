use std::time::Instant;
use exec;
use serde_json;

pub fn launch_program(form: serde_json::Value) {
    let duration = 3.0;
    let now = Instant::now();
    // let _form = &form;
    let mut _bin = String::from("");
    match form["bin"].as_str() {
        Some(x) => {_bin = String::from(x)},
        None => {panic!("Failed to find bin")}
    };
    let args: Vec<String> = match form.get("args").and_then(|value| value.as_array()) {
        Some(arr) => arr
        .iter()
        .filter_map(|value| value.as_str().map(|v| v.to_string())).collect(),
        None => panic!("Failed to get args")
    };
    loop {
        if now.elapsed().as_secs_f64() >= duration {
            break;
        }
    }
    let err = exec::Command::new(_bin).args(&args).exec();
    println!("End of process {}", err);
}

pub fn clean_repository(form: serde_json::Value) {
    let duration = 5.0;
    let now = Instant::now();
    let mut _path = String::new();

    match form["path"].as_str() {
        Some(x)=> {_path = String::from(x)},
        None => panic!("Failed to get path")
    };
    loop {
        if now.elapsed().as_secs_f64() >= duration {
            break;
        }
    }
    let mut args = Vec::new();
    args.push(_path.clone() + "/*.tmp");
    args.push(_path.clone() + "/*~");
    args.push(_path.clone() + "/*#");
    args.push(String::from("-f"));
    let err = exec::Command::new("rm").exec();
    println!("CLEAN REPOSITORY ERROR: {}", err);
}
