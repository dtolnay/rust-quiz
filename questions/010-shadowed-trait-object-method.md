Answer: 222222
Difficulty: 2
Warnings: dead_code

# Hint

This won't help you answer the question but may help feel better: the quiz
author was also stumped by this one.

# Explanation

This question contains a trait method `Trait::f` as well as an inherent method
`f` on the trait object type `dyn Trait`.

*As far as I know,* given that these names shadow each other, the inherent
method is literally uncallable. There is currently no syntax in Rust for calling
the inherent `f` on `dyn Trait`.

One additional syntax to try would be:

```rust
<dyn Trait>::f(&true);
<dyn Trait>::f(&true as &dyn Trait);
```

If the trait method were named something different, both of these would call the
inherent method. If the inherent method were named something different, both of
these would call the trait method. But if the trait method and the inherent
method are both `f` then the compiler reports an ambiguity.

```
error[E0034]: multiple applicable items in scope
  --> questions/010.rs:18:5
   |
18 |     <dyn Trait>::f(&true);
   |     ^^^^^^^^^^^^^^ multiple `f` found
   |
note: candidate #1 is defined in an impl for the type `dyn Trait`
  --> questions/010.rs:6:5
   |
6  |     fn f(&self) {
   |     ^^^^^^^^^^^
note: candidate #2 is defined in the trait `Trait`
  --> questions/010.rs:2:5
   |
2  |     fn f(&self);
   |     ^^^^^^^^^^^^
   = help: to disambiguate the method call, write `Trait::f(...)` instead
```

Maybe some day it will be possible to disambiguate a call to an inherent method
on a trait object shadowed by a trait method. For now, the quiz code prints
`222222`.
