Answer: 1223
Difficulty: 1

# Hint

The variable `i` is captured by value in the compiler-generated closure object.

# Explanation

The object passed into `g` is a `FnMut` closure which captures an integer by
value. Effectively it's an unnameable struct containing a single field whose
type is `i32`, with a function call operator that takes `&mut self`:

```rust
#[derive(Copy, Clone)]
pub struct UnnameableClosure {
    i: i32,
}

impl UnnameableClosure {
    pub fn unnameable_call_operator(&mut self) {
        self.i += 1;
        print!("{}", self.i);
    }
}

let mut i = 0i32;
g(UnnameableClosure { i });
```

The behavior of the 4 calls inside `g` is as follows:

- `f()` runs the closure and its by-value captured value of `i` becomes 1.

- `call(f)` makes a **copy** of `f` to become the argument of `call`. The copy
  gets executed and its `i` becomes 2, but the original closure still holds a
  value of 1 for its captured `i`. The copy of the closure gets dropped as it
  goes out of scope at the end of the body of `call`.

- `f()` runs the original closure a second time and its `i` becomes 2.

- `call(f)` copies `f` a second time and executes the copy, its `i` becomes 3.

Since Rust 1.26, closures automatically implement `Clone` if all their captures
implement `Clone`, and `Copy` if all the captures implement `Copy`.

If the `move` keyword were omitted from the quiz code, the compiler-generated
closure would capture `i` by mutable reference instead of by value:

```rust
pub struct UnnameableClosure<'a> {
    i: &'a mut i32,
}
```

and there would no longer be a `Copy` impl, because it's incorrect to duplicate
a mutable reference into multiple copies (aliasing xor mutation; this is the
point of the borrow checker).

One recurring source of confusion for Rust beginners is the relationship between
`move` and non-`move` closures vs `Fn` and `FnMut` and `FnOnce` closures. These
are two nearly-orthogonal things. As illustrated in the `UnnameableClosure`
pseudocode above, `move` vs non-`move` is about whether the *fields* of the
compiler-generated closure struct have the same type as the original captured
variable's type, vs are references to the original captured variable's type
(`i32` vs `&mut i32`, for example). In contrast, `Fn` vs `FnMut` vs `FnOnce` is
about whether the *call method* of the compiler-generated closure struct has a
receiver which is `&self` vs `&mut self` vs `self`.
