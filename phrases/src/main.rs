extern crate phrases;

use phrases::english::{greetings,farewells};
use phrases::russian;

fn main() {
    println!("Hello in english: {hello}", hello=greetings::hello());
    println!("Goodbye in english: {}", farewells::goodbye());

    println!("Hello in russian: {0}", russian::hello());
    println!("Goodbye in russian: {}", russian::goodbye());
}
