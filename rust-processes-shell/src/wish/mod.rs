pub struct Wish;

use std::fs::File;
use std::io::{self, prelude::*, BufReader, Write};

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

    pub fn run_with_file(&self, file_name: &str) -> u8 {
        let file = File::open(file_name);
        let Ok(file) = file else {
            println!("wish: cannot open file {}", file_name);
            return 1;
        };

        let reader = BufReader::new(file);

        for line in reader.lines() {
            let Ok(line) = line else {
                println!("wish: cannot read file {}", file_name);
                return 1;
            };
            match self.execute_line(&line) {
                ExecutionResult::Exit => return 0,
                ExecutionResult::Continue(code) => {
                    if code != 0 {
                        println!("wish: command failed with exit code {}", code);
                        return code;
                    } else {
                        continue;
                    }
                }
            }
        }
        0
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
