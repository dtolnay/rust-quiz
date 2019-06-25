Answer: error
Difficulty: 3

# Hint

The way that `f` and `g` are written is not interchangeable.

# Explanation

Function pointer comparison is generally a Bad Idea. It is easily possible to
get nonsensical behavior in optimized builds. For a jaw-dropping example of such
behavior, check out [rust-lang/rust#54685] in which `x == y` is both true and
not true at the same time.

[rust-lang/rust#54685]: https://github.com/rust-lang/rust/issues/54685

That said, the quiz code in this question fails to compile. The reason is
somewhat technical and, as someone who is not a Rust compiler developer, I do
not fully understand it. Hopefully a compiler developer can help me flesh out
this explanation.

Here is the compiler output:

```
error: cannot specify lifetime arguments explicitly if late bound lifetime parameters are present
 --> questions/011.rs:5:18
  |
5 |     let pf = f::<'static> as fn();
  |                  ^^^^^^^
  |
note: the late bound lifetime parameter is introduced here
```

From this I gather that lifetime parameters can be "late bound" or "early
bound". I believe this is compiler-internal terminology that Rust programmers
are not intended to know about or think about in everyday code.

The signature `fn f<'a>()` has a late bound lifetime parameter while the
signature `fn g<'a: 'a>()` has an early bound lifetime parameter.

As far as I know, for most purposes `f` and `g` are interchangeable except that
`f` cannot be given explicit lifetime arguments via turbofish.
