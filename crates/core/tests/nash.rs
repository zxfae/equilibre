use equilibre_core::normal_form::{pure_nash, Game};

fn game(rows: &[&str], cols: &[&str], payoffs: Vec<Vec<(f64, f64)>>) -> Game {
    Game {
        row_strategies: rows.iter().map(|s| s.to_string()).collect(),
        col_strategies: cols.iter().map(|s| s.to_string()).collect(),
        payoffs,
    }
}

/// Prisoner's dilemma: both betray.
#[test]
fn prisoners_dilemma_has_one_equilibrium() {
    let g = game(
        &["silent", "betray"],
        &["silent", "betray"],
        vec![vec![(3.0, 3.0), (0.0, 5.0)], vec![(5.0, 0.0), (1.0, 1.0)]],
    );
    assert_eq!(pure_nash(&g), vec![(1, 1)]);
}

/// Freelance quote with strengths (relationship 30 €, portfolio 20 €),
/// payoffs in thousands of euros: both at 515, and I win.
#[test]
fn freelance_with_edge_has_one_equilibrium() {
    let g = game(
        &["me 550", "me 530", "me 515"],
        &["sam 550", "sam 530", "sam 515"],
        vec![
            vec![(11.0, 0.0), (0.0, 10.6), (0.0, 10.3)],
            vec![(10.6, 0.0), (10.6, 0.0), (0.0, 10.3)],
            vec![(10.3, 0.0), (10.3, 0.0), (10.3, 0.0)],
        ],
    );
    assert_eq!(pure_nash(&g), vec![(2, 2)]);
}

/// Meeting up: being together beats being alone, each prefers a different place.
/// Two equilibria: both at the cinema, or both at the restaurant.
#[test]
fn meeting_up_has_two_equilibria() {
    let g = game(
        &["cinema", "restaurant"],
        &["cinema", "restaurant"],
        vec![vec![(2.0, 1.0), (0.0, 0.0)], vec![(0.0, 0.0), (1.0, 2.0)]],
    );
    assert_eq!(pure_nash(&g), vec![(0, 0), (1, 1)]);
}

/// Matching pennies: I want to match, the opponent wants to differ.
/// No pure equilibrium.
#[test]
fn matching_pennies_has_no_pure_equilibrium() {
    let g = game(
        &["heads", "tails"],
        &["heads", "tails"],
        vec![
            vec![(1.0, -1.0), (-1.0, 1.0)],
            vec![(-1.0, 1.0), (1.0, -1.0)],
        ],
    );
    assert_eq!(pure_nash(&g), Vec::<(usize, usize)>::new());
}
