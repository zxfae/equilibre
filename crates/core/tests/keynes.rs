use equilibre_core::keynes::{compute_result, reasoning_level, KeynesError, Level, Rules};

fn approx(a: f64, b: f64) -> bool {
    (a - b).abs() < 1e-9
}

//compute_result

#[test]
fn one_winner_school() {
    let choices = [
        ("Tom", 90.0),
        ("Leo", 50.0),
        ("Emma", 33.0),
        ("Zoe", 20.0),
        ("Moi", 0.0),
    ];
    let result = compute_result(&choices, &Rules::default()).unwrap();

    assert_eq!(result.n_players, 5);
    assert!(approx(result.average, 193.0 / 5.0));
    assert!(approx(result.target, 2.0 / 3.0 * 193.0 / 5.0));
    assert_eq!(result.winners, vec!["Zoe"]);
}

#[test]
fn one_high_winner_school() {
    let choices = [
        ("Tom", 100.0),
        ("Leo", 50.0),
        ("Emma", 33.0),
        ("Zoe", 20.0),
        ("Moi", 0.0),
    ];
    let result = compute_result(&choices, &Rules::default()).unwrap();

    assert_eq!(result.n_players, 5);
    assert!(approx(result.average, 203.0 / 5.0));
    assert!(approx(result.target, 2.0 / 3.0 * 203.0 / 5.0));
    assert_eq!(result.winners, vec!["Emma"]);
}

#[test]
fn two_winner_school() {
    let choices = [
        ("Tom", 90.0),
        ("Leo", 50.0),
        ("Emma", 20.0),
        ("Zoe", 20.0),
        ("Moi", 0.0),
    ];
    let result = compute_result(&choices, &Rules::default()).unwrap();

    assert_eq!(result.n_players, 5);
    assert!(approx(result.average, 180.0 / 5.0));
    assert!(approx(result.target, 2.0 / 3.0 * 180.0 / 5.0));
    assert_eq!(result.winners, vec!["Emma", "Zoe"]);
}

#[test]
fn low_limit() {
    let choices = [
        ("Tom", 90.0),
        ("Leo", 50.0),
        ("Emma", 20.0),
        ("Zoe", 20.0),
        ("Moi", -1.0),
    ];
    assert_eq!(
        compute_result(&choices, &Rules::default()),
        Err(KeynesError::OutOfRange)
    );
}

#[test]
fn high_limit() {
    let choices = [
        ("Tom", 90.0),
        ("Leo", 50.0),
        ("Emma", 20.0),
        ("Zoe", 20.0),
        ("Moi", 110.0),
    ];
    assert_eq!(
        compute_result(&choices, &Rules::default()),
        Err(KeynesError::OutOfRange)
    );
}

#[test]
fn zero_players() {
    assert_eq!(
        compute_result(&[], &Rules::default()),
        Err(KeynesError::NoPlayers)
    );
}

// reasoning_level

#[test]
fn level_zero() {
    assert_eq!(
        reasoning_level(50.0, &Rules::default()),
        Ok(Level::Reasoning(0))
    );
}

#[test]
fn level_two() {
    assert_eq!(
        reasoning_level(20.0, &Rules::default()),
        Ok(Level::Reasoning(2))
    );
}

#[test]
fn between_two_levels() {
    assert_eq!(
        reasoning_level(27.0, &Rules::default()),
        Ok(Level::Reasoning(2))
    );
}

#[test]
fn dominated() {
    assert_eq!(
        reasoning_level(90.0, &Rules::default()),
        Ok(Level::Dominated)
    );
}

#[test]
fn equilibrium() {
    assert_eq!(
        reasoning_level(0.0, &Rules::default()),
        Ok(Level::Equilibrium)
    );
}

#[test]
fn level_low_limit() {
    assert_eq!(
        reasoning_level(-5.0, &Rules::default()),
        Err(KeynesError::OutOfRange)
    );
}

#[test]
fn level_high_limit() {
    assert_eq!(
        reasoning_level(150.0, &Rules::default()),
        Err(KeynesError::OutOfRange)
    );
}

#[test]
fn levels_scale_with_range() {
    // range 0..1000: level 0 = 500, level 1 = 333.3
    let rules = Rules { low: 0.0, high: 1000.0, ..Rules::default() };
    assert_eq!(reasoning_level(333.0, &rules), Ok(Level::Reasoning(1)));
}

#[test]
fn dominated_scales_with_range() {
    let rules = Rules { low: 0.0, high: 1000.0, ..Rules::default() };
    assert_eq!(reasoning_level(900.0, &rules), Ok(Level::Dominated));
}
