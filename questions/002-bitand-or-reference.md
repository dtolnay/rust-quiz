Answer: 123
Difficulty: 2
Warnings: unused_parens

# Hint

One of these four closures is unlike the other three.

# Explanation

The closures `f`, `g`, and `h` are all of type `impl Fn()`. The closure bodies
are parsed as an invocation of the user-defined bitwise-AND operator defined
above by the `BitAnd` trait impl. When the closures are invoked, the bitwise-AND
implementation prints the content of the `S` from the right-hand side and
evaluates to `()`.

The closure `i` is different. Formatting the code with rustfmt makes it clearer
how `i` is parsed.

```rust
let i = || {
    {}
    &S(4)
};
```

The closure body consists of an empty block-statement `{}` followed by a
*reference* to `S(4)`, not a bitwise-AND. The type of `i` is `impl Fn() ->
&'static S`.

The parsing of this case is governed by [this code][classify] in rustc\_ast.

[classify]: https://github.com/rust-lang/rust/blob/1.90.0/compiler/rustc_ast/src/util/classify.rs#L78-L83
