use std::io;

fn main() {
    println!("What's your name?");
    let mut name = String::new();

    io::stdin().read_line(&mut name)
    .ok()
    .expect("Failed to read line");

    println!("Your name is: {}", name);
}
