use super::traits::Command;

pub struct Serve;

impl Command for Serve {
    fn execute(&self, _args: &std::collections::HashMap<String, String>) -> Result<(), String> {
        println!("serving the application");
        return Ok(())
    }

    fn new() -> Self where Self: Sized {
        return Self {}
    }
}
