use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        let command = args[1].clone(); // cannot move, so use clone
        match command {
            _hello => println!("Hello there!"),
        }
    }
}