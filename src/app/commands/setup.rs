use std::{collections::HashMap, env, fs::File, io::Write, process};

use crate::app::fs_util;

use super::traits::Command;
use bytes::Bytes;
use colored::Colorize;

pub struct Health;

const REVEAL_VERSION: &str ="v5.0.0-rc.1";
const REVEAL_PATH: &str = "~/.docki/asciidoctor-reveal";

fn reveal_url(os: String) -> String {
    return format!("https://github.com/asciidoctor/asciidoctor-reveal.js/releases/download/{}/asciidoctor-revealjs-{}", REVEAL_VERSION, os)
}

impl Command for Health {
   fn execute(&self, _args: &HashMap<String, String>) -> Result<(), String> {
       Self::health();
       return Ok(())
   } 

   fn new() -> Self where Self: Sized {
       return Self {}
   }
}


impl Health {

    fn health() {
        println!("checking required softwar ... \n");
        let asciidoctor_installed = Self::asciidoctor_is_installed();
        let asciidoctor_reveal_installed = Self::asciidoctor_revealjs_is_installed();

        if asciidoctor_installed {
            println!("- ✔️ {}", "asciidoctor".green())
        } else {
            println!("- ❓{}", "asciidoctor \n".bright_red());
            Self::print_asciidoctor_install_help();
            println!("");
        }

        if asciidoctor_reveal_installed {
            println!("- ✔️ {}", "asciidoctor-revealjs".green())
        } else {
            println!("- ❓{}", "asciidoctor-revealjs \n".bright_red());
            Self::ask_to_install_reveal()
        }
    }

    fn ask_to_install_reveal() {
            print!("Do you want to install it ? (y/n)");
            let user_input: String = text_io::read!("{}\n");

            if user_input.to_lowercase() == "y" {
                let os = env::consts::OS;
                let url = reveal_url(os.to_string());
                println!("installing");
                let data = Self::donwload(&url).expect("failed installing");
                Self::save_to("~/.docki/asciidoctor-revealjs", data);
            } else if user_input.to_lowercase() == "y"{
                println!("not installing")
            } else {
                println!("not a valid option (not installing)")
            }
    }

    fn print_asciidoctor_install_help() {
            println!("you may want to install it with your package manager");
            println!("");
            println!("{}", "sudo apt install asciidoctor".yellow());
            println!("{}", "brew install asciidoctor".yellow());
            println!("{}", "sudo pacman -Syu asciidoctor".yellow());
            println!("{}", "yay -Syu asciidoctor".yellow());
            println!("{}", "dnf install asciidoctor".yellow());
    }

    fn asciidoctor_is_installed() -> bool {
        return process::Command::new("asciidoctor")
            .output()
            .is_ok()
    }

    fn asciidoctor_revealjs_is_installed() -> bool {
        return process::Command::new("asciidoctor-revealjs")
            .output()
            .is_ok()
    }

    fn donwload(url: &str) -> Result<Bytes, ()> {
        let Ok(response) = reqwest::blocking::get(url) else {
            return Err(());
        };

        let Ok(data) = response.bytes() else {
            return Err(());
        };

        return Ok(data)
    }

    fn save_to(path: &str, data: Bytes) -> () {
        let segments: &Vec<&str> = &path.split("/").collect();
        let parent_dir = &segments[0..segments.len() - 1].join("/");
        fs_util::create_dir_recursive(parent_dir);
        let mut file = File::create(path).expect("failed to create file");
        file.write_all(&data).expect("failed saving file")
    }
}

#[cfg(test)]
mod test {
    use std::env;


    #[test]
    fn download() {
        println!("{}", env::consts::OS);
        assert!(false)
    }
}
