Answer: 2
Difficulty: 1

# Hint

The set of operators supported by Rust is documented in [`std::ops`].

[`std::ops`]: https://doc.rust-lang.org/std/ops/index.html

# Explanation

Unlike C or Java, there is no unary increment or decrement operator in Rust.
There is an FAQ entry that touches on the reason: [Why doesn't Rust have
increment and decrement operators?][faq].

[faq]: https://www.rust-lang.org/en-US/faq.html#why-doesnt-rust-have-increment-and-decrement-operators

In the absense of postfix and prefix decrement operators, `a-- - --b` is parsed
as `a - (-(-(-(-b))))`. In the case of `a = 5` and `b = 3` the value of this
expression is `5 - 3` which is `2`.
