Answer: 24
Difficulty: 3

# Hint

`||` is a closure introducer. `..` is range syntax, normally seen in slicing
operations like `&s[1..4]` or `&s[..s.len() - 1]`.

# Explanation

The two rational possibilities are `1` or `24`, depending on how the precedence
of `|| .. .method()` is disambiguated.

- As `|| ((..).method())`, which is a closure whose body invokes our impl of
  `Trait` on `RangeFull`. In this case `main` would print `1`. It would *not*
  print `13` because the `fn()` returned from `(..).method()` is never invoked
  by `main`.

- As `(|| ..).method()`, which is an invocation of our impl of `Trait` on
  `FnOnce() -> T` where `T` is inferred to be `RangeFull`. In this case `main`
  would print `24`.

The latter of those is the correct answer.

We can achieve the former behavior by explicitly parenthesizing as shown in the
bullet above.

Partially parenthesizing as `|| (.. .method())` is not sufficient. This results
in a parse error.

```
error: expected one of `)` or `,`, found `.`
  --> src/main.rs:22:13
   |
22 |     (|| (.. .method()))();
   |            -^ expected one of `)` or `,`
   |            |
   |            help: missing `,`
```

Correctly handling a quite ambiguous expression like `|| .. .method()` is a
challenge for tooling, as seen by the associated bugs in Rustfmt
([rust-lang/rustfmt#4808]) and Syn ([dtolnay/syn#1019]).

[rust-lang/rustfmt#4808]: https://github.com/rust-lang/rustfmt/issues/4808
[dtolnay/syn#1019]: https://github.com/dtolnay/syn/issues/1019
