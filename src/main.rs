enum Suit {
    Spades,
    Diamonds,
    Clubs,
    Hearts,
}

// 1-13, 1 is ace
struct Card {
    value: u8,
    suit: Suit,
}

enum Hand {
    HighCard {high: Card},
    Pair {high: Card},
    TwoPair {high: Card},
    ThreeOfAKind {high: Card},
    Straight {high: Card},
    Flush {high: Card},
    FullHouse {high: Card},
    FourOfAKind {high: Card},
    StraightFlush {high: Card},
}

pub mod config;
fn main() {
    let config = config::get_config();
}
