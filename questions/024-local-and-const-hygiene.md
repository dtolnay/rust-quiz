Answer: 14
Difficulty: 1
Warnings: dead_code, unused_variables

# Hint

Hygiene in `macro_rules!` only applies to local variables.

# Explanation

This program prints `14` because hygiene in `macro_rules!` only applies to local
variables.

You can imagine hygiene as a way of assigning a color to each mention of the
name of a local variable, allowing for there to be multiple distinguishable
local variables in scope simultaneously with the same name.

At the top of `main`, suppose we consider the name of the local variable `x` to
be a purple `x`. The name of the constant `K` is just plain `K`, as constants
are considered items rather than local variables (you can place items outside of
a function body; you cannot place local variables outside of a function body).

<pre><code>let <b style="background-color:mediumpurple;color:white">x</b>: u8 = 1;
const <b>K</b>: u8 = 2;</code></pre>

Continuing down the body of `main`, within the declaration of the macro `m!`
there are identifiers `x` and `K` being used. Since there is a local variable
`x` in scope, the use of the identifier `x` within the macro body picks up the
same color as the local variable `x`. There is no local variable `K` in scope so
the `K` within the declaration of the macro is assigned some new color, say
orange.

<pre><code>macro_rules! m {
    () => {
        print!("{}{}", <b style="background-color:mediumpurple;color:white">x</b>, <b style="background-color:coral;color:white">K</b>);
    };
}</code></pre>

Next we enter a new scope (delimited by curly braces) containing another `x` and
`K`. Every new local variable always introduces a new color so let's call this
`x` blue. The const again is not a local variable so no color is assigned to
`K`.

<pre><code>{
    let <b style="background-color:dodgerblue;color:white">x</b>: u8 = 3;
    const <b>K</b>: u8 = 4;

    m!();
}</code></pre>

When `m!()` expands, the expanded code refers to a purple `x` and an orange `K`.
The purple `x` is distinguishable from the blue `x` -- the value of the purple
`x` is printed which is `1`. As for the `K`, an unhygienic (uncolored) `K` is
allowed to act like any color. The second `K` is shadowing the first one. It
gets picked up when looking for an orange `K` and its value is printed, which is
`4`.

So the output of the quiz code is `14`.
