Answer: 1001
Difficulty: 2

# Hint

`let` is a statement, while assignment is an expression.

# Explanation

In both cases, since we don't assign the `Drop0` instance to a variable, it is a
[temporary].

In `let` statements, [temporary lifetime extension][tle] takes place and extends
the temporary's lifetime until the end of the block, there it is dropped. So `1`
is printed first, and then the `Drop0` is dropped and `0` is printed.

In assignments, however (`_ = ` is a [destructuring assignment][des_assign]
expression), there is no temporary lifetime extension, and temporaries are
dropped at the end of the statement. So, `0` is printed first then `1`.

This behavior also means that if we would try to use the value after the
assignment, the compiler will disallow this with a borrow checker error, as the
value was already dropped.

[temporary]: https://doc.rust-lang.org/stable/reference/expressions.html#temporaries
[tle]: https://doc.rust-lang.org/stable/reference/destructors.html#temporary-lifetime-extension
[des_assign]: https://rust-lang.github.io/rfcs/2909-destructuring-assignment.html
