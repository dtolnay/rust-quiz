Answer: 112031
Difficulty: 1

# Hint

Refer to the documentation of the [`Iterator`] trait.

[`Iterator`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html

# Explanation

As described in the documentation of the [`Iterator::map`] method, the map
operation is performed lazily. The closure provided as an argument to `map` is
only invoked as values are consumed from the resulting iterator. The closure is
not applied eagerly to the entire input stream up front.

[`Iterator::map`]: https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.map

In this code, the `for` loop is what drives the iteration. For each element
consumed from the `parity` iterator, our closure needs to be evaluated one time.
Thus the output will alternate between numbers printed by the closure and
numbers printed by the loop body.
