use std::{
    io::Cursor,
    path::PathBuf
};

use crate::app::{
    build::{docki_build, DockiBuildResult},
    fs_util, log::display_status,
};

pub struct BuildExecution {
    progress: usize,
    goal: usize,
}

impl BuildExecution {
    pub fn new() -> Self {
        return BuildExecution {
            progress: 0,
            goal: 0,
        };
    }

    pub async fn execute(&mut self) -> Result<(), String> {
        let path = "./docs/".to_string();

        if !fs_util::directory_exists(&path) {
            return Err(
                "docs directory does not exist it. Create it or use the template".to_string(),
            );
        }

        if let Err(error) = Self::prepare().await {
            return Err(error);
        }

        return self.build_dir(&path);
    }


    async fn prepare() -> Result<(), String> {
        let reveal_version = "3.9.2";
        let target = format!("https://github.com/hakimel/reveal.js/archive/{reveal_version}.zip");

        let Ok(response) = reqwest::get(target).await else {
            return Err("could not downlaod revealjs".to_string())
        };

        let Ok(bytes) = response.bytes().await else {
            return Err("could not extract bytes".to_string())
        };

        let out = PathBuf::from("./docs/slides/revealjs");

        if zip_extract::extract(Cursor::new(bytes), &out, true).is_err() {
            return Err("could not write extracted archive to disk".to_string());
        }

        return Ok(());
    }

    fn build_dir(&mut self, path: &str) -> Result<(), String> {
        let result = fs_util::fetch_paths_recursive(&path);

        let Ok(paths) = result else {
            return Err(result.unwrap_err())
        };

        for (index, in_path) in paths.iter().enumerate() {
            self.progress = index + 1;
            self.goal = paths.len();
            let result = docki_build(&in_path);

            match result {
                DockiBuildResult::Err(err) => {
                    self.display_building_status("Error", in_path, "");
                    println!("{}", err)
                },
                DockiBuildResult::Copy(out_path) => self.display_building_status("Copy", &in_path, &out_path),
                DockiBuildResult::Slide(out_path) => self.display_building_status("Slide", &in_path, &out_path),
                DockiBuildResult::Doc(out_path) => self.display_building_status("Doc", &in_path, &out_path)
            }
        }

        return Ok(());
    }

    fn display_building_status(&self, status_type: &str, in_path: &str, out_path: &str) -> () {
        let progress_str = format!("{} / {}", self.progress, self.goal);
        display_status(&progress_str, status_type, in_path, out_path);
    }

}
