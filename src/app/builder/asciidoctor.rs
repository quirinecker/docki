use std::process;

use super::Builder;

fn exec_command(command: &mut process::Command) -> Result<(), String> {
    let result = command.output();

    if let Ok(success) = result {
        if success.stderr.len() == 0 {
            return Ok(());
        } else {
            return Err(AsciiDoctorDocsBuilder::from_utf8(success.stderr));
        }
    } else {
        println!("{}", result.unwrap_err());
        return Err("asciidoctor not installed. You may need to run docki setup!".to_string());
    }
}

fn asciidoctor_docs(in_path: &str, out_path: &str) -> process::Command {
    let mut command = process::Command::new(format!("asciidoctor"));

    command
        .arg(format!("{in_path}"))
        .arg(format!("--out-file={out_path}"));

    return command;
}

fn asciidoctor_slides(in_path: &str, out_path: &str) -> process::Command {
    let mut command = process::Command::new(format!("asciidoctor-revealjs-linux"));
    let revealjs_path = path_between(out_path, "./dist/slides/revealjs");

    command
        .arg(format!("{in_path}"))
        .arg(format!("--out-file={out_path}"));

    return command;
}

fn path_between(from: &str, to: &str) -> &str {
    let from_segments = from.split("/");
    let to_segments = to.split("/");
    let last_common
}

pub struct AsciiDoctorDocsBuilder;

impl Builder for AsciiDoctorDocsBuilder {
    fn build(&self, in_path: &str, out_path: &str) -> Result<(), String> {
        let mut command = asciidoctor_docs(in_path, out_path);
        return exec_command(&mut command);
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
        let mut command = asciidoctor_slides(in_path, out_path);
        return exec_command(&mut command);
    }
}
