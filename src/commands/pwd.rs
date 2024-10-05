use colored::Colorize;
use rand::Rng;

pub fn handle_pwd(parts: &Vec<&str>) {
    match (parts.get(1), parts.get(2)) {
        (Some(&"gen"), Some(length_str)) => {
            if let Ok(length) = length_str.parse::<usize>() {
                let generated_password = generate_password(length);
                println!(
                    "{} {}",
                    "🔑 Generated Password:".bright_green().bold(),
                    generated_password
                );
            } else {
                println!(
                    "{}",
                    "❌ Please provide a valid number for password length."
                        .red()
                        .bold()
                );
            }
        }
        _ => println!(
            "{}",
            "📖 Usage: 'pwd gen <length>' - Generate a secure password with the given length."
                .white()
                .bold()
        ),
    }
}

fn generate_password(length: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                             abcdefghijklmnopqrstuvwxyz\
                             0123456789)(*&^%$#@!~";
    let mut rng = rand::thread_rng();
    let password: String = (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    password
}
