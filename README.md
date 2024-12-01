# Advent of Code 2024

## How to use the script to pull the inputs

### Requirements:
- Python3
  - python-dotenv dependency installed

### Environment file

It is required to have a .env file with the following inside to pull the day specific data:

```env
SESSION=<session token>
```

### How to run

```sh
python3 ./pull_days.py
```

## Requirements

- cargo

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
