Answer: 1
Difficulty: 2

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

If the method `p` had been written as:

```rust
match self {
    Enum::First => print!("1"),
    Enum::Second => print!("2"),
}
```

then this program would print `2`.

However, as written, both arms of the `match` expression are wildcard matches
that successfully match *any* value and bind a *variable* with the name `First`
or `Second`. Match arms are applied in order so the wildcard match in the first
arm is always the one matched.

The compiler helps us out with not one but two relevant warnings. First it
describes exactly how this `match` is parsed and why that is probably silly.

```
warning: unreachable pattern
  --> questions/007.rs:11:13
   |
10 |             First => print!("1"),
   |             ----- matches any value
11 |             Second => print!("2"),
   |             ^^^^^^ unreachable pattern
```

Second, it recognizes what the programmer has done wrong and what they probably
meant to write instead.

```
warning[E0170]: pattern binding `First` is named the same as one of the variants of the type `Enum`
  --> questions/007.rs:10:13
   |
10 |             First => print!("1"),
   |             ^^^^^ help: to match on the variant, qualify the path: `Enum::First`
```

An alternative to writing qualified paths in the pattern is to bring the
variants into scope.

```rust
use Enum::*;

match self {
    First => print!("1"),
    Second => print!("2"),
}
```

Having variants brought into scope by the [standard library prelude][prelude] is
what allows us to write `Ok` and `Some` in match arms, rather than the qualified
paths `Result::Ok` and `Option::Some`.

[prelude]: https://doc.rust-lang.org/std/prelude/index.html
