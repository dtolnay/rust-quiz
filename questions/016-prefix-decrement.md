Answer: 44
Difficulty: 1

# Hint

The set of operators supported by Rust is documented in [`std::ops`].

[`std::ops`]: https://doc.rust-lang.org/std/ops/index.html

# Explanation

Unlike C or Java, there is no unary increment or decrement operator in Rust.
There is an FAQ entry that touches on the reason: [Why doesn't Rust have
increment and decrement operators?][faq].

[faq]: https://www.rust-lang.org/en-US/faq.html#why-doesnt-rust-have-increment-and-decrement-operators

In the absense of a decrement operator, `--x` is parsed as `-(-x)`. In the case
of `x = 4` this would be `-(-4)` which is `4`. The program is equivalent to:

```rust
fn main() {
    let mut x = 4;
    4;
    print!("{}{}", 4, 4);
}
```
