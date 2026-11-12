//! Keynesian beauty contest: every player picks a number, the target is
//! `fraction × average`, and the closest player wins.

/// The rules of the game, chosen by the designer.
#[derive(Debug, Clone)]
pub struct Rules {
    pub fraction: f64,
    pub low: f64,
    pub high: f64,
    /// Highest reasoning level modeled (a modeling assumption).
    pub max_level: u32,
}

impl Default for Rules {
    fn default() -> Self {
        Self {
            fraction: 2.0 / 3.0,
            low: 0.0,
            high: 100.0,
            max_level: 4,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeynesResult {
    pub n_players: usize,
    pub average: f64,
    pub target: f64,
    pub winners: Vec<String>,
}

/// Reasoning level of a choice.
#[derive(Debug, Clone, PartialEq)]
pub enum Level {
    /// Level 0 (= 50), 1, 2… up to `max_level`.
    Reasoning(u32),
    /// Closest to 0, the Nash equilibrium.
    Equilibrium,
    /// Above `fraction × high`: a dominated strategy.
    Dominated,
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeynesError {
    OutOfRange,
    NoPlayers,
}

/// Computes the average, the target and the winners (in input order).
pub fn compute_result(choices: &[(&str, f64)], rules: &Rules) -> Result<KeynesResult, KeynesError> {
    if choices.is_empty() {
        return Err(KeynesError::NoPlayers);
    }
    for (_, choice) in choices {
        if choice < &rules.low || choice > &rules.high {
            return Err(KeynesError::OutOfRange);
        }
    }
    // Average of all choices
    let sum: f64 = choices.iter().map(|(_, choice)| choice).sum();
    let average: f64 = sum / choices.len() as f64;
    let target = rules.fraction * average;

    let mut min_distance = f64::INFINITY;

    for (_, choice) in choices {
        let distance = (choice - target).abs();

        if distance < min_distance {
            min_distance = distance;
        }
    }

    let mut winners = Vec::new();
    for (player, choice) in choices {
        let distance = (choice - target).abs();

        if distance == min_distance {
            winners.push(player.to_string());
        }
    }

    Ok(KeynesResult {
        n_players: choices.len(),
        average,
        target,
        winners,
    })
}

/// Maps a choice to the closest reasoning level.
pub fn reasoning_level(choice: f64, rules: &Rules) -> Result<Level, KeynesError> {
    if choice < rules.low || choice > rules.high {
        return Err(KeynesError::OutOfRange);
    } else if choice > rules.fraction * rules.high {
        return Ok(Level::Dominated);
    }

    let mut targets: Vec<(Level, f64)> = Vec::new();
    let middle = (rules.low + rules.high) / 2.0;
    let mut value = middle;

    for level in 0..=rules.max_level {
        targets.push((Level::Reasoning(level), value));
        value *= rules.fraction;
    }
    targets.push((Level::Equilibrium, 0.0));

    let mut best = &targets[0];
    for target in &targets {
        if (target.1 - choice).abs() < (best.1 - choice).abs() {
            best = target;
        }
    }

    Ok(best.0.clone())
}
