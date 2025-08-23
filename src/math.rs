// Determines new pot odds from bet and current pot
pub fn pot_odds(bet: u32, pot: u32) -> f64 {
    bet as f64 / (bet as f64 + pot as f64)
}

pub fn call_series_pot_odds(pot_odds: f64, iterations: u32) -> f64 {
    pot_odds / (iterations as f64 * pot_odds + 1.0)
}
