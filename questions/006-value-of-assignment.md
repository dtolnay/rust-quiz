Answer: 0
Difficulty: 1
Warnings: unused_assignments, unused_variables

# Hint

There are two variables named `a`. What is the type of each one?

# Explanation

There are two variables named `a`, one shadowing the other. The program is
equivalent to:

```rust
let a;
let b = a = true;
print!("{}", mem::size_of_val(&b));
```

Further, the value being assigned to `b` is the expression `a = true`.

In Rust, assignment expressions always have the value `()`. Simplified some
more, the quiz code is equivalent to:

```rust
let a = true;
let b = ();
print!("{}", mem::size_of_val(&b));
```

Refer to the documentation of [`size_of_val`] for a specification of its
behavior, but in this case it is being instantiated with `T = ()` and we end up
printing the value of `size_of::<()>()`.

[`size_of_val`]: https://doc.rust-lang.org/std/mem/fn.size_of_val.html

`()` is one example of a [*zero-sized type*][zst] or ZST and is represented by
zero bytes of data at runtime, so the program prints `0`.

[zst]: https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts
