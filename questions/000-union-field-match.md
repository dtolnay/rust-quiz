Answer: 1065353216
Difficulty: 2

# Hint

Unions have no notion of "active" fields. 
`#[repr(C)]` guarantees there is no undefined behaviour in this case. 

# Explanation

Union field access [is analogous][reference] to a transmute to the field's type,
so the match block is roughly equivalent to: 

```rust 
match u {
    U { f1 } if std::mem::transmute<U, i32>(u) == 1 => 
        std::mem::transmute<U, i32>(u), 
    U { f1 } => std::mem::transmute<U, i32>(u), 
    U { f2 } => std::mem::transmute<U, f32>(u) as i32, 
}
```

where the second arm is a wildcard that matches any value after transmuting it 
to the type of `f1`. 

The compiler helps us out here by warning that the last match arm is unreachable: 
```
warning: unreachable pattern
  --> questions/032-union-field-match.rs:12:9
   |
11 |         U { f1: i } => i,
   |         ----------- matches any value
12 |         U { f2: f } => (f + 1.0) as i32,
   |         ^^^^^^^^^^^ unreachable pattern
   |
```
[reference]: https://doc.rust-lang.org/reference/items/unions.html#reading-and-writing-union-fields
