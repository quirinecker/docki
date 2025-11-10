use std::fs;

use self::asciidoctor::{build_doc, build_slide};

use super::fs_util;

pub mod asciidoctor;

pub fn docki_build(in_path: &str, offline_reveal: bool) -> DockiBuildResult {
    let out_path = in_path.replace("/docs/", "/dist/");
    let convert_out_path = out_path.replace(".adoc", ".html");

    if in_path.starts_with("./docs/slides/") && in_path.ends_with(".adoc") {
        if let Err(err) = build_slide(&in_path, &convert_out_path, offline_reveal) {
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

fn copy(in_path: &str, out_path: &str) -> Result<(), String> {
    fs_util::create_parent_dir_recursive(out_path);

    if let Err(err) = fs::copy(in_path, out_path) {
        return Err(err.to_string());
    }

    Ok(())
}

pub enum DockiBuildResult {
    Slide(String),
    Doc(String),
    Copy(String),
    Err(String),
}
