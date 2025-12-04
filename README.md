# Advent of code 2025

This year I'm practicing writing Rust libraries with Python bindings. The tools for each solution are written in a Rust package, which is compiled into a library with Python bindings and used by a small Python application to put the solution together and report the result along with timings.

<!--- advent_readme_stars table --->
## 2025 Results

| Day | Part 1 | Part 2 |
| :---: | :---: | :---: |
| [Day 1](https://adventofcode.com/2025/day/1) | ⭐ | ⭐ |
| [Day 2](https://adventofcode.com/2025/day/2) | ⭐ | ⭐ |
| [Day 3](https://adventofcode.com/2025/day/3) | ⭐ | ⭐ |
| [Day 4](https://adventofcode.com/2025/day/4) | ⭐ | ⭐ |
<!--- advent_readme_stars table --->

## Setup

To run this against your AoC inputs, add your puzzle data to `data/eval/` with a structure matching `data/test/`.

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

All test inputs:
```sh
uv run aoc-2025 --test
```

Just some specific days:
```sh
uv run aoc-2025 -d 1 -d 5
```

Repeat 30 times and average the timing results:
```sh
uv run aoc-2025 -r 30
```

Help:
```sh
uv run aoc-2025 --help
```

### Useful combos

Compile in release mode and run with timings averaged over 30 runs
```sh
uv run maturin develop --release && uv run aoc-2025 -r 30
```

Do everything - dev build and test inputs (data/test)
```sh
cargo run --bin stub_gen && uv run maturin develop && uv run aoc-2025 --test
```

Do everything - release build and real inputs (data/eval)
```sh
cargo run --bin stub_gen && uv run maturin develop --release && uv run aoc-2025
```
