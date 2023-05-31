pub struct Wish;

use std::io::{self, Write};

fn flush_stdout() {
    io::stdout().flush().expect("Cannot print to stdout");
}

impl Wish {
    pub fn new() -> Self {
        Wish
    }

    pub fn run(&self) {
        loop {
            print!("wish> ");
            //flush stdout
            flush_stdout();

            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            if input == "exit" {
                break;
            }
            println!("You entered: {}", input);
        }
    }
}
