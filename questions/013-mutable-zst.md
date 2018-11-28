Answer: 1
Difficulty: 1

# Hint

Is it okay for two mutable references to point to the same memory location? What
could go wrong?

# Explanation

In this code, `S` is a [zero sized type][zst] or ZST. Zero sized types are
compile-time concepts that disappear during compilation and have a runtime
representation of zero bytes.

[zst]: https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts

The first line of `main` creates a local value of type `[S; 2]`. Let's refer to
that temporary as `tmp`. The `let`-binding binds two references into `tmp`, `x`
referring to `&mut tmp[0]` and `y` referring to `&mut tmp[1]`.

On the second line of `main` we want to know whether `x` and `y` as pointers
have the same value.

The array type `[S; 2]` is itself a zero sized type. You can confirm this by
printing the value of `std::mem::size_of::<[S; 2]>()`. Indeed the first and
second element of the array have the same memory address.

Ordinarily having multiple mutable references to the same memory location would
not be safe, but in the case of mutable references to zero sized types,
dereferencing is a no-op so there is no way to violate any memory safety
guarantees this way.
