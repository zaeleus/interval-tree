# interval-tree

A self-balancing [interval tree] library in Rust.

[interval tree]: https://en.wikipedia.org/wiki/Interval_tree

## Install

Add `interval-tree` as a dependency to `Cargo.toml`.

```toml
interval-tree = { git = "https://github.com/zaeleus/interval-tree.git", tag = "v0.2.0" }
```

## Usage

```rust
use interval_tree::IntervalTree;

fn main() {
    let mut tree = IntervalTree::new();

    tree.insert(18..=31, "apple");
    tree.insert(10..=12, "orange");
    tree.insert(17..=24, "pear");

    for entry in tree.find(20..=25) {
        println!("{:?} => {}", entry.key(), entry.get());
    }

    // 17..=24 => pear
    // 18..=31 => apple
}
```
