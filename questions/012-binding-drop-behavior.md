Answer: 1243
Difficulty: 1

# Hint

The pattern `S { ref x, .. }` borrows a binding `x` from the owner of a value of
type `S`.

# Explanation

This question involves drop-placement. Where does `D` get dropped?

In the first `let`-binding, we destructure a value of type `S` into its field
`x` of type `u8` as well as `..` which represents "the rest of `S`". The part
that is the rest of `S` is dropped immediately at that point because it no
longer has an owner.

In the second `let`-binding, we borrow a field `x` from the owner of a value of
type `S`. The whole value of type `S` remains in scope during the time that its
field `x` is borrowed, and goes out of scope at the close curly brace of `main`.

The output is `1243`.
