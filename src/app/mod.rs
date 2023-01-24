mod commands;
pub mod builder;
pub mod rx;
pub mod fs_util;

use std::collections::HashMap;

use commands::traits::Command;
use commands::CommandRegistry;

pub struct App {
    command_regisrty: CommandRegistry,
}

impl App {
    pub fn new() -> App {
        return App {
            command_regisrty: CommandRegistry::new()
        }
    }

    pub fn start(&self, args: Vec<String>) {
        let command_args = &args[1..];
        let mut path = String::from("");
        let mut argument_map = HashMap::new();
        let mut only_options_left = false;

        for (index, argument) in command_args.iter().enumerate() {
            if argument.starts_with("--") {
                only_options_left = true;
                let value = command_args.get(index + 1);
                if let Some(v) = value {
                    if v.starts_with("--") {
                        argument_map.insert(argument.replace("--", ""), String::from(""));
                    } else {
                        argument_map.insert(argument.replace("--", ""), String::from(v));
                    }
                } else {
                    argument_map.insert(argument.replace("--", ""), String::from(""));
                }
            } else if !only_options_left {
                path.push_str(&format!("/{argument}"))
            }
        }

        self.execute_path(&path, &argument_map);
    }

    fn execute_path(&self, path: &String, args: &HashMap<String, String>) {
        let command = self.command_regisrty.command_by(path);

        if let Some(c) = command {
            self.execute_command(c, args)
        } else {
            println!("command not found")
        }
    }

    fn execute_command(&self, c: &Box<dyn Command>, args: &HashMap<String, String>) {
        let result = c.execute(args);

        match result {
            Ok(_) => println!("successfully executed"),
            Err(message) => println!("{message}"),
        }
    }
}


