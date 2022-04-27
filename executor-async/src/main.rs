use async_std::{
    io::{prelude::BufReadExt, BufReader},
    prelude::StreamExt,
    process::{exit, Command, Stdio},
};
use clap::{Arg, Command as ClapCommand};
use std::error::Error;
use std::os::unix::process::ExitStatusExt;

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let matches = ClapCommand::new("executor")
        .trailing_var_arg(true)
        .allow_hyphen_values(true)
        .arg(
            Arg::new("shell")
                .long("shell")
                .help("Use shell to run cmd")
                .takes_value(false),
        )
        .arg(Arg::new("cmd").multiple_values(true).required(true))
        .get_matches();
    let mut program: Vec<&str> = matches.values_of("cmd").unwrap().collect();
    let joined;
    if matches.is_present("shell") {
        joined = program.join(" ");
        program = vec!["/bin/sh", "-c", joined.as_str()];
    }
    let mut process = Command::new(program[0])
        .args(&program[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let pid = process.id();
    let displayed_name = format!("{:?} {}", program, pid);
    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();
    // thread for stdout and stderr
    let name = displayed_name.clone();
    let stdout = async_std::task::spawn(async move {
        let reader = BufReader::new(stdout);
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| {
                println!("{} stdout: {}", displayed_name, line);
            }).await;
    });
    let stderr = async_std::task::spawn(async move {
        let reader = BufReader::new(stderr);
        reader
            .lines()
            .filter_map(|line| line.ok())
            .for_each(|line| {
                println!("{} stderr: {}", name, line);
            }).await;
    });
    stdout.await;
    stderr.await;
    let return_code = process.status().await?;
    exit(match return_code.code() {
        Some(code) => code,
        None => 128 + return_code.signal().unwrap(),
    });
}
