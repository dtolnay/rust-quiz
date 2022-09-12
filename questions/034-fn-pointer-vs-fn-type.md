Answer: 20
Difficulty: 2

# Hint

The integer type parameters for `a`  have no affect in this example - they are a
red herring.

# Explanation

There is no difference in size between any of `a::<u8>`, `a::<u16>`, `a::<u32>`
and `a::<u64>`. In fact all `fn` items have zero-sized types (ZST), and each
one's type is unique from the others.

This detail is an important part of how many of Rust's zero-cost abstractions
are implemented. The unique type of each function allows the type itself to
carry the information of what function will be called, so there is no runtime
overhead to passing functions around as values.

There is no syntax to express the type of a specific function, so they are
always passed as a generic type parameter with a `FnOnce`, `Fn` or `FnMut`
bound. In error messages, you might see function types appear in the form
`fn(T) -> U {fn_name}`, but you can't use this syntax in code.

On the other hand, a function pointer, `fn(T) -> U` , is exactly as it sounds -
a pointer, which takes up space. Function types can be coerced into function
pointers, which can be useful in case you need to defer the choice of exactly
_which_ function to call until runtime.

The first two calls to `d` are inside the function `a`, whose argument is
coerced to a `fn(T)`. This is a function pointer and, like other pointers, has a
size of 8 (on a 64 bit system), so the output is `4`.

The second two calls call `d` directly and the `T` parameter is inferred to be
the inexpressible types of the actual functions. The size is therefore `0`, and
that is what is printed.
