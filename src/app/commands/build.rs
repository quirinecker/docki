use std::{collections::HashMap, fs, path::Path};

use crate::app::builder::{asciidoctor::AsciiDoctorBuilder, Builder};

use super::traits::Command;

pub struct Build {
    builder: Box<dyn Builder>,
}

impl Command for Build {
    fn execute(&self, _args: &HashMap<String, String>) -> Result<(), String> {
        let path = format!("./docs/");
        let mut error_count = 0;

        if !self.docs_directory_exists(&path) {
            error_count += 1;
            println!(
                "docs directory does not exist. Either create it or clone the template from gitlab"
            )
        } else {
            for result in self.build_dir(&path) {
                match result {
                    Err(e) => {
                        error_count += 1;
                        println!("{e}");
                    }
                    Ok(()) => println!("success"),
                };
            }
        }

        if error_count > 0 {
            return Err(format!("failed with {} errors", error_count));
        }

        return Ok(());
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        return Build {
            builder: Box::new(AsciiDoctorBuilder {}),
        };
    }
}

impl Build {
    fn build_dir(&self, path: &str) -> Vec<Result<(), String>> {
        let mut results = vec![];
        let Ok(dirs) = fs::read_dir(path) else {
           return vec![Err(format!("direcotry {path} was not found. The filesystem was maybe updated while build"))]
       };

        for result in dirs {
            let Ok(entry) = result else {
               return vec![Err("could not read entry".to_string())];
            };

            let path = entry
                .path()
                .to_str()
                .expect("could not get text path")
                .to_string()
                .clone();

            if entry.path().is_dir() {
                results = [results, self.build_dir(&path)].concat()
            } else {
                results.push(self.build_file(&path));
            }
        }

        return results;
    }

    fn build_file(&self, path: &str) -> Result<(), String> {
        let out_path = path
            .clone()
            .replace("docs", "dist")
            .replace(".adoc", ".html");

        return self.builder.build(&path, &out_path);
    }

    fn docs_directory_exists(&self, path: &String) -> bool {
        Path::new(path).is_dir()
    }
}

