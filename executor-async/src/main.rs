use clap::{Arg, Command as ClapCommand};
use tokio::io::AsyncBufReadExt;
use tokio::task::JoinHandle;
use std::os::unix::process::ExitStatusExt;
use std::{
    error::Error,
    process::{exit, Stdio},
};

#[tokio::main]
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
    let mut process = tokio::process::Command::new(program[0])
        .args(&program[1..])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let pid = process.id();
    let displayed_name = format!("{:?} {}", program, pid.unwrap_or(0));
    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();
    // thread for stdout and stderr
    let name = displayed_name.clone();
    let stdout: JoinHandle<Result<(), tokio::io::Error>> = tokio::spawn(async move {
        let reader = tokio::io::BufReader::new(stdout);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            println!("{} stdout: {}", displayed_name, line);
        }
        Ok(())
    });
    let stderr: JoinHandle<Result<(), tokio::io::Error>> = tokio::spawn(async move {
        let reader = tokio::io::BufReader::new(stderr);
        let mut lines = reader.lines();
        while let Some(line) = lines.next_line().await? {
            println!("{} stderr: {}", name, line);
        }
        Ok(())
    });
    let _ = stdout.await?;
    let _ = stderr.await?;
    let return_code = process.wait().await?;
    exit(match return_code.code() {
        Some(code) => code,
        None => 128 + return_code.signal().unwrap(),
    });
}
