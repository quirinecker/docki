pub mod asciidoctor;

use std::{
    fs, io::Cursor, path::PathBuf
};

use crate::app::{
    build::{asciidoctor::build_slide, asciidoctor::build_doc}, config::config::Config, fs_util::{self, create_dir_recursive}, log::display_status
};

pub struct DockiBuilder<'a> {
    progress: usize,
    goal: usize,
	config: &'a Config
}

impl <'a> DockiBuilder <'a> {
    pub fn new(config: &'a Config) -> Self {
        return Self {
            progress: 0,
            goal: 0,
			config: config
        };
    }


	///
	/// Prepares everything for building the documentation
	///
	/// 1. Checks if the input directory exists and if not, returns an error
	/// 2. When offline_reveal is set to true, it downloads revealjs. When it fails, it returns an error
    pub async fn prepare(&self) -> Result<(), String> {
        if !fs_util::directory_exists(&self.config.input_dir) {
            return Err(
                "docs directory does not exist it. Create it or use the template".to_string(),
            );
        }

		if !self.config.offline_reveal {
			return Ok(())
		}

        let reveal_version = "5.2.1";
        let target = format!("https://github.com/hakimel/reveal.js/archive/{reveal_version}.zip");

        create_dir_recursive(format!("{}/slides", self.config.input_dir).as_str());

		reqwest::get(target.clone()).await.unwrap();
        let Ok(response) = reqwest::get(target).await else {
            return Err("could not downlaod revealjs".to_string())
        };

        let Ok(bytes) = response.bytes().await else {
            return Err("could not extract bytes".to_string())
        };

        let out = PathBuf::from(format!("{}/slides/revealjs", self.config.input_dir));

        if zip_extract::extract(Cursor::new(bytes), &out, true).is_err() {
            return Err("could not write extracted archive to disk".to_string());
        }

        return Ok(());
    }


	/// Builds all files in the input directory with a pretty output
    pub fn build_docs(&mut self) -> Result<(), String> {
        let result = fs_util::fetch_paths_recursive(&self.config.input_dir);
        let Ok(paths) = result else {
            return Err(result.unwrap_err())
        };

		let reveal_dir = format!("{}/slides/revealjs", self.config.input_dir);
		let paths = paths.into_iter()
			.filter(|path| self.config.offline_reveal || !path.starts_with(reveal_dir.as_str()))
			.collect::<Vec<String>>();

		self.goal = paths.len();

        for (index, in_path) in paths.iter().enumerate() {
            self.progress = index + 1;
            let result = self.build_file(&in_path);

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


	/// Builds a single file without a pretty output
	pub fn build_file(&self, path: &str) -> DockiBuildResult {
		let out_path = path.replace(&self.config.input_dir, &self.config.output_dir);
		let convert_out_path = out_path.replace(".adoc", ".html");

		if path.starts_with(format!("{}/slides/", &self.config.input_dir).as_str()) && path.ends_with(".adoc") {
			if let Err(err) = build_slide(&path, &convert_out_path, self.config) {
				return DockiBuildResult::Err(err);
			}

			DockiBuildResult::Slide(convert_out_path)
		} else if path.ends_with(".adoc") {
			if let Err(err) = build_doc(&path, &convert_out_path, self.config) {
				return DockiBuildResult::Err(err);
			}

			DockiBuildResult::Doc(convert_out_path)
		} else {
			if let Err(err) = Self::copy(&path, &out_path) {
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

    fn display_building_status(&self, status_type: &str, in_path: &str, out_path: &str) -> () {
        let progress_str = format!("{} / {}", self.progress, self.goal);
        display_status(&progress_str, status_type, in_path, out_path);
    }

}


/// Used for the Result of build_file. This way it is known what the builder did
pub enum DockiBuildResult {
    Slide(String),
    Doc(String),
    Copy(String),
    Err(String),
}
