Answer: 32
Difficulty: 1

# Hint

In what ways is a `const` different from a non-mut `static`?

# Explanation

The semantics of `const` is that any mention of the `const` by name in
expression position is substituted with the value of the `const` initializer. In
this quiz code the behavior is equivalent to:

```rust
struct S {
    x: i32,
}

fn main() {
    let v = &mut S { x: 2 };
    v.x += 1;
    S { x: 2 }.x += 1;
    print!("{}{}", v.x, S { x: 2 }.x);
}
```

I have simply substituted every mention of `S` in expresson position with the
value of `const S` which is `S { x: 2 }`.

The first line of `main` is equivalent to:

```rust
let mut _tmp0 = S { x: 2 };
let v = &mut _tmp0;
```

The second line of `main` mutates the value pointed to by `v`. The same value
remains accessible through `v` for the rest of the lifetime of `v`, which is why
the first character printed is `3`.

The third line of `main` mutates a temporary that immediately goes out of scope
at the semicolon. The second character printed is coming from a brand new `S {
x: 2 }`, so `2` is printed.

One additional wrinkle in this code is the concept of namespaces and name
resolution in Rust. Any name that refers to a *type* lives in the *type
namespace*, and any name that refers to a *value* lives in the *value
namespace*. These are two separate sets of names, and the language is structured
such that we can always tell which namespace to look up a name in.

In the context of the quiz code, the name of the struct `S` is part of the type
namespace and the name of the const `S` is part of the value namespace. That is
how we can have seemingly two different things with the same name in scope at
the same time.
