# nand2tetris

[nand2tetris](https://www.nand2tetris.org/) is a very interesting "learning by doing"
project that enables one to learn how a computer is designed from the ground up, and
also how it can be programmed from the very lowest of levels. It is a fantastic
course and very rewarding to those who finish it.

The projects in this course start from designing various boolean gates using NAND
gates as a building block and slowly build upon each other to finally allow for
running rich and complex full-featured programs such as Tetris.

The project homepage forbids public distribution of the code so as to not rob other
students of the joy of self-learning, but that is really up to the students
themselves; so this repository is public.

## Running the `has` assembler (Project 6)

The `has` assembler from Project 6 is written in Rust and so `cargo run` works
perfectly well. Run it as follows:

```rust
cargo run --release --bin has -- -f projects/6/rect/Rect.asm
```

Read its full help message for more options.

```rust
cargo run --release --bin has -- --help
```
