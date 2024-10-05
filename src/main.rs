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

        println!("{}", command)
    }
}
