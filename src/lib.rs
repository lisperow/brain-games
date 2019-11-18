use std::io;

pub fn run() {
    println!("Welcome to the Brain Games!");
    println!("May I have your name?");

    let mut name = String::new();

    io::stdin().read_line(&mut name)
        .expect("Failed to read line");

    println!("Hello, {}", name);
}
