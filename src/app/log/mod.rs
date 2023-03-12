use colored::Colorize;

pub fn display_status(context1: &str, context2: &str, in_path: &str, out_path: &str) {
    let colored_context = color_context(context2);
    println!(
        "({}) [{}] {} -> {}",
        context1.bold(),
        colored_context,
        in_path,
        out_path
    );
}

fn color_context(context: &str) -> colored::ColoredString {
    if context == "Error" {
        return context.bright_red()
    } else {
        return context.bright_green()
    }
}
