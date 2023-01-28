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
    let revealjs_path = path_between(out_path.to_string(), "./dist/slides/revealjs".to_string());

    command
        .arg(format!("{in_path}"))
        .arg(format!("--out-file={out_path}"));

    return command;
}

fn path_between(from: String, to: String) -> String {
    let from_segments: Vec<&str> = from.split("/").collect();
    let to_segments: Vec<&str> = to.split("/").collect();
    let last_matching_index = last_matching_index(&from_segments, &to_segments);
    let number_of_backs = from_segments.len() - last_matching_index;
    return "".to_string();
}

pub fn last_matching_index(from_segments: &Vec<&str>, to_segments: &Vec<&str>) -> usize {
    for (index, from_segment) in from_segments.iter().enumerate() {
        if let Some(to_segment) = to_segments.get(index){
            if  from_segment != to_segment  {
                return index - 1;
            }
        } else {
            return index - 1;
        }
    }

    return from_segments.len();
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
