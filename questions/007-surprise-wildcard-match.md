Answer: 2
Difficulty: 2
Warnings: dead_code, non_snake_case, unreachable_patterns

# Hint

The argument of the call to `Enum::p` is guaranteed to be `Enum::Second`.

# Explanation

Filling in the implicit discriminants, the definition of `Enum` is equivalent
to:

```rust
#[repr(u8)]
enum Enum {
    First = 0u8,
    Second = 1u8,
}
```

The unsafe transmute is a red herring. The attribute `#[repr(u8)]` guarantees
that our type has the same representation as `u8`, and the discriminant on
`Enum::Second` guarantees that `Enum::Second` has the same representation as
`1u8`. The transmute is well-defined and evaluates to `Enum::Second`.

