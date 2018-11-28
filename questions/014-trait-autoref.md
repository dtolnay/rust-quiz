Answer: 10
Difficulty: 1

# Hint

Trait method auto-ref is covered in [this Stack Overflow answer][SO].

[SO]: https://stackoverflow.com/a/28552082/6086311

# Explanation

Trait impls anywhere in a program are always in scope, so there is no
significance to the `impl Trait for char` being written inside of a block of
code. In particular, that impl is visible throughout the whole program, not just
within the block containing the impl.

This question relates to the behavior of trait method auto-ref which is covered
in [this Stack Overflow answer][SO].

[SO]: https://stackoverflow.com/a/28552082/6086311

The call to `0.is_reference()` observes that there is no implementation of
`Trait` for an integer type that we could call directly. Method resolution
inserts an auto-ref, effectively evaluating `(&0).is_reference()`. This time the
call matches `impl<'a, T> Trait for &'a T` and prints `1`.

The call to `'?'.is_reference()` instead finds `impl Trait for char`, printing
`0`.
