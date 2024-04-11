Answer: 12
Difficulty: 2
Warnings: dead_code

# Hint

I can't help you with this one. This is a pretty arbitrary choice made by the
language. Try all the possibilities!

# Explanation

`S.f()` calls the inherent method `f`. If an inherent method and a trait method
have the same name and receiver type, plain method call syntax will always
prefer the inherent method. The caller would need to write `Trait::f(&S)` or `<S
as Trait>::f(&S)` in order to call the trait method.

It is important for macro authors to be aware of this. Macro-generated code
typically should not use method call syntax to invoke trait methods on types
defined by the user. Those calls could get unintentionally hijacked by inherent
methods having the same name as the trait method.

On the other hand, `S.g()` calls the trait method `g`. Auto-ref during method
resolution always prefers making something into `&` over making it into `&mut`
where either one would work.

See [this Stack Overflow answer][SO] for a more detailed explanation of auto-ref
during method resolution.

[SO]: https://stackoverflow.com/a/28552082/6086311
