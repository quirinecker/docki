use colored::Colorize;

const INFO: &str = "
You can add completions for docki with the following methods. If you want the completions to be persistent add
them to a init file e.g. ~/.zshrc, ~/.bashrc, ~/.config/fish/config.fish.

Get Fish Completions
source ~/.docki/completions/docki.fish

Get Zsh Completions
source ~/.docki/completions/_docki

Get Bash Completions
source ~/.docki/completions/docki.bash
";

pub fn completions() {
    println!();
    println!("{}", "Completions".blue().bold());
    println!("{}", INFO.bright_black());
}
