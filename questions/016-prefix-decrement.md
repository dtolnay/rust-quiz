Answer: 44
Difficulty: 1
Warnings: unused_must_use, unused_mut

# Hint

The set of operators supported by Rust is documented in [`std::ops`].

[`std::ops`]: https://doc.rust-lang.org/std/ops/index.html

# Explanation

Unlike C or Java, there is no unary increment or decrement operator in Rust. The
Rust language design FAQ (no longer available online) used to touch on the
reason:

> **Why doesn't Rust have increment and decrement operators?**<br>
> Preincrement and postincrement (and the decrement equivalents), while
> convenient, are also fairly complex. They require knowledge of evaluation
> order, and often lead to subtle bugs and undefined behavior in C and C++. `x =
> x + 1` or `x += 1` is only slightly longer, but unambiguous.

In the absence of a decrement operator, `--x` is parsed as `-(-x)`. In the case
of `x = 4` this would be `-(-4)` which is `4`. The program is equivalent to:

```rust
fn main() {
    let mut x = 4;
    4;
    print!("{}{}", 4, 4);
}
```
