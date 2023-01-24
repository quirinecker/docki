use std::process;

use super::Builder;

fn asciidoctor(postfix: &str, in_path: &str, out_path: &str) -> Result<(), String> {
    let result = process::Command::new(format!("asciidoctor{postfix}"))
        .arg(format!("{in_path}"))
        .arg(format!("--out-file={out_path}"))
        .output();

    if let Ok(success) = result {
        if success.stderr.len() == 0 {
            return Ok(());
        } else {
            println!("something went wrong");
            return Err(AsciiDoctorDocsBuilder::from_utf8(success.stderr));
        }
    } else {
        println!("{}", result.unwrap_err());
        return Err("asciidoctor not installed. You may need to run docki setup!".to_string());
    }
}

pub struct AsciiDoctorDocsBuilder;

impl Builder for AsciiDoctorDocsBuilder {
    fn build(&self, in_path: &str, out_path: &str) -> Result<(), String> {
        return asciidoctor("", in_path, out_path);
    }
}

impl AsciiDoctorDocsBuilder {
    fn from_utf8(input: Vec<u8>) -> String {
        return match String::from_utf8(input) {
            Ok(m) => m,
            Err(e) => panic!("could not print error message: {}", e),
        };
    }
}

pub struct AsciiDoctorSlideBuilder;

impl Builder for AsciiDoctorSlideBuilder {
    fn build(&self, in_path: &str, out_path: &str) -> Result<(), String> {
        return asciidoctor("-revealjs-linux", in_path, out_path);
    }
}
