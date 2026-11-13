# Équilibre

Game theory building blocks in Rust: pure, deterministic and fully tested.

Équilibre is a lab where I learn game theory by implementing it, then apply it to real
systems: protocol incentives, blockchain consensus, cryptography, fintech.
Notes on what I learn along the way: [zxfae/TIL](https://github.com/zxfae/TIL).

> [!NOTE]
> Work in progress. The tests are written first, the implementations follow.

## Principles

- **Deterministic core.** Rules, payoffs and equilibria are computed by plain code, never guessed.
- **No dependencies.** `equilibre-core` depends on nothing, so every block can be reused anywhere.
- **Tests first.** Each concept starts with hand-solved examples turned into tests.

## Games

| Module | Game | Concepts |
|---|---|---|
| [`keynes`](crates/core/src/keynes.rs) | Keynesian beauty contest: pick a number, the closest to ⅔ of the average wins | best response, iterated reasoning (level-k), iterated elimination of dominated strategies, Nash equilibrium |

## Tools

### `keynes`

Plays a beauty contest from a file of choices, one `name,choice` per line:

```
# examples/class.csv
Tom,90
Leo,50
Emma,33
Zoe,20
Moi,0
```

```sh
cargo run -p keynes-cli -- examples/class.csv
```

```
Players  5
Average  38.60
Target   25.73   (0.67 x average)
Winner   Zoe

Player  Choice  Distance  Level
Tom       90.0      64.3  dominated
Leo       50.0      24.3  level 0
Emma      33.0       7.3  level 1
Zoe       20.0       5.7  level 2
Moi        0.0      25.7  equilibrium
```

Each player gets a reasoning level: level 0 picks the middle of the range, level k plays
the best response to level k−1, and choices above ⅔ of the maximum can never win
(dominated). The Nash equilibrium is 0, yet the player who plays it rarely wins: being
fully rational is not enough when the others are not.

Install it with `cargo install --path crates/keynes-cli`, then run `keynes <file>`
(use `-` to read from standard input).

## Layout

```
crates/
├── core/          # crate `equilibre-core`: the games, no dependencies
│   ├── src/
│   └── tests/
└── keynes-cli/    # the `keynes` command
examples/          # sample inputs
```

## Usage

```sh
cargo test
```

## License

[Apache-2.0](LICENSE)
