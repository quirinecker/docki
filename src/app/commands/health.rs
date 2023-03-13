use std::{process::Command, io::ErrorKind};
use colored::Colorize;

const INFO_ASCIIDOC: &str = "
Install the binary with your package manager!

sudo apt install asciidoctor
brew install asciidoctor
gem install asciidoctor
sudo pacman -Syu asciidoctor
yay -Syu asciidoctor
";

const INFO_REVEAL: &str = "
There are two options to install it:

Option 1:
- run `docki install-reveal

Option 2:
- Install the binary from Github https://github.com/asciidoctor/asciidoctor-reveal.js/releases
- Move the downloaded binary in a folder included in the path
- Make sure the binary is called asciidoctor-revealjs and not asciidoctor-revealjs-linux or similar
";

pub fn health() {
    check_asciidoc();
    check_reveal();
}

fn check_reveal() -> () {
    if reveal_is_installed() {
        print_health_ok("asciidoctor-revealjs")
    } else {
        print_health_not_ok("asciidoctor-revealjs", INFO_REVEAL)
    }
}

fn reveal_is_installed() -> bool {
    return check_command("asciidoctor-revealjs")
}

fn check_asciidoc() -> () {
    if asciidoc_is_installed() {
        print_health_ok("asciidoctor")
    } else {
        print_health_not_ok("asciidoctor", INFO_ASCIIDOC)
    }
}

fn asciidoc_is_installed() -> bool {
    return check_command("asciidoctor")
}

fn check_command(command: &str) -> bool {
    return match Command::new(command)
        .output() {
        Ok(_) => true,
        Err(e) => ErrorKind::NotFound != e.kind()
    }
}

fn print_health_ok(name: &str) {
    println!("- ✔️ {}", name.bright_green());
}

fn print_health_not_ok(name: &str, info: &str) {
    println!("- ❗{}", name.bright_red());
    println!("{}", info.bright_black())
}

