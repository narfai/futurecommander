use std::io::{stdin, stdout};

fn main() {
    loop {
        print!("> ");

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        println!("{}", command);
    }
}
