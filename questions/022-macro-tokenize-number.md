Answer: 22222
Difficulty: 1

# Hint

The macro is counting how many "tokens" are in its input.

# Explanation

All five invocations of `m!` pass two tokens as input: a minus sign followed by
an integer or floating point literal token.

The floating point literals `1.`, `1.0`, `1.0e1`, `1.0e-1` are each a single
atomic token.

The parser built into the Rust compiler always parses a negative sign as a
separate token from the numeric literal that is being negating. However, it is
possible for a user-defined parser within a [procedural macro] to construct a
negative number as a single token by passing a negative integer or negative
floating point value to one of the constructors of [`proc_macro::Literal`]. If
such a negative literal ends up in the input of a subsequent procedural macro
invocation, it is up to the compiler whether to rewrite into a pair of tokens or
keep them as one.

[procedural macro]: https://github.com/dtolnay/syn
[`proc_macro::Literal`]: https://doc.rust-lang.org/proc_macro/struct.Literal.html

The behavior of the compiler's parser is observable in the surface language as
well, not only in macros. For example the following code prints `-81` because
the expression is parsed as `-(3i32.pow(4))` rather than `(-3i32).pow(4)`.

```rust
fn main() {
    let n = -3i32.pow(4);
    println!("{}", n);
}
```
