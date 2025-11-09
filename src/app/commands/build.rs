use super::executions::build_execution::BuildExecution;

pub async fn build(offline_reveal: bool) -> () {
    let mut build_execution = BuildExecution::new();
    build_execution.execute(offline_reveal).await.expect("build failed")
}
