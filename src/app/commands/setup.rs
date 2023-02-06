use std::{collections::HashMap, env, fs::File, io::Write};

use super::traits::Command;
use bytes::Bytes;

pub struct Setup;

impl Command for Setup {
   fn execute(&self, _args: &HashMap<String, String>) -> Result<(), String> {
       Self::setup();
       return Ok(())
   } 

   fn new() -> Self where Self: Sized {
       return Self {}
   }
}


impl Setup {

    fn setup() {
        let os = env::consts::OS;

        let reveal_bin_url = format!("https://github.com/asciidoctor/asciidoctor-reveal.js/releases/download/v5.0.0-rc.1/asciidoctor-revealjs-{os}");
        let reveal_bin = Self::donwload(&reveal_bin_url).expect("could not download asciidoctor binary");
        let mut reveal_file = File::create("/usr/local/bin/asciidoctor").expect("could not save asciidoctor binary");
        reveal_file.write_all(&reveal_bin).expect("could not save asciidoctor binary");
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
