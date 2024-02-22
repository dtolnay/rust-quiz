Answer: 84
Difficulty: 3

# Hint

The two calls don't call the same method.

# Explanation

Rust prefers inherent methods over trait methods.

However, in the first call the type of `x` is inference variable `{integer}`,
and on such types Rust only calls trait methods.

After the first call, the type of `x` is constrained to `i64` (since this is the
only `impl`), and then we have a concrete type which we can call inherent methods on.
