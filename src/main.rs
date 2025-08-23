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
    HighCard { high: Card },
    Pair { high: Card },
    TwoPair { high: Card },
    ThreeOfAKind { high: Card },
    Straight { high: Card },
    Flush { high: Card },
    FullHouse { high: Card },
    FourOfAKind { high: Card },
    StraightFlush { high: Card },
}

pub mod config;
pub mod math;
fn main() {
    let config = config::get_config();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_series_test() {
        let odds: f64 = math::pot_odds(100, 200);
        let test1 = {
            let mut new_odds = odds;
            let mut pot = 300;
            for _ in 0..10 {
                new_odds = math::pot_odds(100, pot);
                pot += 100;
            }

            new_odds
        };

        let test2 = math::call_series_pot_odds(odds, 10);

        println!("{}\n{}", test1, test2);
    }
}
