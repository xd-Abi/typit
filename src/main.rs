use colored::Colorize;
use inquire::Text;

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
        let command = Text::new("")
            .with_placeholder("pwd gen 32")
            .prompt()
            .unwrap();

        let parts: Vec<&str> = command.split_whitespace().collect();

        match parts.get(0) {
            Some(&"exit") => {
                println!("{}", "👋 Goodbye!".bright_yellow().bold());
                break;
            }
            _ => println!(
                "{}",
                "❓ Unknown command! Use the --help command.".red().bold()
            ),
        }
    }
}
