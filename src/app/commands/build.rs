use std::collections::HashMap;

use super::{executions::build_execution::BuildExecution, traits::Command};

pub struct Build;

impl Command for Build {
    fn execute(&self, _args: &HashMap<String, String>) -> Result<(), String> {
        let mut build_execution = BuildExecution::new();
        return build_execution.execute();
    }

    fn new() -> Self
    where
        Self: Sized,
    {
        return Build {}
    }
}
