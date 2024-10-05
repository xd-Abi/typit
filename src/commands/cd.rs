use colored::Colorize;
use std::{env, path::Path};

pub fn handle_cd(parts: &[&str]) {
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
            println!("{}", "📖 Usage: 'cd <directory>'".white().bold());
        }
    }
}
