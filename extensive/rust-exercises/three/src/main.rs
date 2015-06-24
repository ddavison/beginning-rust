use std::io;

fn main() {
    println!("What's your name?");
    let mut name = String::new();

    io::stdin().read_line(&mut name)
    .ok()
    .expect("Failed to read line");

    match name {
        "Alice".to_string() => greet(name),
        _       => { println!("Not allowed.. was {}", name) }
    }
}

fn greet(name: String) {
    println!("Your name is: {}", name);
}
