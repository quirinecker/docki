mod commands;
pub mod builder;
pub mod fs_util;
mod args;

use std::collections::HashMap;
use std::env;

use commands::traits::Command;
use commands::CommandRegistry;

use self::args::args;

pub struct App {
    command_regisrty: CommandRegistry,
}

impl App {
    pub fn new() -> App {
        return App {
            command_regisrty: CommandRegistry::new()
        }
    }

    pub fn start(self, old_args: Vec<String>) {
        let args = args();
        Self::preapare_env_path();
        let command_args = &old_args[1..];
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

    fn preapare_env_path() {
        env::set_var("PATH", fs_util::docki_path_env());
    }

    fn execute_path(self, path: &String, args: &HashMap<String, String>) {
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
            Err(message) => println!("{message}")
        }
    }
}


