# Puzzle for Rust

[![Travis](https://img.shields.io/travis/george-hopkins/puzzle/master.svg)](https://travis-ci.org/george-hopkins/puzzle)
[![crates.io](https://img.shields.io/crates/v/puzzle.svg)](https://crates.io/crates/puzzle)

The [Puzzle library](https://www.pureftpd.org/project/libpuzzle) lets you quickly find visually similar images, even if they have been resized, recompressed, recolored or slightly modified.


## Getting Started

```rust
extern crate puzzle;

fn main() {
    let mut context = puzzle::Context::new();
    let a = context.cvec_from_file("a.jpg").unwrap();
    let b = context.cvec_from_file("b.jpg").unwrap();
    println!("{}", a.distance(&b));
}
```

## Features

 * `gd`: linking with GD (enabled by default)
 * `jpeg-decoder`: reading JPEGs with [`jpeg-decoder`](https://crates.io/crates/jpeg-decoder) (disabled by default)
