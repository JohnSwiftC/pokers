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

use yaml_rust::Yaml;
fn main() {
    let config = config::get_config();
    let doc = &config[0];

    if let Yaml::Hash(map) = doc {
        let iter = map.iter();
        for (sim, sim_cfg) in iter {
            if let Yaml::String(s) = sim {
                match s.as_str() {
                    "calling-table" => {
                        // Praying user doesnt use negatives, i guess it should be a given
                        let table_size = sim_cfg["table-size"].as_i64().unwrap() as u32;
                        let initial_bet = sim_cfg["initial-bet"].as_i64().unwrap() as u32;
                        let initial_pot = sim_cfg["initial-pot"].as_i64().unwrap() as u32;
                        let show_steps = sim_cfg["show-steps"].as_bool().unwrap_or(true);

                        println!("========================\nCalling Table");
                        calling_table(table_size, initial_bet, initial_pot, show_steps);
                        println!("========================");
                    }
                    &_ => eprintln!("Test '{}' is not a valid test", s),
                }
            }
        }
    }
}

// Shows how pot odds progress for a table as players continue to call
fn calling_table(table_size: u32, initial_bet: u32, initial_pot: u32, show_steps: bool) {
    let mut odds = math::pot_odds(initial_bet, initial_pot);
    if show_steps {
        for _ in 0..table_size {
            println!("[#] Call Made - Odds ({})", odds);
            odds = math::call_series_pot_odds(odds, 1);
        }
    } else {
        if table_size < 2 {
            println!("Table size not large enough");
        } else {
            odds = math::call_series_pot_odds(odds, table_size - 1);
            println!("[#] Call Made - Odds ({})", odds);
        }
    }
}