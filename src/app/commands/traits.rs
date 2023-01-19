use std::collections::HashMap;

pub trait Command {
    fn execute(&self, args: &HashMap<String, String>) -> Result<(), String>;
    fn new() -> Self where Self: Sized;
} 



