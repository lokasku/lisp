# Lisp
Rust implementation of John McCarthy's meta-circular evaluator with its 7 primitives (`quote`, `atom`, `eq`, `car`, `cdr`, `cons` and `cond` + `eval`).
As for how they work, I invite you to read Paul Graham's [The Roots of Lisp](https://languagelog.ldc.upenn.edu/myl/llog/jmc.pdf).

As I'm a nice guy, here's a brief presentation of the implemented primitives:
- `quote` : returns its content, avoiding `eval`uation of the latter.
- `atom` : returns `t` (true) or `()` (false) depending on the atomic membership of its argument.
- `eq` : returns `t` or `()` depending on whether or not its two arguments are equal.
- `car` : takes a list, and returns its head.
- `cdr` : takes a list, and returns its tail.
- `cons` : takes an item (atom or list) and adds it to the beginning of the second argument, which must be a list.
- `cond` : takes an arbitrary (unevaluated) number of lists of the form $(a, b)$ and returns $b$ for the first $a$ not equal to `()`.
# Thanks
Special thanks to [Mesabloo](https://github.com/Mesabloo) and [Gustek](https://github.com/Gystek) for their help throughout the project.
