use std::{collections::HashMap, env, fs};

use crate::app::builder::{Builder, asciidoctor::AsciiDoctorBuilder};

use super::traits::Command;

pub struct Build {
    builder: Box<dyn Builder>
}

impl Build {
    fn build_dir(&self, path: String) -> Vec<Result<(), String>> {
        let mut results = vec![];
        let Ok(dirs) = fs::read_dir(path) else {
           return vec![Err("could not read file system".to_string())]
       };

        for result in dirs {
            let Ok(entry) = result else {
               return vec![Err("could not read entry".to_string())];
            };

            let path = entry.path().to_str()
                .expect("could not get text path");

            if entry.path().is_dir() {
                results = [
                    results, vec![self.build_file(&path)]
                ].concat()
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
}

impl Command for Build {
    fn execute(&self, _args: &HashMap<String, String>) -> Result<(), String> {
        let Ok(project_cwd_object) = env::current_dir() else {
           return Err("current dirctory does not seem to exist".to_string())
       };

        let Some(project_cwd) = project_cwd_object.to_str() else {
           return Err("invalid path".to_string());
       };

        let path = format!("{project_cwd}/docs/");
        let mut error_count = 0;

        for result in self.build_dir(path) {
            match result {
                Err(e) => {
                    error_count += 1;
                    println!("{e}");
                },
                Ok(()) => println!("success")
            };
        }

        if error_count > 0{
            return Err(format!("failed with {} errors", error_count))
        }

        return Ok(());
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        return Build {
            builder: Box::new(AsciiDoctorBuilder {})
        };
    }
}
