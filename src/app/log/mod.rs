use nu_ansi_term::Color::LightGreen;
use nu_ansi_term::Color::LightRed;
use nu_ansi_term::Style;

pub fn display_status(context1: &str, context2: &str, in_path: &str, out_path: &str) {
    let colored_context = if context2 == "Error" {
        LightRed.paint(context2)
    } else {
        LightGreen.paint(context2)
    };

    println!(
        "({}) [{}] {} -> {}",
        Style::new().paint(context1),
        colored_context,
        in_path,
        out_path
    );
}
