Answer: 121
Difficulty: 1
Warnings: unused_variables

# Hint

There are some programs for which `cargo expand` produces expanded code that
compiles, but behaves differently than the original code with the original macro
hygiene.

# Explanation

There are two reasonable paths to an incorrect answer on this question, based on
your assumptions around how this macro gets expanded:

1. `let a = X(2);`
2. `{ let a = X(2); }`

If the first expansion were right, the macro would introduce a new binding, `a`,
which shadows the `a` already directly assigned in `main`. So the print
statement in `main` would execute first, printing `2`, then the variables would
drop in reverse order of introduction, printing `2` then `1`, with a final
output of `221`.

If the second expansion were right, the macro would introduce `a` in a nested
scope, shadowing the already existing `a` only inside of that scope and not
beyond it. Since the new `a`'s scope ends before the print statement, its `Drop`
impl when going out of scope would be the first print to execute, printing `2`.
Next the print in `main` would print `1` which is the value of the first `a`,
and finally `1` again when that value drops at the end of `main`, with final
output `211`.

If you've read about macro hygiene then you might have guessed it would be
implemented something like this second option. It's important that internals of
a macro don't interfere coincidentally with variables in scope at the call site,
and Rust macros mostly do a good job of preventing unintended name collisions.
However, this is not how hygiene is implemented; introducing artificial scopes
around macro expansions would make them more limited in their usefulness, and
wouldn't solve a lot of other hygiene problems.

You can instead imagine hygiene as a way of assigning a color to each mention of
the name of a local variable, allowing for there to be multiple distinguishable
local variables in scope simultaneously with the same textual name.

<pre><code>fn main() {
    let <b style="background-color:mediumpurple;color:white">a</b> = X(1);
    let <b style="background-color:coral;color:white">a</b> = X(2);
    print!("{}", <b style="background-color:mediumpurple;color:white">a</b>.0);
}</code></pre>

So what's printed is the value of `main`'s identifier
<code><b style="background-color:mediumpurple;color:white">a</b></code>
which is `1`, then the two values are dropped in reverse order of introduction
printing `2` then `1`, and the output of the program is `121`.
