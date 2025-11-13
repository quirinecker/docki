use serde::Deserialize;

use crate::app::config::arguments::CommandArg;

#[derive(Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub port: u16,
    pub input_dir: String,
    pub offline_reveal: bool,
    pub output_dir: String,
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
            input_dir: args.input_dir.clone().unwrap_or(self.input_dir),
            output_dir: args.output_dir.clone().unwrap_or(self.output_dir),
            offline_reveal: args.offline_reveal || self.offline_reveal,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            port: 8080,
            input_dir: "./docs".to_string(),
            output_dir: "./dist".to_string(),
            offline_reveal: false,
        }
    }
}
