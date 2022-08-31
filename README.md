# math-test
A simple console application of a math test powered by Rust

## Instalation

You need to have [cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed.

Run:
```
cargo build
```

## Play

Run:
```
cargo run
```

## Gameplay

This test your mind calculation abilities. During each round you need to solve 5 math equations with addition only.
There are 3 level of hardness:
 - Easy
 - Medium
 - Hard

On each step you can run `exit`. If you run `exit` before the round you would successfully exit from the test, If you run `exit` during the round you would finish the round.

After you finished the round you can repeat it or exit completely from the test.
After each round score, average time and you mark would be shown and store the highest score during the whole game lifetime.