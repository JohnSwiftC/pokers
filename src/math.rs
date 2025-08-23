// Determines new pot odds from bet and current pot
pub fn pot_odds(bet: u32, pot: u32) -> f64 {
    bet as f64 / (bet as f64 + pot as f64)
}

pub fn call_series_pot_odds(pot_odds: f64, iterations: u32) -> f64 {
    pot_odds / (iterations as f64 * pot_odds + 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_series_test() {
        let odds: f64 = pot_odds(100, 200);
        let test1 = {
            let mut new_odds = odds;
            let mut pot = 300;
            for _ in 0..10 {
                new_odds = pot_odds(100, pot);
                pot += 100;
            }

            new_odds
        };

        let test2 = call_series_pot_odds(odds, 10);

        println!("{}\n{}", test1, test2);
    }
}
