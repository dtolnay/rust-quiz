Answer: 1
Difficulty: 1

# Hint

What type would type inference infer for `x`?

# Explanation

During type inference the variable `x` has type `&{integer}`, a reference to
some as yet undetermined integer type.

If we want to resolve the trait method call `Trait::f(x)`, we find that its
argument `x` must be of type `&Self` for some type `Self` that implements
`Trait`. We find that inferring `0: u32` satisfies both the constraint that
`u32` is an integer as well as `u32` implements `Trait`, so the method call ends
up calling `<u32 as Trait>::f(x)` and prints `1`.

Trait method resolution is covered in more detail in [this Stack Overflow
answer][SO].

[SO]: https://stackoverflow.com/a/28552082/6086311
