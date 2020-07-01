Answer: 124
Difficulty: 2

# Hint

Either way would be confusing in different situations; there isn't a clear right
behavior that a hint could help identify. Guess both. :/

# Explanation

This question covers two behaviors of `match` arms and guards.

First, whether an `if` guard on a match-arm containing `|` applies to *all*
alternatives in the match-arm or just to the one it is adjacent to. In the quiz
code, does `check(x)` execute at all for `(x, _)` or does it only cover the `(_,
x)` case? We would expect `1` would get printed if and only if the latter is the
case. In fact `1` does get printed. A match-arm gets to have at most one `if`
guard and that guard applies to all the `|`-separated alternatives in the arm.

But second, this question also covers a kind of "backtracking" behavior of
match-arms. After `check(x)` returns false on `(x, _)`, does the whole match-arm
fail to match at that point or does Rust move on to `(_, x)` and execute the
guard a second time? We would expect `2` to be printed if and only if the latter
is the case. In fact `2` does get printed; the guard is being run multiple
times, once per `|`-separated alternative in the match-arm.
