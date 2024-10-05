use colored::Colorize;
use inquire::Text;
use std::{env, path::Path};

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

        let command = Text::new(&prompt_text).prompt().unwrap();

        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.get(0) {
            Some(&"calc") => commands::calc::handle_calc(&parts),
            Some(&"hash") => commands::hash::handle_hash(&parts),
            Some(&"mac") => commands::mac::handle_mac(),
            Some(&"pwd") => commands::pwd::handle_pwd(&parts),
            Some(&"cd") => commands::cd::handle_cd(&parts),
            Some(&"exit") => {
                println!("{}", "👋 Goodbye!".bright_yellow().bold());
                break;
            }
            Some(&cmd) => commands::native::handle_native_cmd(&cmd, &parts[1..]),
            None => println!(
                "{}",
                "❓ Unknown command! Use the --help command.".red().bold()
            ),
        }
    }
}
