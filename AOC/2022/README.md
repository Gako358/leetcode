# Advent of code 

> **Disclaimer:** _This template is based on_ [fspoettel's](https://github.com/fspoettel/advent-of-code-rust) with modifications.

## Usage

### Scaffold a day
```sh
# example: `cargo scaffold 1`
cargo scaffold <day>

# output:
# Created module "src/bin/01.rs"
# Created empty input file "src/inputs/01.txt"
# Created empty example file "src/examples/01.txt"
# ---
# 🎄 Type `cargo solve 01` to run your solution.
```

### Download input for a day

> **Note**  
> This command requires [installing the aoc-cli crate](#download-puzzle-inputs-via-aoc-cli).

```sh
# example: `cargo download 1`
cargo download <day>

# output:
# Downloading input with aoc-cli...
# Loaded session cookie from "/home/felix/.adventofcode.session".
# Downloading input for day 1, 2021...
# Saving puzzle input to "/tmp/tmp.MBdcAdL9Iw/input"...
# Done!
# ---
# 🎄 Successfully wrote input to "src/inputs/01.txt"!
```

To download inputs for previous years, append the `--year/-y` flag. _(example: `cargo download 1 --year 2020`)_

Puzzle inputs are not checked into git. [Reasoning](https://old.reddit.com/r/adventofcode/comments/k99rod/sharing_input_data_were_we_requested_not_to/gf2ukkf/?context=3).

### Run solutions for a day

```sh
# example: `cargo solve 01`
cargo solve <day>

# output:
#     Running `target/debug/01`
# 🎄 Part 1 🎄
#
# 6 (elapsed: 37.03µs)
#
# 🎄 Part 2 🎄
#
# 9 (elapsed: 33.18µs)
```

`solve` is an alias for `cargo run --bin`. To run an optimized version for benchmarking, append the `--release` flag.

Displayed _timings_ show the raw execution time of your solution without overhead (e.g. file reads).

### Run all solutions

```sh
cargo all

# output:
#     Running `target/release/advent_of_code`
# ----------
# | Day 01 |
# ----------
# 🎄 Part 1 🎄
#
# 0 (elapsed: 170.00µs)
#
# 🎄 Part 2 🎄
#
# 0 (elapsed: 30.00µs)
# <...other days...>
# Total: 0.20ms
```

`all` is an alias for `cargo run`. To run an optimized version for benchmarking, use the `--release` flag.

_Total timing_ is computed from individual solution _timings_ and excludes as much overhead as possible.

