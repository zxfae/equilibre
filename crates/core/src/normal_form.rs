//! Two-player games in normal form: strategies and a payoff matrix.

/// Which player we look at: the one choosing a row, or the one choosing a column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Player {
    Row,
    Column,
}

/// A two-player game in normal form.
///
/// `payoffs[r][c]` is `(row payoff, column payoff)` when the row player
/// plays strategy `r` and the column player plays strategy `c`.
#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub row_strategies: Vec<String>,
    pub col_strategies: Vec<String>,
    pub payoffs: Vec<Vec<(f64, f64)>>,
}

/// True if, for `player`, strategy `a` pays strictly more than strategy `b`
/// against every strategy of the opponent.
pub fn strictly_dominates(game: &Game, player: Player, a: usize, b: usize) -> bool {
    match player {
        Player::Row => {
            for c in 0..game.col_strategies.len() {
                let gain_a = game.payoffs[a][c].0;
                let gain_b = game.payoffs[b][c].0;

                if gain_a <= gain_b {
                    return false;
                }
            }
        }
        Player::Column => {
            for row in &game.payoffs {
                let gain_a = row[a].1;
                let gain_b = row[b].1;

                if gain_a <= gain_b {
                    return false;
                }
            }
        }
    }
    true
}

/// True if, for `player`, strategy `a` never pays less than strategy `b`
/// and pays strictly more against at least one strategy of the opponent.
pub fn weakly_dominates(game: &Game, player: Player, a: usize, b: usize) -> bool {
    let mut strictly_better = false;

    match player {
        Player::Row => {
            for c in 0..game.col_strategies.len() {
                let gain_a = game.payoffs[a][c].0;
                let gain_b = game.payoffs[b][c].0;

                if gain_a > gain_b {
                    strictly_better = true;
                }
                if gain_a < gain_b {
                    return false;
                }
            }
        }
        Player::Column => {
            for row in &game.payoffs {
                let gain_a = row[a].1;
                let gain_b = row[b].1;

                if gain_a > gain_b {
                    strictly_better = true;
                }
                if gain_a < gain_b {
                    return false;
                }
            }
        }
    }
    strictly_better
}
