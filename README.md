# Advent of Code 2024

## Disclaimer

The attached tests are the solution to my inputs.
You will need to use your own inputs.
However, the spirit of the event is to explore the project and find your own solution.
I strongly recommend attempting each problem on your own and using this as inspiration only if you get stuck.
Anyone can copy and paste solutions,
but the true value lies in the journey of solving them.

## How to use the script to pull inputs

### Requirements:
- Python3
  - `python-dotenv` dependency installed

### Environment file

Create a `.env` file with the following content to pull day-specific data:

```env
SESSION=<session token>
```

### How to run

```sh
python3 ./pull_days.py
```

## Requirements

- `cargo`

## How to run the specific days

in the following examples day 1 is used as an example

### Option 1

```sh
cargo run --release -- 1
```

### Option 2

```sh
cargo build --release
./target/release/aoc24 1
```

### Option 3

```sh
cargo install --path .
aoc24 1
```
