use std::process::{Command, Stdio};
use std::io::{BufRead, BufReader};

pub fn exec(command: String) {
    println!("$ {}", command);
    let mut child = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdout(Stdio::piped())
        .spawn()
        .expect("Failed to do the spawning task!");

    let stdout = child.stdout.take().unwrap();
    let reader = BufReader::new(stdout);

    for line in reader.lines() {
        println!("{}", line.unwrap());
    }
}