Answer: 121
Difficulty: 2

# Hint

The Rust grammar involving `break` is different from the grammar involving
`return`.

# Explanation

Let's work through the functions one at a time.

- `fn return1`

    The condition of the `if`-statement is parsed as a return-expression that
    returns the value `{ print!("1") }` of type `()`. The value needs to be
    evaluated prior to being returned so this function prints `1`.

- `fn return2`

    This function is parsed the same as `return1`. The `return` keyword eagerly
    consumes a trailing return value, even if the return value begins with a
    curly brace, and even in the condition of an `if`-statement where curly
    braces such as in a struct literal would ordinarly not be accepted. This
    function prints `2`.

- `fn break1`

    The condition of the `if`-statement is a break-with-value expression that
    breaks out of the enclosing loop with the value `{ print!("1") }` of type
    `()`. Similar to `return1`, in order to break with this value the value
    needs to be evaluated and this function prints `1`.

- `fn break2`

    Here we observe a difference between the grammar of `break` and the grammar
    of `return`. Unlike `return`, the `break` keyword in the condition of this
    `if`-statement *does not* eagerly parse a value that begins with a curly
    brace. This code is parsed as:

    ```rust
    loop {
        if break {
            print!("2")
        }
        {}
    }
    ```

    We break out of the loop before executing the print, so this function does
    not print anything.

    I believe the reason for the difference between `return` and `break` is that
    returning a value was obviously supported at Rust 1.0 and well before, but
    break-with-value was introduced fairly late, in [Rust 1.19]. The code in
    `break2` was perfectly legal Rust code prior to Rust 1.19 so we cannot
    change its behavior when implementing the break-with-value language feature.

    It is possible that a future Edition would adjust the two grammars to align
    with each other.

[Rust 1.19]: https://blog.rust-lang.org/2017/07/20/Rust-1.19.html

The output from `main` is `121`.
