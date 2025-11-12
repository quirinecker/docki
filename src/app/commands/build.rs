use crate::app::config::config::Config;

use super::executions::build_execution::BuildExecution;

pub async fn build(config: &Config) -> () {
    let mut build_execution = BuildExecution::new(config);

	build_execution.prepare().await.expect("could not prepare for build");
    build_execution.build_dir().expect("build failed")
}
