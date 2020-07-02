Answer: 124
Difficulty: 2

# Hint

Either way would be confusing in different situations; there isn't a clear right
behavior that a hint could help identify. Guess both. :/

# Explanation

An `if` guard on a match-arm containing `|` applies to *all* alternatives in the
match-arm not just to the one it is adjacent to.

In the quiz code, does `check(x)` execute at all for `(x, _)` or does it only
cover the `(_, x)` case? We would expect `1` would get printed if and only if
the former is the case. In fact `1` does get printed. A match-arm gets to have
at most one `if`guard and that guard applies to all the `|`-separated
alternatives in the arm.

The `match` expression is equivalent to:

```
match (1, 2) {
    (x, _) if check(x) => print!("3"),
    (_, x) if check(x) => print!("3"),
    _ => print!("4"),
}
```
