pub struct Wish;

use std::io::{self, Write};

enum ExecutionResult {
    Continue(u8),
    Exit,
}

impl Wish {
    pub fn new() -> Self {
        Wish
    }

    pub fn run(&self) -> u8 {
        loop {
            print!("wish> ");
            io::stdout().flush().unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).unwrap();

            match self.execute_line(&input) {
                ExecutionResult::Exit => break 0,
                ExecutionResult::Continue(code) => {
                    if code != 0 {
                        println!("wish: command failed with exit code {}", code);
                        break code;
                    } else {
                        continue;
                    }
                }
            }
        }
    }

    fn execute_line(&self, cmd: &str) -> ExecutionResult {
        let cmd = cmd.trim();
        println!("You entered: \"{}\"", cmd);
        if cmd == "exit" {
            println!("Exiting...");
            ExecutionResult::Exit
        } else {
            ExecutionResult::Continue(0)
        }
    }
}
