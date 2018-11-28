Answer: 1
Difficulty: 1

# Hint

The call `.f()` resolves to either the field `f` or the inherent method `f`. How
would you write a call to the other one?

# Explanation

A call that looks like `.f()` always resolves to a method, in this case the
inherent method `S::f`. If there were no method `f` in scope, a call like this
would fail to compile even if a field `f` exists and contains a function
pointer.

To call the function pointer stored in field `f`, we would need to write
parentheses around the field access:

```rust
fn main() {
    let print2 = || print!("2");
    (S { f: print2 }.f)();
}
```
