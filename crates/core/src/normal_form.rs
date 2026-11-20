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
    let n = match player {
        Player::Row => game.col_strategies.len(),
        Player::Column => game.row_strategies.len(),
    };
    let all: Vec<usize> = (0..n).collect();
    strictly_dominates_among(game, player, a, b, &all)
}

/// True if, for `player`, strategy `a` pays strictly more than strategy `b`
/// against every strategy of the opponent listed in `opponents`
/// (the ones still in play during iterated elimination).
pub fn strictly_dominates_among(
    game: &Game,
    player: Player,
    a: usize,
    b: usize,
    opponents: &[usize],
) -> bool {
    match player {
        Player::Row => {
            // Only the opponent's strategies still in play: those already
            // eliminated must not count in the comparison.
            for &c in opponents {
                let gain_a = game.payoffs[a][c].0;
                let gain_b = game.payoffs[b][c].0;

                if gain_a <= gain_b {
                    return false;
                }
            }
        }
        Player::Column => {
            for &r in opponents {
                let gain_a = game.payoffs[r][a].1;
                let gain_b = game.payoffs[r][b].1;

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

/// Strategies removed during one round of iterated elimination, as indices
/// into the original game. Both players are checked against the same table,
/// so everything dominated at the start of the round is removed together.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Round {
    pub rows: Vec<usize>,
    pub cols: Vec<usize>,
}

/// Result of iterated elimination of strictly dominated strategies.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elimination {
    /// One entry per round, in order. Its length is the number of rounds.
    pub rounds: Vec<Round>,
    /// Row strategies left at the end, as indices into the original game.
    pub rows: Vec<usize>,
    /// Column strategies left at the end, as indices into the original game.
    pub cols: Vec<usize>,
}

/// Removes strictly dominated strategies, round after round, until none is
/// left. In each round, every strategy dominated in the current table, for
/// either player, is removed at once.
pub fn iterated_elimination(game: &Game) -> Elimination {
    let mut rows: Vec<usize> = (0..game.row_strategies.len()).collect();
    let mut cols: Vec<usize> = (0..game.col_strategies.len()).collect();
    let mut rounds = Vec::new();

    loop {
        let mut dead_row: Vec<usize> = Vec::new();
        for &x in &rows {
            for &y in &rows {
                if y != x && strictly_dominates_among(game, Player::Row, y, x, &cols) {
                    dead_row.push(x);
                    break;
                }
            }
        }
        let mut dead_cols: Vec<usize> = Vec::new();
        for &x in &cols {
            for &y in &cols {
                if y != x && strictly_dominates_among(game, Player::Column, y, x, &rows) {
                    dead_cols.push(x);
                    break;
                }
            }
        }
        if dead_row.is_empty() && dead_cols.is_empty() {
            break;
        }
        rows.retain(|r| !dead_row.contains(r));
        cols.retain(|c| !dead_cols.contains(c));

        rounds.push(Round {
            rows: dead_row,
            cols: dead_cols,
        });
    }
    Elimination { rounds, rows, cols }
}

/// Pure-strategy Nash equilibria: every cell `(row, col)` where each player's
/// payoff is a best response to the other's strategy, in table order. Empty when
/// the game has no pure equilibrium.
pub fn pure_nash(game: &Game) -> Vec<(usize, usize)> {
    let mut equilibria: Vec<(usize, usize)> = Vec::new();
    let rows: Vec<usize> = (0..game.row_strategies.len()).collect();
    let cols: Vec<usize> = (0..game.col_strategies.len()).collect();

    for &r in &rows {
        for &c in &cols {
            //post-it
            let mut row_ok = true;

            for &r2 in &rows {
                if game.payoffs[r2][c].0 > game.payoffs[r][c].0 {
                    row_ok = false;
                }
            }

            let mut col_ok = true;
            for &c2 in &cols {
                if game.payoffs[r][c2].1 > game.payoffs[r][c].1 {
                    col_ok = false;
                }
            }
            if row_ok && col_ok {
                equilibria.push((r, c));
            }
        }
    }
    equilibria
}
