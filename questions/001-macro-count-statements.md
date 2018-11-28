Answer: 112
Difficulty: 3

# Hint

The expression in the output of the macro evaluates to the same value as `1 <<
(n - 1)` where `n` is the number of statements contained in the macro input.

# Explanation

This question revolves around where the Rust grammar places statement
boundaries.

The input rule of the macro `m!` is `$($s:stmt)*` which matches zero or more
Rust statements. The `$(`...`)*` part of the rule is a *repetition* which
matches the contents of the repetition zero or more times, and the `$s:stmt` is
a fragment specifier that matches a Rust statement (`stmt`) conforming to the
rules of the Rust grammar. The matched statements are available within the
expanded code as the fragment variable `$s`.

A *statement* is the top-level unit of syntax permitted within a function body.
All of the following are examples of statements. The grammar of function bodies
requires that some types of statements are followed by a semicolon, but the
semicolon is not part of the statement for the purpose of macro syntax.

```rust
// Items are statements.
struct S { x: u64 }

// Let-bindings are statements.
let mut s = S { x: 1 }

// Expressions are statements.
s.x + 1
```

The macro `m!` expands to zero or more copies of `{ stringify!($s); 1 }`
separated by the `<<` token. The `$(`...`)<<*` part of the rule is a repetition
using `<<` as the separator.

Using `<<` as a separator in a repetition in a macro is highly unusual. The most
commmonly used separator is the comma, written as `$(`...`),*`, but any other
single token is allowed here. Crucially, `macro_rules!` treats all built-in Rust
operators as single tokens, even those that consist of multiple characters like
`<<`.

The `{ stringify!($s); 1 }` is an expression whose value is always 1. The value
of `stringify!($s)` is discarded, so this is equivalent to the expression `{ 1
}`. The reason for having `stringify!($s)` in there is to control the number of
times the repetition is repeated, which is determined by which fragment
variables are used within the repetition. Writing a repetition without using any
fragment variables inside of it would not be legal.

Suppose we call this macro with three of the statements shown above as input.

```rust
m! {
    struct S { x: u64 }
    let mut s = S { x: 1 }
    s.x + 1
}
```

The macro expands to:

```rust
{ stringify!(struct S { x: u64 }); 1 }
    << { stringify!(let mut s = S { x: 1 }); 1 }
    << { stringify!(s.x + 1); 1 }
```

Each of the `stringify`s expands to a string literal:

```rust
{ "struct S { x: u64 }"; 1 }
    << { "let mut s = S { x: 1 }"; 1 }
    << { "s.x + 1"; 1 }
```

The values of the string literals are not used. In this case the expression is
equivalent to `{ 1 } << { 1 } << { 1 }`, which is equivalent to `1 << 1 << 1`.
The `<<` operator is left-associative; the numeric value of this expression is
4\.

Altogether, the relevant behavior of this macro is that it evaluates to `1 << 1
<< 1 << ...` where the number of ones is equal to the number of Rust statements
in the input of the macro. In closed form, the numeric value is `1 << (n - 1)`
where `n` is the number of statements, except in the case that `n` is zero where
the macro expands to nothing and we get a syntax error at the call site.

It remains to determine how many statements are in the three invocations of
`m!` in the quiz code.

1. `return || true`

    This is a return-expression that would return the closure `|| true`. It is
    equivalent to `return (|| true)`. It is parsed as a single statement so the
    `m!` invocation evaluates to `1`.

2. `(return) || true`

    This is a logical-OR expression. The `||` is a binary operator, where the
    left-hand side is the expression `(return)` (of diverging type `!`) and the
    right-hand side is the expression `true`. This expression is a single
    statement so `m!` again evaluates to `1`.

3. `{return} || true`

    This one is two statements! A block-statement `{return}` followed by a
    closure expression `|| true`.

    The Rust grammar distinguishes between expressions that require a semicolon
    in order to stand alone as a statement, and expressions that can be
    statements even without a semicolon. Consider two examples:

    ```rust
    // No trailing semicolon required.
    for t in vec {
        /* ... */
    }

    // Trailing semicolon required.
    self.skip_whitespace()?;
    ```

    The list of expression types that stand alone without a semicolon is defined
    [here][classify] in libsyntax. The distinction informs a few different early
    bail-out cases where the parser decides to finish parsing the current
    expression.

    Relevant to our case is that block expressions `{ /* ... */ }` terminate an
    expression if doing so would be syntactically sensible. The parser does not
    eagerly consume binary operators after a block expression. Thus one might
    write:

    ```rust
    fn f() -> &'static &'static bool {
        // Block expression.
        {
            println!("What a silly function.");
        }

        // Reference to reference to true.
        &&true
    }
    ```

    In order to parse a block followed by a binary operator, we would need to
    make it syntactically insensible for the parser to terminate an expression
    at the close curly brace. This would usually be done by wrapping in
    parentheses.

    ```rust
    fn f() -> bool {
        ({ true } && true)
    }
    ```

[classify]: https://github.com/rust-lang/rust/blob/1.30.1/src/libsyntax/parse/classify.rs#L17-L37

Anyhow, the output of the program is `112`.
