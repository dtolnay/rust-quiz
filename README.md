Rust Quiz
=========

#### What is the output of this Rust program?

<br>

<p align="center">
<b>
<a href="https://dtolnay.github.io/rust-quiz">
<code>
https://dtolnay.github.io/rust-quiz
</code>
</a>
</b>
</p>

<p align="center">
<a href="https://dtolnay.github.io/rust-quiz">
<img src="https://raw.githubusercontent.com/dtolnay/rust-quiz/master/screenshot.png" width="300">
</a>
</p>

<br>

*If you enjoy the Rust Quiz and also know C++, you may like to check out
http://cppquiz.org which inspired this project.*

## Contributing

I welcome suggestions for new quiz questions, either by filing a GitHub issue in
this repository or by sending a pull request containing your question, hint,
answer, and explanation of the answer.

The best questions are drawn from personal experience writing or reading Rust
code and being bewildered by its behavior.

Aim for no more than 25 lines including blank lines, but shorter than that is
better. Questions up to 35 lines may be accepted if there is no possible way to
frame the same idea more concisely. Aim for no wider than 40 columns. The only
exceptions on width will be for obvious boilerplate that is not necessary to
read for solving the question.

The website shows choices for "undefined behavior" and "does not compile", but
please prefer adding questions that do compile and are well-defined.

- Not fun: "does this program compile or does it not compile?"
- Not fun: "is this undefined behavior or is it not?"
- Fun: "does this print obvious possibility A or obvious possibility B?"

To add a question, you need to add one *.rs* file and one *.md* file under the
*questions/* directory. Pick a very brief (2-4 words) description to include in
the file names. Please use 000 as the question number to avoid races between
concurrent pull requests. I will assign a number when merging.

Refer to an existing *.md* file and copy the format. In particular, you will
need to provide a correct answer on the first line, a difficulty rating (1, 2 or
3) on the second line, a **Hint** section, and an **Explanation** section.

The difficulty rating should primarily reflect how obscure is the knowledge
required to confidently solve the question.

When writing a hint, keep it brief. Maximum three lines, maximum two sentences
is ideal.

In the explanation, feel free to be as thorough as possible without dwelling on
concepts that are not relevant to the crux of the quiz question.

To launch the site locally and preview your rendered Markdown, run the
following inside this directory.

```bash
# Package all the questions into a single JavaScript file
# and serve website over http at localhost:8000.
cargo run -- serve
```

Then your question, assuming you numbered it 000, will be accessible at
http\://localhost:8000/rust-quiz/0.

## License

The quiz questions, explanations, website, and all other intellectual property
in this repository are all licensed under the [Creative Commons
Attribution-ShareAlike 4.0 International License](LICENSE-CC-BY-SA).
