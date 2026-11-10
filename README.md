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
| [`keynes`](crates/core/src/keynes.rs) | Keynesian beauty contest: pick a number, the closest to ⅔ of the average wins | best response, iterated reasoning (level-k), dominated strategies, Nash equilibrium |

## Layout

```
crates/
└── core/          # crate `equilibre-core`
    ├── src/
    └── tests/
```

## Usage

```sh
cargo test
```

## License

[Apache-2.0](LICENSE)
