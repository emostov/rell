use std::{
    env,
    error::Error,
    path::Path,
    process::{Child, Command, Stdio},
};

#[derive(Debug)]
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

// Using a trimmed string of user input, generate command configs assuming adjacent commands pipe
/// into each other.
pub fn create_command_configs(user_inputs: String) -> Result<Vec<Config>, &'static str> {
    user_inputs
        .split("|")
        .map(|group| Config::new(group.to_string()))
        .collect::<Result<Vec<Config>, &'static str>>()
}

/// Run a single command or run a series of commands that are meant to be piped together
pub async fn run_pipeables(configs: Vec<Config>) -> Result<(), Box<dyn Error>> {
    let mut prev_command = None;

    for (pos, config) in configs.iter().enumerate() {
        let stdin = prev_command.map_or(Stdio::inherit(), |output: Child| {
            Stdio::from(output.stdout.unwrap())
        });

        let stdout = if pos == configs.len() - 1 {
            Stdio::inherit()
        } else {
            Stdio::piped()
        };

        let child = Command::new(&config.command)
            .stdin(stdin)
            .stdout(stdout)
            .args(&config.args)
            .spawn()?;

        prev_command = Some(child);
    }

    // TODO figure out a way to handle error better than unwrap
    // Wait for the last command to execute
    prev_command.unwrap().wait()?;

    Ok(())
}

/// Runs a command from a single config
pub async fn run_command(config: Config) -> Result<(), Box<dyn Error>> {
    // Create a new process of the give command (e.g. `ls`)
    let mut child = Command::new(config.command)
        .args(config.args) // Give the remaining args to the command
        .spawn()?; // Spawn the process and return an error if there is an issue

    // Wait for the spawned process to finish before continuing
    child.wait()?;

    Ok(())
}

/// change_dir changes the current working directory, the equivalent of `cd`
pub fn change_dir(config: &Config) -> Result<(), Box<dyn Error>> {
    let base_path = "/".to_string();

    // If no path is given is we give a base path, otherwise we use the user path
    let path = if config.args.len() > 0 {
        &config.args[0]
    } else {
        &base_path
    };

    // Create a Path - the Path struct deals with the paths of the underlying file system
    let next_dir = Path::new(path);

    // Change the current working directory to the next directory
    env::set_current_dir(&next_dir)?;

    Ok(())
}
