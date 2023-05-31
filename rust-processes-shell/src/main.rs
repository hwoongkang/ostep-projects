mod wish;
use std::env;
fn main() {
    let wish = wish::Wish::new();
    let args: Vec<String> = env::args().collect();

    match args.len() {
        0 => unreachable!("NO ARGUMENTS"),
        1 => wish.run(),
        2 => wish.run_with_file(&args[1]),
        _ => {
            println!("Too many arguments");
            1u8
        }
    };
}
