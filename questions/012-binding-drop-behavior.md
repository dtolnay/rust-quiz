Answer: 1243
Difficulty: 1
Warnings: dead_code

# Hint

The pattern `S { ref x, .. }` borrows a binding `x` from the owner of a value of
type `S`.

# Explanation

This question involves drop-placement. Where does `D` get dropped?

In the first `let`-binding, we [destructure] a value of type `S` into its field
`x` of type `u8` as well as `..` which represents "the rest of `S`". The part
that is the rest of `S` is dropped immediately at that point because it no
longer has an owner.

[destructure]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values

In the second `let`-binding, we borrow a field `x` from the owner of a value of
type `S` using a `ref` pattern, causing the entire value scope of type `S` to
be [extended] to the end of `main`.

[extended]: https://doc.rust-lang.org/reference/destructors.html#extending-based-on-patterns

The output is `1243`.
