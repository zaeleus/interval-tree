# interval-tree

A self-balancing [interval tree] library in Rust.

[interval tree]: https://en.wikipedia.org/wiki/Interval_tree

## Install

Add `interval-tree` as a dependency to `Cargo.toml`.

```toml
interval-tree = { git = "https://github.com/zaeleus/interval-tree.git" }
```

## Usage

```rust
extern crate interval_tree;

use interval_tree::IntervalTree;

fn main() {
    let mut tree = IntervalTree::new();

    tree.insert(18..31, "pistachio");
    tree.insert(10..12, "almond");
    tree.insert(17..24, "coconut");

    for entry in tree.find(20..25) {
        println!("{:?} => {}", entry.key, entry.value);
    }

    // 17..24 => coconut
    // 18..31 => pistachio
}
```
