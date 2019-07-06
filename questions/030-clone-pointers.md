Answer: 111011
Difficulty: 1

# Hint

Immutable pointers `&T` and `Rc<T>` implement `Clone` even if `T` doesn't.

# Explanation

Both of our non-reference types, `()` and `A`, are zero-sized so, for our
purposes, the function `d` will print `0` for dereferenced values and `1` for
pointers, regardless of their size.

`d(a)` prints `1` because `a` is a reference. `A` does not implement `Clone`
but there is a blanket implementation for `&T` which just clones the reference.
`d(a.clone())` will therefore print `1` too.

The type `()` _does_ implement `Clone`, so `b.clone()` returns a `()`, not a
`&()`. `b` is still a reference so `d(b)` prints `1`, but `d(b.clone())` prints
`0` since `()` is zero-sized.

It's considered idiomatic to clone a `Rc` using `Rc::clone(&c)` instead of
`c.clone()`, because it makes it clearer that you cloning the pointer and not
the underlying data. However, these are exactly the same function! In both cases
we clone the `Rc` and `d` prints `1`. To call the `clone` method of a value
inside a `Rc`, you need to dereference it first: `(*c).clone()`.
