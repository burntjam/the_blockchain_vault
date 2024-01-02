use std::env;
use std::process::{Command, Child};
use std::thread;
use std::time::Duration;

fn start_program(program: &str) -> Option<Child> {
    match Command::new(program).spawn() {
        Ok(child) => {
            println!("Started {} with id: {}", program, child.id());
            Some(child)
        },
        Err(e) => {
            eprintln!("Failed to start {}: {}", program, e);
            None
        }
    }
}

pub fn main() {
    // Collect the command line arguments into a vector
    let args: Vec<String> = env::args().collect();

    // If there are no additional arguments beyond the program name, print usage and exit
    if args.len() <= 1 {
        eprintln!("Usage: {} <program1> <program2> ...", args[0]);
        return;
    }

    // Create a vector of children processes
    let mut children: Vec<(String, Option<Child>)> = args[1..]
        .iter()
        .map(|program| (program.clone(), start_program(program)))
        .collect();

    loop {
        // Sleep for a duration before checking child statuses
        thread::sleep(Duration::from_secs(5));

        for (program, child_opt) in &mut children {
            if let Some(child) = child_opt {
                match child.try_wait() {
                    Ok(Some(status)) => {
                        eprintln!("Program {} exited with {:?}", program, status);
                        *child_opt = start_program(program);
                    },
                    Ok(None) => {
                        // Still running, do nothing
                    },
                    Err(e) => {
                        eprintln!("Failed to check status of {}: {}", program, e);
                        *child_opt = start_program(program);
                    }
                }
            } else {
                // If the child was never started, try to start it again
                *child_opt = start_program(program);
            }
        }
    }
}
