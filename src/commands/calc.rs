use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use colored::Colorize;
use meval::Expr;

pub fn handle_calc(parts: &Vec<&str>) {
    if parts.len() < 2 {
        println!(
            "{}",
            "📖 Usage: 'calc <expression>' - Evaluate a mathematical expression."
                .white()
                .bold()
        );
        return;
    }

    let expression = parts[1..].join(" ");

    match expression.parse::<Expr>() {
        Ok(expr) => match expr.eval() {
            Ok(result) => {
                let result_str = result.to_string();
                let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();

                if ctx.set_contents(result_str.clone()).is_ok() {
                    println!(
                        "{} {} = {}",
                        "🧮 Calculated Result (copied to clipboard):"
                            .bright_green()
                            .bold(),
                        expression,
                        result_str.bright_yellow().bold()
                    );
                } else {
                    println!("{}", "❌ Could not copy the result to clipboard.".red());
                }
            }
            Err(_) => println!("{}", "❌ Error evaluating the expression.".red()),
        },
        Err(_) => println!("{}", "❌ Invalid expression syntax.".red()),
    }
}
