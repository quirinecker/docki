use std::{fs::File, io::Write};

use crate::app::fs_util;

use super::traits::Command;

pub struct Reveal;

const ASCIIDOC_REVEAL_VERSION: &str= "v4.1.0-rc.5";

fn url() -> String {
    return format!("https://github.com/asciidoctor/asciidoctor-reveal.js/releases/download/{}/asciidoctor-revealjs-linux", ASCIIDOC_REVEAL_VERSION);
}

impl Command for Reveal {
    fn execute(&self, _args: &std::collections::HashMap<String, String>) -> Result<(), String> {
        Self::install_asciidocto_revealjs();
        return Ok(())
    }

    fn new() -> Self where Self: Sized {
        return Self {}
    }
}

impl Reveal {
    fn install_asciidocto_revealjs() -> () {
        let result = reqwest::blocking::get(url())
            .expect("Could not download reveal. Make sure you are connected to the internet");

        let binary = result.bytes().expect("could not get binary");

        let home_path = home::home_dir().expect("could not find home dir");
        let save_path = format!("{}/.docki/asciidoctor-revealjs", home_path.display());
        let save_dir = format!("{}/.docki", home_path.display());

        fs_util::create_dir_recursive(save_dir.as_str());

        let mut file = File::create(save_path).expect("could not save binary");
        file.write_all(&binary).expect("could not save binary");
    }

}
