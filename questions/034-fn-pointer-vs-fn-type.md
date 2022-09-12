Answer: 20
Difficulty: 2

# Hint

The answer would be the same with any other integer type in place of `u8`.

# Explanation

The expression `a::<u8>`'s type is a zero-sized type (ZST).

Rust's implementation choices around function types are different from nearly
all other languages, but are an important enabler of many of Rust's
zero-overhead abstractions. In Rust, every function (or every distinct
instantiation of a generic function) has its own unique type. In particular,
even two functions with the same function signature would have different types.

Having a unique type for each function allows the type itself to carry the
information of what function will be called, not needing any runtime state such
as a pointer.

To understand the optimization advantages of this approach, consider
`Iterator::map` and the two calls `iter.map(f)` and `iter.map(g)` where `f` and
`g` are different functions with the same signature. Because `f` and `g` have
distinct types, the two `map` calls would produce two different monomorphic
instantiations of the generic `map` function, one of which statically calls `f`
and the other statically calls `g`, as if you had directly written a
special-purpose map implementation specific to each function without the
abstraction provided by `map`. The generic `map` is thus a zero-overhead
abstraction. Traditionally in other languages such as C++ or Go, in this
situation `f` and `g` would be passed to `map` as a function pointer and there
would be just one instantiation of `map`, containing a dynamic dispatch to
execute the function call, which is usually going to be slower than statically
calling the right function. This performance penalty makes `map` in those
languages not a zero-overhead abstraction.

Currently in Rust there is no syntax to express the type of a specific function,
so they are always passed as a generic type parameter with a `FnOnce`, `Fn` or
`FnMut` bound. In error messages you might see function types appear in the
form `fn(T) -> U {fn_name}`, but you can't use this syntax in code.

On the other hand, a function pointer, `fn(T) -> U`, is pointer-sized at
runtime. Function types can be coerced into function pointers, which can be
useful in case you need to defer the choice of function to call until runtime.

In the quiz code, the first call in `main` coerces `a::<u8>` from a function to
a function pointer (`fn(fn(u8)) {a::<u8>}` to `fn(fn(u8))`) prior to calling
`d`, so its size would be 8 on a system with 64-bit function pointers. The
second call in `main` does not involve function pointers; `d` is directly called
with `T` being the inexpressible type of `a::<u8>`, which is zero-sized.
