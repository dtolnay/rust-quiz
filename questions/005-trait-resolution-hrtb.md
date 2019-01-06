Answer: error
Difficulty: 3

# Hint

The answer is different for Rust versions 1.0 through 1.32 vs 1.33+. The answer
accepted as correct here is the one for compilers 1.33+.

# Explanation

This is a rare example of a Rust program that *used to* compile. This code
compiles and runs successfully with every Rust version 1.0 through 1.32,
printing the output `112`. The reasoning on those compilers is as follows.

The first impl applies to function pointers of type `fn(T)` where `T` is any
single concrete type. The second impl applies to function pointers of
[higher-ranked] type `for<'a> fn(&'a T)` for some concrete type `T` that
outlives `'a`.

[higher-ranked]: https://doc.rust-lang.org/nomicon/hrtb.html

Inside of `main`, the compiler is going to use type inference to substitute all
occurrences of `_` in a type by some concrete type.

For the function pointer `a` we infer `_ = u8`, yielding the function pointer
type `fn(u8)` taking an argument of type `u8` and returning `()`.

For `b` we infer `_ = &'x u8` for some concrete lifetime `'x` that will
ultimately feed into the borrow checker. The type of `b` is `fn(&'x u8)`.

And finally for `c` we infer `_ = u8`, yielding the higher-ranked function
pointer type `for<'a> fn(&'a u8)`.

Framed in this way, it follows that the trait method calls at the end of `main`
print `112`.

The compiler's reasoning changed in Rust version 1.33 as part of the ["universe
transition"] and this program no longer compiles. Under the new model the first
impl applies to all three function pointers. If the second impl didn't exist,
the program would compile and print `111`. But with both impls present these are
considered conflicting impls and the program fails to compile.

["universe transition"]: https://github.com/rust-lang/rust/issues/56105
