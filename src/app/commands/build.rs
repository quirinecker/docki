use std::{collections::HashMap, path::Path};

use crate::app::{
    builder::{asciidoctor::{AsciiDoctorDocsBuilder, AsciiDoctorSlideBuilder}, Builder},
    fs_util,
};

use super::traits::Command;

pub struct Build {
    slides_builder: Box<dyn Builder>,
    docs_builder: Box<dyn Builder>
}

impl Command for Build {
    fn execute(&self, _args: &HashMap<String, String>) -> Result<(), String> {
        let path = "./docs/".to_string();

        if !Self::docs_directory_exists(&path) {
            return Self::docs_directory_missing();
        }

        let result = fs_util::fetch_paths_recursive(&path, ".adoc");

        let Ok(paths) = result else {
            return Err(result.unwrap_err())
        };

        for (index, path) in paths.iter().enumerate() {
            if path.starts_with("./docs/slides") {
                if self.build_slide(&path).is_ok() {
                    println!(
                        "({} / {}) [{}] {} -> {}",
                        index,
                        paths.len(),
                        "slides",
                        path,
                        path.replace(".adoc", ".html")
                    );
                }
            } else {
                if self.build_doc(&path).is_ok() {
                    println!(
                        "({} / {}) [{}] {} -> {}",
                        index,
                        paths.len(),
                        "doc",
                        path,
                        path.replace(".adoc", ".html")
                    );
                }
            }
        }

        return Ok(());
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        return Build {
            slides_builder: Box::new(AsciiDoctorSlideBuilder {}),
            docs_builder: Box::new(AsciiDoctorDocsBuilder {}),
        };
    }
}

impl Build {
    fn build_file(&self, builder: &Box<dyn Builder>, path: &str) -> Result<(), String> {
        let out_path = path
            .clone()
            .replace("docs", "dist")
            .replace(".adoc", ".html");

        return builder.build(&path, &out_path);
    }

    fn build_doc(&self, path: &str) -> Result<(), String> {
        return self.build_file(&self.docs_builder, path);
    }

    fn build_slide(&self, path: &str) -> Result<(), String> {
        return self.build_file(&self.slides_builder, path);
    }

    fn docs_directory_exists(path: &String) -> bool {
        Path::new(path).is_dir()
    }

    fn docs_directory_missing() -> Result<(), String> {
        return Err(
            "direcotry {path} was not found. The filesystem was maybe updated while build"
                .to_string(),
        );
    }
}
