use std::process;

use super::Builder;

pub struct AsciiDoctorBuilder;

impl Builder for AsciiDoctorBuilder {
    fn build(&self, in_path: &str, out_path: &str) -> Result<(), String> {
        let result = process::Command::new("asciidoctor")
            .arg(format!("{in_path}"))
            .arg(format!("--out-file={out_path}"))
            .output();

        if let Ok(success) = result {
            if success.stderr.len() == 0 {
                return Ok(());
            } else {
                return Err(AsciiDoctorBuilder::from_utf8(success.stderr));
            }
        } else {
            return Err("command failed to execute".to_string());
        }
    }
}

impl AsciiDoctorBuilder {
    fn from_utf8(input: Vec<u8>) -> String {
        return match String::from_utf8(input) {
            Ok(m) => m,
            Err(e) => panic!("could not print error message: {}", e),
        };
    }
}
