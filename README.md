## Overview
TaskProcessor is a lightweight and efficient task management program written in Rust. It simplifies the processing of asynchronous tasks in your applications by providing a robust framework for queuing, scheduling, and executing tasks.

## Features
- **Asynchronous Task Handling**: Process tasks without blocking the main thread.
- **Customizable Scheduling**: Define task priorities and execution schedules.
- **Scalability**: Efficiently handle a large number of tasks.
- **Error Handling**: Built-in mechanisms for managing task failures and retries.
- **Extensibility**: Easily integrate with other systems or extend functionality.

## Installation
To use TaskProcessor in your Rust project, add the following to your `Cargo.toml`:
```toml
[dependencies]
taskprocessor = "0.1.0"
```

## Usage
Here’s a quick example to get started:
```rust
mod cli;
use dotenv::dotenv;
mod tasks;
mod data;
mod process;
mod utils;

fn main() {
    dotenv().ok();
    cli::prompt();
}
```
### Test
Here's is a quick example to test
```bash
cargo run
```
Command

```bash
launch_program # Launch a specific program
send_email # Send email to someone
list # List available type of tasks
tasks # List all tasks (TYPE STATUS ID)
exit # Quit program
```
To use email feature, the following env variables are mandatory
```bash
APP_PASSWORD= #Your google account application password
APP_EMAIL= #Your google account email address
```
By: JosiasDH