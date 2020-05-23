use std::{
    error::Error,
    process::Command,
};

pub struct Config {
    pub args: Vec<String>,
    pub command: String,
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
