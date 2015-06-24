enum Suits {
    Spades,
    Clubs,
    Hearts,
    Diamonds,
}

enum Values {
    Ace, // ace
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack, // jack
    Queen, // queen
    King, // king
}

struct Card {
    value: String,
    suit: String,
}

impl Card {
    fn new(value: &str, suit: &str) -> Card {
        Card {
            value: value.to_string(),
            suit: suit.to_string(),
        }
    }
}

fn main() {
    let AS = Card::new(Values::Ace, Suits::Spades);

}
