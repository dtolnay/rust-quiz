Answer: 1001
Difficulty: 2

# Hint

`let` is a statement, while assignment is an expression.

# Explanation

When `Drop0` is created, in both cases, it's assigned to a [temporary] memory
location. Temporary memory locations are normally [scoped] to the statement
where they're created, so the value is dropped at the end of the statement
unless it's moved to a new location.

In the [assignment] statement, `_ = &Drop0`, the [underscore
expression][und_expr] `_`, is used to ignore the binding, therefore not moving
the value, and it ends up being dropped at the end of the statement. So, `0` is
printed first then `1`.

In contrast, with `let` statements, [temporary lifetime extension][tle] can take
place and extend the lifetime of the temporary until the end of the block
containing the `let`. For `Drop0`, in the statement `let _ = &Drop0`, this
happens because it's the [operand of a borrow expression][expr_ext]. So even if
the value itself doesn't move, the lifetime of the temporary is extended until
the end of the block and so `1` is printed first then `0`.

[scoped]: https://doc.rust-lang.org/stable/reference/destructors.html#r-destructors.scope.temporary
[temporary]: https://doc.rust-lang.org/stable/reference/expressions.html#temporaries
[tle]: https://doc.rust-lang.org/stable/reference/destructors.html#temporary-lifetime-extension
[assignment]: https://doc.rust-lang.org/stable/reference/expressions/operator-expr.html#r-expr.assign.destructure
[und_expr]: https://doc.rust-lang.org/stable/reference/expressions/underscore-expr.html#_-expressions
[expr_ext]: https://doc.rust-lang.org/stable/reference/destructors.html#r-destructors.scope.lifetime-extension.exprs
