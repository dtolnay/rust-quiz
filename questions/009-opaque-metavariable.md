Answer: 21
Difficulty: 2

# Hint

Upon being matched as a `$:expr`, the matched expression becomes a single opaque
token tree.

# Explanation

This question involves the behavior of macro matchers as regards matching macro
metavariables.

Starting from the bottom of the quiz code, the invocation `t!(1)` matches the
first rule of `t!` and expands to `e!(1); m!(1);`.

The invocation `e!(1)` matches the first rule of `e!`. As part of this match,
the expression `1` is packaged into an opaque expression token called `$e`. At
no subsequent point will it be possible for any `macro_rules!` macro to look
inside of `$e`. All that can be known is that `$e` is *some* expression.

In any case, `e!(1)` expands to `m!($e)` where `$e` is an opaque expression
containing `1`. That `m!($e)` *does not* match the first rule of `m!` because
`$e` is opaque. Instead it matches the second rule of `m!` and prints `2`.

After `e!(1)` there is an invocation `m!(1)` coming from the expansion of `t!`.
That one *does* match the first rule of `m!` and prints `1`. The output of this
program is `21`.

Most fragment specifiers have this behavior of becoming opaque token boxes, but
some do not. Specifiers that are opaque once matched:

- `$:block`
- `$:expr`
- `$:item`
- `$:literal`
- `$:meta`
- `$:pat`
- `$:path`
- `$:stmt`
- `$:ty`

The rest of the specifiers do not become opaque and can be inspected by
subsequent rules:

- `$:ident`
- `$:lifetime`
- `$:tt`

For example:

```rust
macro_rules! m {
    ('a) => {};
}

macro_rules! l {
    ($l:lifetime) => {
        // $l is not opaque.
        m!($l);
    }
}

l!('a);
```
