use colored::Colorize;
use inquire::Text;
use std::{
    env,
    path::Path,
    process::{Command, Stdio},
};

mod commands;

fn main() {
    let banner = r#"

                ████████╗██╗   ██╗██████╗ ██╗████████╗                
                ╚══██╔══╝╚██╗ ██╔╝██╔══██╗██║╚══██╔══╝                
█████╗█████╗       ██║    ╚████╔╝ ██████╔╝██║   ██║       █████╗█████╗
╚════╝╚════╝       ██║     ╚██╔╝  ██╔═══╝ ██║   ██║       ╚════╝╚════╝
                   ██║      ██║   ██║     ██║   ██║                   
                   ╚═╝      ╚═╝   ╚═╝     ╚═╝   ╚═╝                   

               Your personal command-line assistant 🤖!
"#;

    println!("{}", banner.bright_cyan().bold());

    loop {
        let current_dir = env::current_dir().unwrap_or_else(|_| Path::new("C:\\").to_path_buf());
        let prompt_text = format!("{}> ", current_dir.display()).bright_black().bold();

        let command = Text::new(&prompt_text)
            .with_placeholder("pwd gen 32")
            .prompt()
            .unwrap();

        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.get(0) {
            Some(&"calc") => commands::calc::handle_calc(&parts),
            Some(&"hash") => commands::hash::handle_hash(&parts),
            Some(&"mac") => commands::mac::handle_mac(),
            Some(&"pwd") => commands::pwd::handle_pwd(&parts),
            Some(&"cd") => {
                change_directory(&parts);
            }
            Some(&"exit") => {
                println!("{}", "👋 Goodbye!".bright_yellow().bold());
                break;
            }
            Some(&cmd) => {
                run_native_command(cmd, &parts[1..]);
            }
            None => println!(
                "{}",
                "❓ Unknown command! Use the --help command.".red().bold()
            ),
        }
    }
}

fn change_directory(parts: &[&str]) {
    match parts.get(1) {
        Some(&path) => {
            if let Err(e) = env::set_current_dir(Path::new(path)) {
                println!(
                    "{}: {}",
                    "❌ Failed to change directory".red(),
                    e.to_string().yellow()
                );
            }
        }
        None => {
            println!(
                "{}",
                "❌ Please provide a path. Usage: cd <directory>"
                    .red()
                    .bold()
            );
        }
    }
}

fn run_native_command(command: &str, args: &[&str]) {
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
