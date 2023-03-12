use super::executions::build_execution::BuildExecution;

pub async fn build() -> () {
    let mut build_execution = BuildExecution::new();
    build_execution.execute().await.expect("build failed")
}
