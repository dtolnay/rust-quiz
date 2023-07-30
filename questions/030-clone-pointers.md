Answer: 111011
Difficulty: 1
Warnings: noop_method_call, unused_variables

# Hint

Immutable pointers `&T` and `Rc<T>` implement `Clone` even if `T` doesn't.

# Explanation

Both of our non-reference types, `()` and `A`, are zero-sized types (ZST). The
function `p<X>` will print `0` if it is passed a value of type `X = ()` or `X =
A`, and it will print `1` if passed a reference `X = &()` or `X = &A` regardless
of exactly how big pointers happen to be.

`p(a)` invokes `p` with `X = &A` because the argument `a` is of type `&A`; this
prints `1`.

On the next line, if `A` implemented `Clone` then `a.clone()` would be a call to
that impl. But since it doesn't, the compiler finds another applicable impl
which is the implementation of `Clone` for references `&T` -- so concretely the
clone call is calling the impl of `Clone` for `&A` which turns a `&&A` into a
`&A` by simply duplicating the reference. We get another call to `p` with `X =
&A` printing `1`. The impl of `Clone` for references is useful in practice when
a struct containing a reference wants to derive `Clone`, but as seen here it can
sometimes kick in unexpectedly.

The type `()` _does_ implement `Clone` so `b.clone()` invokes that impl and
produces `()`. The implementation of `Clone` for `&()` would also be applicable
as happened in the case of `A`, but the compiler prefers calling the trait impl
for `()` which converts `&()` to `()` over the trait impl for `&()` which
converts `&&()` to `&()` because the former is the one that requires fewer
implicit references or dereferences inserted by the trait solver. In the call to
`b.clone()`, `b` is of type `&()` which exactly matches the argument of the impl
`Clone` for `()`, while in order to obtain a `&&()` to pass as argument to the
impl `Clone` for `&()` the trait solver would need to insert an additional layer
of referencing implicitly -- effectively computing `(&b).clone()`.

What we get is `p(b)` calling `p` with `X = &()` and `p(b.clone())` calling `p`
with `X = ()`. Together these print `10`.

Finally in the `Rc` case, both calls to `p` are with `X = Rc<()>` which is
non-zero sized. It is considered idiomatic to clone a `Rc` using `Rc::clone(&c)`
instead of `c.clone()` because it makes it apparent that this is a reference
count bump rather than cloning underlying data, but ultimately both refer to the
same function. To call the `clone` method of a value inside a `Rc`, you would
need to dereference it first: `(*c).clone()`.
