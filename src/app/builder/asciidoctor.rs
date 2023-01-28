use std::{process, fmt::format};

use regex::Regex;

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
    let mut command = process::Command::new(format!("asciidoctor-revealjs"));
    let revealjs_path = path_between(out_path.to_string(), "./dist/slides/revealjs".to_string());

    command
        .arg(format!("{in_path}"))
        .arg(format!("-a revealjsdir={revealjs_path}"))
        .arg(format!("--out-file={out_path}"));

    return command;
}

pub fn path_between(from: String, to: String) -> String {
    let from_segments  = transform_input_to_clone_split(&from);
    let to_segments = transform_input_to_clone_split(&to);
    let last_matching_index = matching_from_start(&from_segments, &to_segments);
    let number_of_backs = from_segments.len() - last_matching_index;
    let mut path_between = path_back(number_of_backs);
    dbg!(&path_between);
    let path_to_to_path = &to_segments[last_matching_index..];
    path_between.push_str(&path_to_to_path.join("/"));
    return path_between; 
}

fn transform_input_to_clone_split(input: &String) -> Vec<String> {
    let regex = Regex::new(r"/$").unwrap();
    let first_transformation = input.clone().replace("./", "");
    return regex.replace_all(&first_transformation, "")
        .to_string().split("/")
        .collect::<Vec<&str>>()
        .iter().map(|s| s.to_string()).collect()
}

fn path_back(count: usize) -> String {
    let mut path = "".to_string();

    for _ in 0..count  {
       path.push_str("../");
    }

    return path;
}

pub fn matching_from_start(from_segments: &Vec<String>, to_segments: &Vec<String>) -> usize {
    for (index, from_segment) in from_segments.iter().enumerate() {
        if let Some(to_segment) = to_segments.get(index){
            if  from_segment != to_segment  {
                return index;
            }
        } else {
            return index;
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
