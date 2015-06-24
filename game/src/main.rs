extern crate game;

use game::character::Character;

fn main() {
    let player = Character::new("Nigel");
    let bandit = Character::new("Bandit");

    println!("The player's name is: {name}", name=player.get_name());
}
