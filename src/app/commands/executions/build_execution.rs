use std::{
    io::Cursor, path::PathBuf
};

use crate::app::{
    build::{DockiBuildResult, docki_build}, config::config::Config, fs_util::{self, create_dir_recursive}, log::display_status
};

pub struct BuildExecution {
    progress: usize,
    goal: usize,
	docs_dir: String
}

impl BuildExecution {
    pub fn new(docs_dir: &str) -> Self {
        return BuildExecution {
            progress: 0,
            goal: 0,
			docs_dir: docs_dir.to_string()
        };
    }

    pub async fn execute(&mut self, config: &Config) -> Result<(), String> {
        let path = self.docs_dir.to_string();

        if !fs_util::directory_exists(&path) {
            return Err(
                "docs directory does not exist it. Create it or use the template".to_string(),
            );
        }

        if let Err(error) = self.prepare(config.offline_reveal).await {
            return Err(error);
        }

        return self.build_dir(&path, config.offline_reveal);
    }


    async fn prepare(&self, offline_reveal: bool) -> Result<(), String> {
		if !offline_reveal {
			return Ok(())
		}

        let reveal_version = "5.2.1";
        let target = format!("https://github.com/hakimel/reveal.js/archive/{reveal_version}.zip");

        create_dir_recursive(format!("{}/slides", self.docs_dir).as_str());

		reqwest::get(target.clone()).await.unwrap();
        let Ok(response) = reqwest::get(target).await else {
            return Err("could not downlaod revealjs".to_string())
        };

        let Ok(bytes) = response.bytes().await else {
            return Err("could not extract bytes".to_string())
        };

        let out = PathBuf::from(format!("{}/slides/revealjs", self.docs_dir));

        if zip_extract::extract(Cursor::new(bytes), &out, true).is_err() {
            return Err("could not write extracted archive to disk".to_string());
        }

        return Ok(());
    }

    fn build_dir(&mut self, path: &str, offline_reveal: bool) -> Result<(), String> {
        let result = fs_util::fetch_paths_recursive(&path);

        let Ok(paths) = result else {
            return Err(result.unwrap_err())
        };

		let reveal_dir = format!("{}/slides/revealjs", path);
		let paths = paths.into_iter()
			.filter(|path| offline_reveal || !path.starts_with(reveal_dir.as_str()))
			.collect::<Vec<String>>();

		self.goal = paths.len();

        for (index, in_path) in paths.iter().enumerate() {
            self.progress = index + 1;
            let result = docki_build(&in_path, offline_reveal, &self.docs_dir);

            match result {
                DockiBuildResult::Err(err) => {
                    self.display_building_status("Error", &in_path, "");
                    println!("{}", err)
                },
                DockiBuildResult::Copy(out_path) => self.display_building_status("Copy", &in_path, &out_path),
                DockiBuildResult::Slide(out_path) => self.display_building_status("Slide", &in_path, &out_path),
                DockiBuildResult::Doc(out_path) => self.display_building_status("Doc", &in_path, &out_path),
            }
        }

        return Ok(());
    }

    fn display_building_status(&self, status_type: &str, in_path: &str, out_path: &str) -> () {
        let progress_str = format!("{} / {}", self.progress, self.goal);
        display_status(&progress_str, status_type, in_path, out_path);
    }

}
