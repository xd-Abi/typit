use clipboard::{ClipboardContext, ClipboardProvider};
use colored::Colorize;
use mac_address::get_mac_address;

pub fn handle_mac() {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

    match get_mac_address() {
        Ok(Some(mac)) => {
            ctx.set_contents(mac.to_string()).unwrap();
            println!(
                "{} {}",
                "🌐 Your Mac address (copied to clipboard):".white().bold(),
                mac.to_string().green().bold()
            )
        }
        Ok(None) => println!("{}", "❌ No MAC address was found".red().bold()),
        Err(_) => println!("{}", "❌ Error reading MAC address".red().bold()),
    }
}
