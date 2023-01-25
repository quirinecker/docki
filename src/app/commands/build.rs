use std::{collections::HashMap, path::Path, fs};

use crate::app::{
    builder::{
        asciidoctor::{AsciiDoctorDocsBuilder, AsciiDoctorSlideBuilder},
        Builder,
    },
    fs_util,
};

use super::traits::Command;

pub struct Build {
    slides_builder: Box<dyn Builder>,
    docs_builder: Box<dyn Builder>,
}

impl Command for Build {
    fn execute(&self, _args: &HashMap<String, String>) -> Result<(), String> {
        let path = "./docs/".to_string();

        if !Self::directory_exists(&path) {
            return Self::docs_directory_missing();
        }

        let result = fs_util::fetch_paths_recursive(&path);

        let Ok(paths) = result else {
            return Err(result.unwrap_err())
        };

        for (index, path) in paths.iter().enumerate() {
            let progress = index + 1;
            let goal = paths.len();

            if path.ends_with(".adoc") && path.starts_with("./docs/slides") {
                let out_path = path.clone().replace("adoc", "html").replace("/docs/", "/dist/");
                self.build_slide(&path, &out_path, goal, progress)
            } else if path.ends_with(".adoc") {
                let out_path = path.clone().replace("adoc", "html").replace("/docs/", "/dist/");
                self.build_doc(&path, &out_path, goal, progress)
            } else {
                let out_path = path.clone().replace("/docs/", "/dist/");
                self.copy(&path, &out_path, goal, progress)
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
    fn build_file(&self, builder: &Box<dyn Builder>, in_path: &str, out_path: &str) -> Result<(), String> {
        return builder.build(&in_path, &out_path);
    }

    fn build_file_and_status(
        &self,
        builder: &Box<dyn Builder>,
        in_path: &str,
        out_path: &str,
        goal: usize,
        progress: usize,
        conversion_type: &str,
    ) {
        let result = self.build_file(builder, in_path, out_path);
        if result.is_ok() {
            Self::display_status(goal, progress, in_path, out_path, conversion_type)
        } else {
            Self::display_status(goal, progress, in_path, out_path, "error");
            let error = result.unwrap_err();
            println!("{error}");
        }
    }

    fn copy(&self, in_path: &str, out_path: &str, goal: usize, progress: usize) {
        let segments: &Vec<&str> = &out_path.split("/").collect();
        let parent_dir = &segments[0..segments.len() - 1].join("/");
        Self::create_dir_recursive(parent_dir);
        let result = fs::copy(in_path, out_path);
        if result.is_ok() {
            Self::display_status(goal, progress, in_path, out_path, "copy")
        } else {
            Self::display_status(goal, progress, in_path, out_path, "error");
            let error = result.unwrap_err();
            println!("{error}");
        }
    }

    fn create_dir_recursive(path: &str) {
        let mut validated_path = "./".to_string();
        for segment in path.split("/")  {
            validated_path.push_str(format!("{segment}/").as_str());
            if !Self::directory_exists(&validated_path) {
                fs::create_dir(&validated_path).unwrap()
            }
        }
    }

    fn display_status(goal: usize, progress: usize, in_path: &str, out_path: &str, conversion_type: &str) -> () {
        println!(
            "({} / {}) [{}] {} -> {}",
            progress,
            goal,
            conversion_type,
            in_path,
            out_path
        );
    }

    fn build_doc(&self, in_path: &str, out_path: &str, goal: usize, progress: usize) {
        self.build_file_and_status(&self.docs_builder, in_path, out_path, goal, progress, "doc");
    }

    fn build_slide(&self, path: &str, out_path: &str, goal: usize, progress: usize) {
        return self.build_file_and_status(&self.slides_builder, path, out_path, goal, progress, "slide");
    }

    fn directory_exists(path: &String) -> bool {
        Path::new(path).is_dir()
    }

    fn docs_directory_missing() -> Result<(), String> {
        return Err(
            "direcotry {path} was not found. The filesystem was maybe updated while build"
                .to_string(),
        );
    }
}
