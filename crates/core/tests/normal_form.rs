use equilibre_core::normal_form::{strictly_dominates, weakly_dominates, Game, Player};

const SILENT: usize = 0;
const BETRAY: usize = 1;

/// Builds a prisoner's dilemma from the four row payoffs; the game is symmetric.
fn dilemma(both_silent: f64, sucker: f64, temptation: f64, both_betray: f64) -> Game {
    let names = vec!["silent".to_string(), "betray".to_string()];
    Game {
        row_strategies: names.clone(),
        col_strategies: names,
        payoffs: vec![
            vec![(both_silent, both_silent), (sucker, temptation)],
            vec![(temptation, sucker), (both_betray, both_betray)],
        ],
    }
}

/// Original game: silent (3,3) (0,5) / betray (5,0) (1,1).
fn original() -> Game {
    dilemma(3.0, 0.0, 5.0, 1.0)
}

/// Betraying is punished by 2: temptation 5 - 2 = 3, both betray 1 - 2 = -1.
fn punished_by_2() -> Game {
    dilemma(3.0, 0.0, 3.0, -1.0)
}

/// Betraying is punished by 3: temptation 5 - 3 = 2, both betray 1 - 3 = -2.
fn punished_by_3() -> Game {
    dilemma(3.0, 0.0, 2.0, -2.0)
}

// Question 1
#[test]
fn betray_strictly_dominates_silent_for_row() {
    assert!(strictly_dominates(&original(), Player::Row, BETRAY, SILENT));
}

// Question 2
#[test]
fn betray_strictly_dominates_silent_for_column() {
    assert!(strictly_dominates(
        &original(),
        Player::Column,
        BETRAY,
        SILENT
    ));
}

// Question 3
#[test]
fn silent_does_not_dominate_betray_in_original() {
    assert!(!strictly_dominates(
        &original(),
        Player::Row,
        SILENT,
        BETRAY
    ));
    assert!(!weakly_dominates(&original(), Player::Row, SILENT, BETRAY));
}

// Question 4
#[test]
fn punishment_of_2_is_not_enough_for_strict() {
    assert!(!strictly_dominates(
        &punished_by_2(),
        Player::Row,
        SILENT,
        BETRAY
    ));
}

// Question 5
#[test]
fn punishment_of_2_gives_weak_domination() {
    assert!(weakly_dominates(
        &punished_by_2(),
        Player::Row,
        SILENT,
        BETRAY
    ));
}

// Question 6
#[test]
fn punishment_of_3_gives_strict_domination() {
    assert!(strictly_dominates(
        &punished_by_3(),
        Player::Row,
        SILENT,
        BETRAY
    ));
}

// Question 7
#[test]
fn a_strategy_never_dominates_itself() {
    assert!(!strictly_dominates(
        &original(),
        Player::Row,
        BETRAY,
        BETRAY
    ));
    assert!(!weakly_dominates(&original(), Player::Row, BETRAY, BETRAY));
}

const TOP: usize = 0;
const BOTTOM: usize = 1;
const LEFT: usize = 0;
const RIGHT: usize = 1;

/// Asymmetric game, so that reading the wrong player's payoffs shows up.
///
///            Left     Right
/// Top       (1, 4)   (2, 1)
/// Bottom    (3, 5)   (0, 2)
fn asymmetric() -> Game {
    Game {
        row_strategies: vec!["top".to_string(), "bottom".to_string()],
        col_strategies: vec!["left".to_string(), "right".to_string()],
        payoffs: vec![vec![(1.0, 4.0), (2.0, 1.0)], vec![(3.0, 5.0), (0.0, 2.0)]],
    }
}

#[test]
fn left_strictly_dominates_right_for_column() {
    assert!(strictly_dominates(
        &asymmetric(),
        Player::Column,
        LEFT,
        RIGHT
    ));
}

#[test]
fn right_does_not_dominate_left_for_column() {
    assert!(!strictly_dominates(
        &asymmetric(),
        Player::Column,
        RIGHT,
        LEFT
    ));
    assert!(!weakly_dominates(
        &asymmetric(),
        Player::Column,
        RIGHT,
        LEFT
    ));
}

#[test]
fn bottom_does_not_dominate_top_for_row() {
    assert!(!strictly_dominates(&asymmetric(), Player::Row, BOTTOM, TOP));
    assert!(!weakly_dominates(&asymmetric(), Player::Row, BOTTOM, TOP));
}

#[test]
fn strict_domination_is_also_weak() {
    assert!(weakly_dominates(&original(), Player::Row, BETRAY, SILENT));
    assert!(weakly_dominates(&asymmetric(), Player::Column, LEFT, RIGHT));
}
