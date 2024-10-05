use colored::Colorize;
use std::process::{Command, Stdio};

pub fn handle_native_cmd(command: &str, args: &[&str]) {
    let full_command = format!("{} {}", command, args.join(" "));

    let result = Command::new("cmd")
        .arg("/C")
        .arg(&full_command)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match result {
        Ok(status) if status.success() => {}
        Ok(status) => {
            println!(
                "{} exited with status code {}.",
                command.red().bold(),
                status.code().unwrap_or_default()
            );
        }
        Err(e) => println!(
            "{}: {}",
            format!("❌ Failed to execute '{}'", command).red(),
            e.to_string().yellow()
        ),
    }
}
