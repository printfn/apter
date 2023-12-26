# Apter Trees in Rust

Apter Trees are a simple tree representation using two vectors, `node_values`
and `parent_indices`.

This library provides a generic `ApterTree<T>` type and implements a number of
useful functions.

```rust
use apter::ApterTree;

fn main() {
    let mut tree = ApterTree::new();
    tree.insert("root", usize::MAX);
    tree.insert("a", 0);
    tree.insert("b", 0);
    assert_eq!(tree.len(), 3);
}
```
