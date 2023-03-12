use std::fs;

use self::asciidoctor::{build_doc, build_slide};

use super::fs_util;

pub mod asciidoctor;

pub trait Builder {
    fn build(&self, in_path: &str, out_path: &str) -> Result<(), String>;
}

pub fn docki_build(in_path: &str) -> DockiBuildResult {
    let out_path = in_path.replace("/docs/", "/dist/");
    let convert_out_path = out_path.replace(".adoc", ".html");

    if in_path.starts_with("./docs/slides/") && in_path.ends_with(".adoc") {
        if let Err(err) = build_slide(&in_path, &convert_out_path) {
            return DockiBuildResult::Err(err);
        }

        DockiBuildResult::Slide(convert_out_path)
    } else if in_path.ends_with(".adoc") {
        if let Err(err) = build_doc(&in_path, &convert_out_path) {
            return DockiBuildResult::Err(err);
        }

        DockiBuildResult::Doc(convert_out_path)
    } else {
        if let Err(err) = copy(&in_path, &out_path) {
            return DockiBuildResult::Err(err);
        }

        DockiBuildResult::Copy(out_path)
    }
}

fn copy(in_path: &str, out_path: &str) ->  Result<(), String> {
    let segments: &Vec<&str> = &out_path.split("/").collect();
    let parent_dir = &segments[0..segments.len() - 1].join("/");
    fs_util::create_dir_recursive(parent_dir);

    if let Err(err) = fs::copy(in_path, out_path) {
        return Err(err.to_string())
    }

    Ok(())
}

pub enum DockiBuildResult {
    Slide(String),
    Doc(String),
    Copy(String),
    Err(String),
}
