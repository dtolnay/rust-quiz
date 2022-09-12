Answer: 121
Difficulty: 1

# Hint

Not every aspect of an expanded macro can be expressed in Rust syntax.

# Explanation

If you got this wrong, there are two "obvious" paths you might have taken, based
on how you expected the macro to be expanded:

1. `let a = X(2);`
2. `{ let a = X(2); }`

If the first expansion was right, the macro would introduce a new binding, `a`,
which shadows the `a` we already declared. So we would first print `2`. Then the
variables would drop in the opposite order to how they were introduced, so we'd
see another `2` then a `1`: `221`.

If the second expansion was right, the macro would introduce `a` in an inner
scope, so that it does not shadow the `a` we already have. Since the new `a`'s
scope ends before the print statement, it is dropped immediately, so the first
thing printed is `2`. Next we print `1` the value of the original variable, and
finally `1` again when it is dropped: `211`.

If you've heard of the phrase "macro hygiene", then you might have guessed that
it would be implemented something like this second case. It's important that
internals of a macro don't interfere accidentally with the code that is using
it, and Rust macros mostly do a good job of preventing unintended naming
conflicts. However, this is not how hygiene is implemented - introducing
artificial scopes around macro expansions would make them more limited in their
usefulness, and wouldn't solve a lot other hygiene problems.

The `cargo expand` command can be used to see how macros are expanded. Its
output for our `main` function is:

```
fn main() {
    let a = X(1);
    let a = X(2);
    // Actually print is also expanded but it's noisy and not useful for us here
    print!("{}", a.0);
}
```

This looks a lot like our guess 1 above! But that is really just a limitation of
`macro expand`: not every macro can be expanded into Rust code that is precisely
representable in Rust syntax. Hidden in the variable names is a kind of
namespace, which tracks where the variable came from. Variables declared inside
macros cannot be addressed from outside, and variables declared outside a macro
cannot be addressed from inside. There is a good explanation of this, using the
idea that variables are "coloured" by their context, in [The Little Book of Rust
Macros](https://danielkeep.github.io/tlborm/book/mbe-min-hygiene.html).

The expansion is really more similar to:

```
fn main() {
    let main_a = X(1);
    let macro_a = X(2);
    print!("{}", main_a.0);
}
```

So we print the value from the first `a`, `1`, then each variable is dropped in
the opposite order to how they were introduced, `2`, then `1`, and the output is
`121`.
