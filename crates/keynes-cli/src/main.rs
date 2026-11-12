//! `keynes`: play a Keynesian beauty contest from a file of choices.
//!
//! Input: one player per line, `name,choice`. Empty lines and lines starting
//! with `#` are ignored. Use `-` to read from standard input.

use std::io::Read;
use std::process::ExitCode;

use equilibre_core::keynes::{compute_result, reasoning_level, KeynesError, Level, Rules};

const USAGE: &str = "usage: keynes <file>   (one `name,choice` per line, `-` for stdin)";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let path = match args.as_slice() {
        [path] if path != "-h" && path != "--help" => path,
        _ => {
            eprintln!("{USAGE}");
            return ExitCode::FAILURE;
        }
    };

    let input = match read_input(path) {
        Ok(text) => text,
        Err(err) => {
            eprintln!("error: cannot read {path}: {err}");
            return ExitCode::FAILURE;
        }
    };

    let players = match parse(&input) {
        Ok(players) => players,
        Err(err) => {
            eprintln!("error: {err}");
            return ExitCode::FAILURE;
        }
    };

    let rules = Rules::default();
    let choices: Vec<(&str, f64)> = players.iter().map(|(n, c)| (n.as_str(), *c)).collect();

    match compute_result(&choices, &rules) {
        Ok(result) => {
            print!("{}", report(&choices, &result, &rules));
            ExitCode::SUCCESS
        }
        Err(KeynesError::NoPlayers) => {
            eprintln!("error: no players in {path}");
            ExitCode::FAILURE
        }
        Err(KeynesError::OutOfRange) => {
            let outside: Vec<&str> = choices
                .iter()
                .filter(|(_, c)| *c < rules.low || *c > rules.high)
                .map(|(n, _)| *n)
                .collect();
            eprintln!(
                "error: choices must be between {} and {} (check: {})",
                rules.low,
                rules.high,
                outside.join(", ")
            );
            ExitCode::FAILURE
        }
    }
}

fn read_input(path: &str) -> std::io::Result<String> {
    if path == "-" {
        let mut text = String::new();
        std::io::stdin().read_to_string(&mut text)?;
        Ok(text)
    } else {
        std::fs::read_to_string(path)
    }
}

/// Parses `name,choice` lines into players, in input order.
fn parse(input: &str) -> Result<Vec<(String, f64)>, String> {
    let mut players = Vec::new();
    for (index, raw) in input.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let number = index + 1;
        let (name, choice) = line
            .split_once(',')
            .ok_or_else(|| format!("line {number}: expected `name,choice`, got `{line}`"))?;
        let name = name.trim();
        if name.is_empty() {
            return Err(format!("line {number}: missing name"));
        }
        let choice: f64 = choice
            .trim()
            .parse()
            .map_err(|_| format!("line {number}: `{}` is not a number", choice.trim()))?;
        players.push((name.to_string(), choice));
    }
    Ok(players)
}

fn level_label(level: &Level) -> String {
    match level {
        Level::Reasoning(k) => format!("level {k}"),
        Level::Equilibrium => "equilibrium".to_string(),
        Level::Dominated => "dominated".to_string(),
    }
}

/// Builds the text report: summary, then one row per player.
fn report(
    choices: &[(&str, f64)],
    result: &equilibre_core::keynes::KeynesResult,
    rules: &Rules,
) -> String {
    let label = if result.winners.len() > 1 {
        "Winners"
    } else {
        "Winner"
    };
    let width = choices
        .iter()
        .map(|(n, _)| n.chars().count())
        .max()
        .unwrap_or(0)
        .max("Player".len());

    let mut out = String::new();
    out += &format!("Players  {}\n", result.n_players);
    out += &format!("Average  {:.2}\n", result.average);
    out += &format!(
        "Target   {:.2}   ({:.2} x average)\n",
        result.target, rules.fraction
    );
    out += &format!("{label:<8} {}\n\n", result.winners.join(", "));
    out += &format!(
        "{:<width$}  {:>6}  {:>8}  Level\n",
        "Player", "Choice", "Distance"
    );
    for (name, choice) in choices {
        let level = reasoning_level(*choice, rules)
            .map(|l| level_label(&l))
            .unwrap_or_else(|_| "-".to_string());
        let distance = (choice - result.target).abs();
        let pad = width - name.chars().count();
        out += &format!(
            "{name}{:pad$}  {choice:>6.1}  {distance:>8.1}  {level}\n",
            ""
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_lines_comments_and_spaces() {
        let input = "# header\nTom, 90\n\n  Leo,50.5\n";
        let players = parse(input).unwrap();
        assert_eq!(
            players,
            vec![("Tom".to_string(), 90.0), ("Leo".to_string(), 50.5)]
        );
    }

    #[test]
    fn rejects_missing_comma() {
        let err = parse("Tom 90").unwrap_err();
        assert!(err.starts_with("line 1"));
    }

    #[test]
    fn rejects_bad_number() {
        let err = parse("Tom,90\nLeo,abc").unwrap_err();
        assert!(err.starts_with("line 2"));
    }

    #[test]
    fn rejects_missing_name() {
        assert!(parse(",42").is_err());
    }

    #[test]
    fn report_lists_every_player_with_level() {
        let choices = [("Tom", 90.0), ("Leo", 50.0), ("Zoe", 20.0), ("Moi", 0.0)];
        let rules = Rules::default();
        let result = compute_result(&choices, &rules).unwrap();
        let text = report(&choices, &result, &rules);
        assert!(text.contains("dominated"));
        assert!(text.contains("level 0"));
        assert!(text.contains("equilibrium"));
    }
}
