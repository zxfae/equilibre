//! `nash`: solve a two-player game written as a payoff table.
//!
//! Input: a table with `|` between columns. The first line names the column
//! player's strategies; each following line starts with a row strategy, then one
//! cell per column holding `row payoff, column payoff`. Empty lines and lines
//! starting with `#` are ignored. Use `-` to read from standard input.
//!
//! ```text
//!         | Left   | Right
//! Top     | 3, 3   | 0, 5
//! Bottom  | 5, 0   | 1, 1
//! ```

use std::io::Read;
use std::process::ExitCode;

use equilibre_core::normal_form::{iterated_elimination, pure_nash, Game};

const USAGE: &str = "usage: nash <file>   (a payoff table, `-` for stdin)";

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

    match parse(&input) {
        Ok(game) => {
            print!("{}", report(&game));
            ExitCode::SUCCESS
        }
        Err(err) => {
            eprintln!("error: {err}");
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

/// Splits a table line on `|` and trims each cell.
fn cells(line: &str) -> Vec<&str> {
    line.split('|').map(str::trim).collect()
}

/// Parses a payoff table into a game.
fn parse(input: &str) -> Result<Game, String> {
    let mut lines = input
        .lines()
        .enumerate()
        .map(|(i, l)| (i + 1, l.trim()))
        .filter(|(_, l)| !l.is_empty() && !l.starts_with('#'));

    let (header_line, header) = lines.next().ok_or("empty table")?;
    let header = cells(header);
    if header.len() < 2 {
        return Err(format!(
            "line {header_line}: expected column names separated by `|`"
        ));
    }
    let col_strategies: Vec<String> = header[1..].iter().map(|s| s.to_string()).collect();
    if col_strategies.iter().any(|s| s.is_empty()) {
        return Err(format!("line {header_line}: a column name is empty"));
    }

    let mut row_strategies = Vec::new();
    let mut payoffs = Vec::new();
    for (number, line) in lines {
        let parts = cells(line);
        if parts.len() != col_strategies.len() + 1 {
            return Err(format!(
                "line {number}: expected {} cells after the row name, got {}",
                col_strategies.len(),
                parts.len().saturating_sub(1)
            ));
        }
        if parts[0].is_empty() {
            return Err(format!("line {number}: missing row name"));
        }
        let mut row = Vec::new();
        for cell in &parts[1..] {
            row.push(parse_cell(cell).map_err(|e| format!("line {number}: {e}"))?);
        }
        row_strategies.push(parts[0].to_string());
        payoffs.push(row);
    }
    if row_strategies.is_empty() {
        return Err("the table has no rows".to_string());
    }

    Ok(Game {
        row_strategies,
        col_strategies,
        payoffs,
    })
}

/// Parses `a, b` into a pair of payoffs.
fn parse_cell(cell: &str) -> Result<(f64, f64), String> {
    let (a, b) = cell
        .split_once(',')
        .ok_or_else(|| format!("expected `row payoff, column payoff`, got `{cell}`"))?;
    let number = |s: &str| {
        s.trim()
            .parse::<f64>()
            .map_err(|_| format!("`{}` is not a number", s.trim()))
    };
    Ok((number(a)?, number(b)?))
}

fn names(all: &[String], picked: &[usize]) -> String {
    if picked.is_empty() {
        "-".to_string()
    } else {
        picked
            .iter()
            .map(|&i| all[i].as_str())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

fn number(x: f64) -> String {
    if x.fract() == 0.0 {
        format!("{x:.0}")
    } else {
        format!("{x}")
    }
}

/// Builds the text report: elimination round by round, then the equilibria.
fn report(game: &Game) -> String {
    let rows = &game.row_strategies;
    let cols = &game.col_strategies;
    let mut out = String::new();

    out += &format!("Game     {} x {}\n", rows.len(), cols.len());
    out += &format!("Rows     {}\n", rows.join(", "));
    out += &format!("Columns  {}\n\n", cols.join(", "));

    let elimination = iterated_elimination(game);
    out += "Iterated elimination of strictly dominated strategies\n";
    if elimination.rounds.is_empty() {
        out += "  nothing is dominated\n";
    }
    for (i, round) in elimination.rounds.iter().enumerate() {
        out += &format!(
            "  round {}  rows: {}  columns: {}\n",
            i + 1,
            names(rows, &round.rows),
            names(cols, &round.cols)
        );
    }
    out += &format!(
        "  left     rows: {}  columns: {}\n",
        names(rows, &elimination.rows),
        names(cols, &elimination.cols)
    );
    let depth = elimination.rounds.len();
    out += &format!(
        "  depth    {depth} round{}\n\n",
        if depth == 1 { "" } else { "s" }
    );

    let equilibria = pure_nash(game);
    out += "Pure Nash equilibria\n";
    if equilibria.is_empty() {
        out += "  none: every cell leaves someone wanting to switch,\n";
        out += "  so the players must randomize (mixed strategies)\n";
    }
    for (r, c) in equilibria {
        let (a, b) = game.payoffs[r][c];
        out += &format!(
            "  ({}, {})  payoffs {}, {}\n",
            rows[r],
            cols[c],
            number(a),
            number(b)
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    const DILEMMA: &str = "
        # prisoner's dilemma
                | Silent | Betray
        Silent  | 3, 3   | 0, 5
        Betray  | 5, 0   | 1, 1
    ";

    #[test]
    fn parses_names_and_payoffs() {
        let game = parse(DILEMMA).unwrap();
        assert_eq!(game.row_strategies, vec!["Silent", "Betray"]);
        assert_eq!(game.col_strategies, vec!["Silent", "Betray"]);
        assert_eq!(game.payoffs[0][1], (0.0, 5.0));
        assert_eq!(game.payoffs[1][0], (5.0, 0.0));
    }

    #[test]
    fn rejects_wrong_cell_count() {
        let err = parse("| A | B\nX | 1, 1\n").unwrap_err();
        assert!(err.starts_with("line 2"));
    }

    #[test]
    fn rejects_bad_cell() {
        let err = parse("| A\nX | 1 2\n").unwrap_err();
        assert!(err.contains("row payoff, column payoff"));
    }

    #[test]
    fn rejects_table_without_rows() {
        assert!(parse("| A | B\n").is_err());
    }

    #[test]
    fn report_shows_rounds_and_equilibrium() {
        let text = report(&parse(DILEMMA).unwrap());
        assert!(text.contains("round 1  rows: Silent  columns: Silent"));
        assert!(text.contains("depth    1 round\n"));
        assert!(text.contains("(Betray, Betray)  payoffs 1, 1"));
    }

    #[test]
    fn report_explains_missing_equilibrium() {
        let pennies = "| H | T\nH | 1, -1 | -1, 1\nT | -1, 1 | 1, -1\n";
        let text = report(&parse(pennies).unwrap());
        assert!(text.contains("nothing is dominated"));
        assert!(text.contains("mixed strategies"));
    }
}
