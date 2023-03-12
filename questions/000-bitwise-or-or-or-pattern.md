Answer: 10
Difficulty: 2

# Hint

The first argument of `matches!` is an expression, the second one is a pattern.

`true as u8 == 1`, `false as u8 == 0`.

# Explanation

`Some(5|20)` has different syntactic meaning in different contexts.

When parsed as an _expression_, it's simply a
[Bitwise OR](https://doc.rust-lang.org/std/ops/trait.BitOr.html#tymethod.bitor).
So it evals to `Some(21)` and matches.

In _pattern_ context things works a bit differently. Function calls are not
allowed, so it's treated as a (nested)
[Or-pattern](https://doc.rust-lang.org/reference/patterns.html#or-patterns),
which is equivalent to `Some(5) | Some(20)`. We now clearly see it doesn't match.

Addition: if we do want to mean Bitwise OR here we can write
`Some(v) if v == 5|20`. Alternatively
[RFC#2920](https://github.com/rust-lang/rfcs/blob/master/text/2920-inline-const.md)
"Inline const expressions and patterns" will allow `Some(const {5|20})` to be
a valid pattern.
