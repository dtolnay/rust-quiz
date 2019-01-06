Answer: 212
Difficulty: 1

# Hint

Figure out what values are owned by which variables where. A value is dropped
when it no longer has an owner.

# Explanation

This program prints `212`.

No value of type `S` gets dropped within the body of function `f`. The function
`f` conjures an `S` and returns ownership of it to the caller of `f`; the caller
determines when to drop the `S` of which it received ownership.

On the first line of `main`, we call `f()` and perform an infallible match that
binds no new variables. As no variables are declared on this line, there is no
variable that could be the owner of the `S` returned by `f()` so that `S` is
dropped at that point, printing `2`. The `S` in `let S = f()` is a unit struct
pattern (not a variable name) that matches a value of type `S` via
[destructuring] but does not bind the value to any variable.

[destructuring]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-to-break-apart-values

The second line of `main` conjures a new `S`, prints it, and drops it at the
semicolon.
