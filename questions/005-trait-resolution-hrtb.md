Answer: 112
Difficulty: 2

# Hint

If you are familiar with [higher-rank trait bound][hrtb] syntax, try desugaring
all the types in the impl signatures and types in `main` into their fully
explicit form.

[hrtb]: https://doc.rust-lang.org/nomicon/hrtb.html

# Explanation

The first impl applies to function pointers of type `fn(T)` where `T` is any
single concrete type. The second impl applies to function pointers of
higher-ranked type `for<'a> fn(&'a T)` for some concrete type `T` that outlives
`'a`.

Inside of `main`, the compiler is going to use type inference to substitute all
occurrences of `_` in a type by some concrete type.

For the closure `a` we infer `_ = u8`, yielding the closure type `fn(u8)` taking
an argument of type `u8` and returning `()`.

For `b` we infer `_ = &'x u8` for some concrete lifetime `'x` that will
ultimately feed into the borrow checker. The type of `b` is `fn(&'x u8)`.

And finally for `c` we infer `_ = u8`, yielding the higher-ranked closure type
`for<'a> fn(&'a u8)`.

Framed in this way, it follows that the trait method calls at the end of `main`
print `112`.
