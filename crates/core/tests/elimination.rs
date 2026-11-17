use equilibre_core::normal_form::{iterated_elimination, Elimination, Game, Round};

fn game(rows: &[&str], cols: &[&str], payoffs: Vec<Vec<(f64, f64)>>) -> Game {
    Game {
        row_strategies: rows.iter().map(|s| s.to_string()).collect(),
        col_strategies: cols.iter().map(|s| s.to_string()).collect(),
        payoffs,
    }
}

fn round(rows: &[usize], cols: &[usize]) -> Round {
    Round {
        rows: rows.to_vec(),
        cols: cols.to_vec(),
    }
}

/// Prisoner's dilemma: silent is dominated for both players from the start,
/// so both go in the same round.
#[test]
fn prisoners_dilemma_takes_one_round() {
    let dilemma = game(
        &["silent", "betray"],
        &["silent", "betray"],
        vec![vec![(3.0, 3.0), (0.0, 5.0)], vec![(5.0, 0.0), (1.0, 1.0)]],
    );
    let expected = Elimination {
        rounds: vec![round(&[0], &[0])],
        rows: vec![1],
        cols: vec![1],
    };
    assert_eq!(iterated_elimination(&dilemma), expected);
}

///            Left     Middle   Right
/// Top       (1, 0)   (1, 2)   (0, 1)
/// Bottom    (0, 3)   (0, 1)   (2, 0)
///
/// Right, then Bottom, then Left. Each removal makes the next one possible.
#[test]
fn two_by_three_takes_three_rounds() {
    let g = game(
        &["top", "bottom"],
        &["left", "middle", "right"],
        vec![
            vec![(1.0, 0.0), (1.0, 2.0), (0.0, 1.0)],
            vec![(0.0, 3.0), (0.0, 1.0), (2.0, 0.0)],
        ],
    );
    let expected = Elimination {
        rounds: vec![round(&[], &[2]), round(&[1], &[]), round(&[], &[0])],
        rows: vec![0],
        cols: vec![1],
    };
    assert_eq!(iterated_elimination(&g), expected);
}

///            Left     Center   Right
/// Top       (2, 1)   (3, 3)   (0, 2)
/// Middle    (1, 2)   (4, 3)   (1, 0)
/// Bottom    (0, 4)   (2, 2)   (5, 1)
///
/// Right, Bottom, Left, Top: four rounds, players take turns.
#[test]
fn three_by_three_takes_four_rounds() {
    let g = game(
        &["top", "middle", "bottom"],
        &["left", "center", "right"],
        vec![
            vec![(2.0, 1.0), (3.0, 3.0), (0.0, 2.0)],
            vec![(1.0, 2.0), (4.0, 3.0), (1.0, 0.0)],
            vec![(0.0, 4.0), (2.0, 2.0), (5.0, 1.0)],
        ],
    );
    let expected = Elimination {
        rounds: vec![
            round(&[], &[2]),
            round(&[2], &[]),
            round(&[], &[0]),
            round(&[0], &[]),
        ],
        rows: vec![1],
        cols: vec![1],
    };
    assert_eq!(iterated_elimination(&g), expected);
}
