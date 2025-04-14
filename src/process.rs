use std::time::Instant;
use std::sync::{Arc, Mutex};
// use lettre::transport::smtp::Error;
use serde_json;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use crate::data::{Status, Worker};
// use env;
// use dotenv::dotenv;

pub fn launch_program(form: serde_json::Value, worker_ptr: Arc<Mutex<Worker>>) {
    let duration = 8.0;
    let now = Instant::now();
    // let _form = &form;
    let args_opt = form.get("args").and_then(|value| value.as_array());
    let args: Vec<String> = args_opt.unwrap().iter().filter_map(|value|value.as_str().map(|v| v.to_string())).collect();
    loop {
        if now.elapsed().as_secs_f64() >= duration {
            break;
        }
    }
    let mut worker = worker_ptr.lock().unwrap();
    let mut _bin = String::from("");
    match form["bin"].as_str() {
        Some(x) => {_bin = String::from(x)},
        None => {worker.status = Status::FAILED; worker.log = String::from("Failed to find bin"); panic!("Failed to find bin")}
    };
    let child = std::process::Command::new(_bin).args(&args).spawn().expect("Failed to launch program");
    let result = child.wait_with_output();
    match result {
        Ok(_)=> {worker.status = Status::SUCCESS},
        Err(_)=> {worker.status = Status::FAILED ;worker.log = String::from("Failed to launch program"); panic!("Failed to launch program")}
    }
}

pub fn send_email(form: serde_json::Value, worker_ptr: Arc<Mutex<Worker>>) {
    // let pass = std::env::var("APP_PASSWORD").expect("APP PASSWORD MUST BE SET.");
    let duration = 12.3;
    let now = Instant::now();
    let from = String::from(form["from"].as_str().unwrap());
    let to = String::from(form["to"].as_str().unwrap());
    let subject = String::from(form["subject"].as_str().unwrap());
    let body = String::from(form["body"].as_str().unwrap());

    
    loop {
        if now.elapsed().as_secs_f64() >= duration {
            break;
        }
    }
    let mut worker = worker_ptr.lock().unwrap();

    if from.len() == 0 || to.len() == 0 || subject.len() == 0 || body.len() == 0 {
        worker.log = String::from("Error: Bad value");
        worker.status = Status::FAILED;
    }
    let email = Message::builder()
    .from(from.parse().unwrap())
    .to(to.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_PLAIN)
    .body(body);
    match email {
        Ok(_) => {},
        Err(why) => {worker.log = String::from("Bad parsing"); worker.status = Status::FAILED; panic!("Bad parsing {}", why)}
    };
    let email_result = email.unwrap();
    let credentials = Credentials::new(from, std::env::var("APP_PASSWORD").expect("APP PASSWORD MUST BE SET."));
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(credentials)
        .build();
    match mailer.send(&email_result) {
        Ok(_) => {worker.status = Status::SUCCESS},
        Err(why)=> {worker.status = Status::FAILED; worker.log = String::from("Couldn't send email"); panic!("Couldn't send email: {why:?}")}
    }
}
