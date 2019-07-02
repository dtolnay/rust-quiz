Answer: 1244
Difficulty: 1

# Hint

A value in parentheses does not have the same type as a 1-tuple.

# Explanation

The trailing comma is required in the case of a 1-tuple, `(0,)`, because it
disambiguates it from `(0)` which is identical to `0`. However, for larger
tuples, it is entirely optional: `(i32)` is a distinct type from `(i32,)`, but
`(i32, i32)` and `(i32, i32,)` are the same.

An integral literal `0` can be inferred to be any integer type, but defaults to
`i32` if insufficient type information is available. `(0)` is inferred to be an
`i32` and `(0,)` is inferred to be a `(u32,)` because those are respectively
the only integral and 1-tuple types with an implementation for `Trait`.

Since `(0, 0)` and `(0, 0,)` have the same type, the output of their `p` methods
must be the same, but Rust needs to somehow choose between the two possible
implementatoins of `Trait`, namely `(i32, i32)` and `(u32, u32)`. Since `i32`
is the default integral type, `(i32, i32)` is chosen in both cases.
