use super::executions::build_execution::BuildExecution;

pub fn build() -> () {
    let mut build_execution = BuildExecution::new();
    build_execution.execute().expect("build failed")
}
