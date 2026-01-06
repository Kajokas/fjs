use std::process::Command;

use crate::opts::build::build;

pub fn run() {
    build(Some("debug".into()));
    println!("Now running");

    let mut go_runner = Command::new("sh");
    go_runner.arg("-c").arg("go run ./build/main.go").output().expect("Failed to start server");
}
