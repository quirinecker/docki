use std::collections::HashMap;

use traits::Command;

use self::build::Build;

pub mod traits;
pub mod executions;
mod build;

pub struct CommandRegistry {
    commands: HashMap<String,  Box<dyn Command>>
}

impl CommandRegistry {

    pub fn register_all(&mut self) {
        let registry = self;
        registry.register("/build".to_string(), Box::new(Build::new()), true)
    }

    pub fn register(&mut self, path: String, command: Box<dyn Command>, enabled: bool) {
        if enabled {
            self.commands.insert(path, command);
        }
    }

    pub fn new() -> CommandRegistry {
        let mut registry = CommandRegistry { commands: HashMap::new() };

        registry.register_all();

        registry
    }

    pub fn command_by(&self, path: &String) -> Option<&Box<dyn Command>> {
        let command = self.commands.get(path);
        return command;
    }
}

