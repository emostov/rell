use std::{env, error::Error, path::Path, process::Command};

pub struct Config {
    pub command: String,
    pub args: Vec<String>,
}

impl Config {
    pub fn new(user_inputs: String) -> Result<Config, &'static str> {
        let mut user_input_vec = user_inputs.split_whitespace().map(|str| str.to_string());

        Ok(Config {
            // If a user does not give a command we give an error
            command: match user_input_vec.next() {
                Some(command) => command,
                None => return Err("Didn't get a command"),
            },

            // No need to match here because it is ok if we have an empty vec
            args: user_input_vec.collect::<Vec<String>>(),
        })
    }
}

pub async fn run_command(config: Config) -> Result<(), Box<dyn Error>> {
    // Create a new process of the give command (e.g. `ls`)
    let mut child = Command::new(config.command)
        .args(config.args) // Give the remaining args to the command
        .spawn()?; // Spawn the process and return an error if there is an issue

    child.wait()?;

    Ok(())
}

pub fn change_dir(config: Config) -> Result<(), Box<dyn Error>> {
    let base_path = "/".to_string();
    let path = if config.args.len() > 0 {
        &config.args[0]
    } else {
        &base_path
    };
    let next_dir = Path::new(path);
    env::set_current_dir(&next_dir)?;
    Ok(())
}
