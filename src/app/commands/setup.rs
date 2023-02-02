use std::collections::HashMap;

use super::traits::Command;

pub struct Setup;

impl Command for Setup {
   fn execute(&self, _args: &HashMap<String, String>) -> Result<(), String> {
       println!("setting up");
       return Ok(())
   } 

   fn new() -> Self where Self: Sized {
       return Self {}
   }
}
