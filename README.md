# Advent of code 2025

This year I'm practicing writing Rust with Python bindings.

<!--- advent_readme_stars table --->
## 2025 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2025/day/1) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

## Commands

Stub generation:
```sh
cargo run --bin stub_gen
```

Recompile
```sh
uv run maturin develop
```

All solutions:
```sh
uv run aoc-2025
```

All tests:
```sh
uv run aoc-2025 --test
```

Just some specific days:
```sh
uv run aoc-2025 -d 1 -d 5
```

More options:
```sh
uv run aoc-2025 --help
```

Compile all solutions in release mode and run

```sh
uv run maturin develop --release && uv run aoc-2025
```

Do everything - test
```sh
cargo run --bin stub_gen && uv run maturin develop && uv run aoc-2025 --test
```

Do everything - release
```sh
cargo run --bin stub_gen && uv run maturin develop --release && uv run aoc-2025
```
