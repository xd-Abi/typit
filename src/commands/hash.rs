use clipboard::{ClipboardContext, ClipboardProvider};
use colored::Colorize;
use sha256::{digest, try_digest};

pub fn handle_hash(parts: &Vec<&str>) {
    match parts.get(1) {
        Some(&"str") => handle_string_hash(parts),
        Some(&"file") => handle_file_hash(parts),
        _ => println!(
            "{}",
            "📖 Usage: 'hash str <text>' or 'hash file <path>'"
                .white()
                .bold()
        ),
    }
}

fn handle_string_hash(parts: &Vec<&str>) {
    if let Some(&text) = parts.get(2) {
        let sha256_hash = digest(text);

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        ctx.set_contents(sha256_hash.clone()).unwrap();

        println!(
            "{} {}",
            "🔐 SHA-256 Hash (copied to clipboard):".white().bold(),
            sha256_hash.bright_green().bold(),
        );
    } else {
        println!("{}", "❌ Please provide a string to hash.".red().bold());
    }
}

fn handle_file_hash(parts: &Vec<&str>) {
    if let Some(file_path) = parts.get(2) {
        match try_digest(file_path) {
            Ok(hash) => {
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
                ctx.set_contents(hash.clone()).unwrap();

                println!(
                    "{} {}",
                    "🔐 File SHA-256 Hash:".white().bold(),
                    hash.bright_green().bold(),
                );
            }
            Err(_) => println!(
                "{}",
                "❌ Could not read the file. Please check the path."
                    .red()
                    .bold()
            ),
        }
    } else {
        println!("{}", "❌ Please provide a valid file path.".red().bold());
    }
}
