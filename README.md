# adventofcode2023
I'm trying to solve Advent of Code 2022 while learning the Rust language.  The goal is not "code golf" or speed.  It is to
learn to use Rust as if I were using it for work or an OSS project: handling errors, using modules and packages, and writing tests.

Each directory `dayN` is a crate with a library and a binary.
The website examples are used as unit test inputs.
Problem inputs are saved as `dayN/input.txt`.
For example:

```
cd day1
cargo test
cargo run prob1 input.txt
cargo run prob2 input.txt
```


