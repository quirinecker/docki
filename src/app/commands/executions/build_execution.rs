use std::{fs, path::{Path, PathBuf}, io::Cursor};

use crate::app::{
    builder::{
        asciidoctor::{AsciiDoctorDocsBuilder, AsciiDoctorSlideBuilder},
        Builder,
    },
    fs_util,
};

pub struct BuildExecution {
    progress: usize,
    goal: usize,
    doc_builder: Box<dyn Builder>,
    slide_builder: Box<dyn Builder>,
}

impl BuildExecution {
    pub fn new() -> Self {
        return BuildExecution {
            progress: 0,
            goal: 0,
            slide_builder: Box::new(AsciiDoctorSlideBuilder {}),
            doc_builder: Box::new(AsciiDoctorDocsBuilder {}),
        };
    }

    pub fn execute(&mut self) -> Result<(), String> {
        let path = "./docs/".to_string();

        if !Self::directory_exists(&path) {
            return Err("docs directory does not exist it. Create it or use the template".to_string())
        }

        if let Err(error) = Self::prepare() {
            return Err(error);
        } 

        return self.build_dir(&path);
    }

    fn build_file(
        &self,
        builder: &Box<dyn Builder>,
        in_path: &str,
        out_path: &str,
    ) -> Result<(), String> {
        return builder.build(&in_path, &out_path);
    }

    fn build_file_and_status(
        &self,
        builder: &Box<dyn Builder>,
        in_path: &str,
        out_path: &str,
        conversion_type: &str,
    ) {
        let result = self.build_file(builder, in_path, out_path);
        if result.is_ok() {
            self.display_status(in_path, out_path, conversion_type)
        } else {
            self.display_status(in_path, out_path, "error");
            let error = result.unwrap_err();
            println!("{error}");
        }
    }

    fn copy(&self, in_path: &str, out_path: &str) {
        let segments: &Vec<&str> = &out_path.split("/").collect();
        let parent_dir = &segments[0..segments.len() - 1].join("/");
        Self::create_dir_recursive(parent_dir);
        let result = fs::copy(in_path, out_path);
        if result.is_ok() {
            self.display_status(in_path, out_path, "copy");
        } else {
            self.display_status(in_path, out_path, "error");
            let error = result.unwrap_err();
            println!("{error}");
        }
    }

    fn create_dir_recursive(path: &str) {
        let mut validated_path = "./".to_string();
        for segment in path.split("/") {
            validated_path.push_str(format!("{segment}/").as_str());
            if !Self::directory_exists(&validated_path) {
                fs::create_dir(&validated_path).unwrap()
            }
        }
    }

    fn display_status(&self, in_path: &str, out_path: &str, conversion_type: &str) -> () {
        println!(
            "({} / {}) [{}] {} -> {}",
            self.progress, self.goal, conversion_type, in_path, out_path
        );
    }

    fn build_doc(&self, in_path: &str, out_path: &str) {
        self.build_file_and_status(&self.doc_builder, in_path, out_path, "doc");
    }

    fn prepare() -> Result<(), String> {
        let reveal_version = "3.9.2";
        let target = format!("https://github.com/hakimel/reveal.js/archive/{reveal_version}.zip");

        let Ok(response) = reqwest::blocking::get(target) else {
            return Err("could not downlaod revealjs".to_string())
        };

        let Ok(bytes) = response.bytes() else {
            return Err("could not extract bytes".to_string())
        };

        let out = PathBuf::from("./docs/slides/revealjs");

        if zip_extract::extract(Cursor::new(bytes), &out, true).is_err() {
            return Err("could not write extracted archive to disk".to_string());
        }

        return Ok(())
    }

    fn build_slide(&self, in_path: &str, out_path: &str) {
        self.build_file_and_status(&self.slide_builder, in_path, out_path, "slide");
    }

    fn directory_exists(path: &String) -> bool {
        Path::new(path).is_dir()
    }

    fn build_dir(&mut self, path: &str) -> Result<(), String> {
        let result = fs_util::fetch_paths_recursive(&path);

        let Ok(paths) = result else {
            return Err(result.unwrap_err())
        };

        for (index, path) in paths.iter().enumerate() {
            self.progress = index + 1;
            self.goal = paths.len();

            if path.ends_with(".adoc") && path.starts_with("./docs/slides") {
                let out_path = path
                    .clone()
                    .replace("adoc", "html")
                    .replace("/docs/", "/dist/");
                self.build_slide(&path, &out_path)
            } else if path.ends_with(".adoc") {
                let out_path = path
                    .clone()
                    .replace("adoc", "html")
                    .replace("/docs/", "/dist/");
                self.build_doc(&path, &out_path)
            } else {
                let out_path = path.clone().replace("/docs/", "/dist/");
                self.copy(&path, &out_path)
            }
        }

        return Ok(());
    }
}
