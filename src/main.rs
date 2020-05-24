use futures::executor::block_on;
use rell::{change_dir, run_command, Config};
use std::{
    io::{self, Write},
    process,
};

// Responsible for parsing logic, configuration and handling errors
fn main() {
    loop {
        // Give the user a prompt to enter a command
        print!("RELL%% ");

        // Flush standard out to make sure the prompt prints and is not stuck in the buffer
        io::stdout().flush().unwrap();

        // Read stdin to a string, but if that fails print the error message and
        // set a non-zero exit status to indicate to the calling process
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap_or_else(|err| {
            eprintln!("There was an error parsing your command {}", err);
            process::exit(1);
        });

        let config = Config::new(buffer).unwrap_or_else(|e| {
            eprintln!("{}", e);
            process::exit(1);
        });

        match config.command.as_ref() {
            ".quit" => return,
            "cd" => if let Err(e) = change_dir(config) {
				eprintln!("{}", e);
			},
            _ => if let Err(e) = block_on(run_command(config)) {
					eprintln!("{}", e);
				},
        }
    }
}
