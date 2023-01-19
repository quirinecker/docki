use std::{collections::HashMap, env, fs, process};

use super::traits::Command;

pub struct Build;
impl Build {
    fn build_dir(&self, path: String) -> Result<(), String> {
        let Ok(dirs) = fs::read_dir(path) else {
           return Err("could not read file system".to_string());
       };

        for result in dirs {
            let Ok(entry) = result else {
               return Err("could not read entry".to_string());
           };

            if entry.path().is_dir() {
                self.build_dir(entry.path().to_str().unwrap().to_string());
            } else {
                self.build_file(entry.path().to_str().unwrap().to_string());
            }
        }

        return Err("No files to build".to_string());
    }

    fn build_file(&self, path: String) -> Result<(), String> {
        let out_path = path
            .clone()
            .replace("docs", "dist")
            .replace(".adoc", ".html");

        println!("asciidoctor {path} --out-file={out_path}");
        let _result = process::Command::new("asciidoctor")
            .arg(format!("{path}"))
            .arg(format!("--out-file={out_path}"))
            .output()
            .expect("fuck");

        dbg!(_result);

        return Ok(());
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

        return self.build_dir(path);
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        return Build {};
    }
}
