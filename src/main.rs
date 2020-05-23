use std::{process::Command, io::{self, Write}};
// Rell, the rust shell

fn main() {

	loop {

		print!("rell% ");
		// Flush standard out to make sure the prompt prints and is not stuck in the buffer
		io::stdout().flush().unwrap();

		let mut input = String::new();

		// Read input to stdin into the string, input.
		io::stdin().read_line(&mut input).unwrap();

		// Trim the trailing new line character, split on whitespaces, and collect into a vec
		let mut commands: Vec<&str> = input.trim().split_whitespace().collect();
		println!("{:?}", commands);

		// Create a new process of the give command (e.g. `ls`)
		let mut child = Command::new(commands.remove(0))
			.args(commands) // Give the remaining args to the command
			.spawn() // Spawn the process
			.expect("There was a failure while trying to spawn a child process");

		child.wait().expect("There was a failure while waiting for the child process");
	}

}
