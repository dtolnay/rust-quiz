Answer: 11
Difficulty: 3
Warnings: unreachable_patterns

# Hint

`|` has many meanings in Rust.

# Explanation

This question explores the many meanings `|` can have in Rust.

Our first use is not that uncommon, we create a closure that ignores its parameter and returns a value: `|_| Some(1)`. The `|`'s here surround the arguments the closure will receive, much like parenthesis in a normal function definition.

The `let (|x| x)` can be confusing and surprising. Firstly, we're using the pattern-matching abilities that `let` statements provide - you may have seen this being used to destruct tuples, like in `let (x, y) = (1, 2);`, but you can also use anything Rust considers a `Pattern`.  One of those patterns is `Or-patterns`, which groups many patterns and matches when one or more of its subpatterns matches, the following example explores a common use of or-patterns:

```rust
match 1 {
    0 | 2  => todo!(),
    1 | 3 | 4 | 5 => todo!(),
    _ => todo!(),
}
```

The tricky part is that Rust flexibilizes the grammar so that a `Pattern` can have an optional leading `|` - this means the pattern `0 | 2` can also be written as `| 0 | 2`. And, if you ally this with the wildcard pattern (`_`), we have the scary ability to write a pattern that resembles a closure ignoring its argument, `|_| 1`. Summing up, the `let (|x| x)` means that we bind whatever is on the right side to a variable `x`.

Next, we have `x(1 | 2)` that calls the `x` closure, defined at line 2, with the result of the bitwise-or (aka `|`) of 1 and 2.

The match arms use the same matching-the-or-pattern tricky, even tho it seems that we're trying to match against closures, we're not. The value being matched is `Some(1)`, but it's also insignificant, as we're also using the wildcard pattern to match anything - so the match matches the very first arm, printing `1` and returning a closure that also ignores its argument and returns `Some(1)`.

The last part also prints `1` for the same reason we match the first match-arm: we're matching against the irrefutable wildcard pattern (hence the `Some(5)` pattern is completely ignored), and then cast `true` to `u8`,  resulting in `1`. One last confusing, yet irrelevant, part is `x(|_:[();1|2]|2|1)` which invokes the closure returned from the match passing yet another closure that receives an array of `()` with `1 | 2` elements (`[(); 3]`) and returns the value of bitwise-or of 2 and 1.
