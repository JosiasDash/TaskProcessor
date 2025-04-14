use std::time::Instant;
use exec;
// use lettre::transport::smtp::Error;
use serde_json;
use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
// use env;
// use dotenv::dotenv;

pub fn launch_program(form: serde_json::Value) {
    let duration = 8.0;
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

pub fn send_email(form: serde_json::Value) {
    // let pass = std::env::var("APP_PASSWORD").expect("APP PASSWORD MUST BE SET.");
    let duration = 12.3;
    let now = Instant::now();

    let from = String::from(form["from"].as_str().unwrap());
    let to = String::from(form["to"].as_str().unwrap());
    let subject = String::from(form["subject"].as_str().unwrap());
    let body = String::from(form["body"].as_str().unwrap());

    if from.len() == 0 || to.len() == 0 || subject.len() == 0 || body.len() == 0 {
        panic!("Error: Bad value");
    }

    loop {
        if now.elapsed().as_secs_f64() >= duration {
            break;
        }
    }
    let email = Message::builder()
    .from(from.parse().unwrap())
    .to(to.parse().unwrap())
    .subject(subject)
    .header(ContentType::TEXT_PLAIN)
    .body(body);
    match email {
        Ok(_) => {},
        Err(why) => {panic!("Bad parsing : {}", why)}
    };
    let email_result = email.unwrap();
    let credentials = Credentials::new(from, std::env::var("APP_PASSWORD").expect("APP PASSWORD MUST BE SET."));
    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .unwrap()
        .credentials(credentials)
        .build();
    match mailer.send(&email_result) {
        Ok(_) => println!("Mail sent successfully"),
        Err(why)=> panic!("Couldn't send email: {why:?}")
    }
}
