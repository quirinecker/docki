use crate::app::config::config::Config;

use super::executions::build_execution::BuildExecution;

pub async fn build(config: &Config) -> () {
    let mut build_execution = BuildExecution::new(&config.docs_dir);
    build_execution.execute(&config).await.expect("build failed")
}
