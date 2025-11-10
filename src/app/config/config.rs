use serde::Deserialize;

use crate::app::config::arguments::CommandArg;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
	pub port: u16,
	pub docs_dir: String,
	pub offline_reveal: bool,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {

        let s = config::Config::builder()
            .add_source(config::File::with_name("./docki.config.toml"))
            .build()?;
        s.try_deserialize()
    }

	pub fn merge_with_args(self, args: &super::arguments::Args) -> Self {
		Self {
			port: match args.command {
				CommandArg::Serve { port } => port.unwrap_or(self.port),
				_ => self.port,
			},
			docs_dir: args.docs_dir.clone().unwrap_or(self.docs_dir),
			offline_reveal: {
				if let CommandArg::Build { offline_reveal } = args.command {
					offline_reveal
				} else {
					self.offline_reveal
				}
			}
		}
	}
}

impl Default for Config {
	fn default() -> Self {
		Self {
			port: 8080,
			docs_dir: "./docs".to_string(),
			offline_reveal: false,
		}
	}
}
