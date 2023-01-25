use std::fs;

struct RecursivePathFetch {
    paths: Vec<String>,
    path: String,
}

impl RecursivePathFetch {
    pub fn new_with_extension_filter(path: String) -> Self {
        return Self {
            paths: vec![],
            path,
        };
    }

    pub fn fetch(&mut self) -> Result<Vec<String>, String> {
        if let Err(error) = self.read_dir(self.path.clone()) {
            return Err(error);
        } else {
            return Ok(self.paths.clone());
        }
    }

    fn read_dir(&mut self, path: String) -> Result<(), String> {
        let Ok(entries) = fs::read_dir(path) else {
            return self.dir_not_found();
        };

        for result in entries {
            let entry = result.unwrap();
            let path = entry.path();
            let str_path = path.to_str().unwrap();

            if path.is_file() {
                self.paths.push(str_path.to_string())
            } else if path.is_dir() {
                let read_result = self.read_dir(str_path.to_string());
                if read_result.is_err() {
                    return read_result;
                }
            }
        }

        return Ok(());
    }

    fn dir_not_found(&self) -> Result<(), String> {
        return Err(format!(
            "directory {} was not found or was changed while building",
            self.path
        ));
    }
}

pub fn fetch_paths_recursive(path: &str) -> Result<Vec<String>, String> {
    let mut path_fetch =
        RecursivePathFetch::new_with_extension_filter(path.to_string());

    return path_fetch.fetch();
}
