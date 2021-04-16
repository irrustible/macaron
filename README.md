# macaron

A sweeter replacement for `macro_rules`.

## Status: pre-alpha

Doesn't work yet. People were curious and wanted to see it.

## Whaaat?

`macro_rules` has a number of irritating limitations. Here are a few:

* No concept of precedence amongst matches.
* Faux hygiene that's more annoying than helpful.
* Inability to compose names.
* Insufficient flexibility for fragment specifiers.
* Places where you'd like to put a macro but can't.
* Group repetition requires use of the iterated variable.
* etc.

The main reason to use `macro_rules` is to avoid needing a
`proc_macro`. However, one can make an argument that pulling in just
one `proc_macro` isn't so bad (particularly if it's one a lot of
people can make use of). Indeed, I used this argument the other day to
permit myself use of the `paste` crate (a `proc_macro` for name
composition).

While `paste` vastly extends the capabilities of `macro_rules`, I
began to get quite annoyed by the other limitations and realised I
would have to write my own.

## Differences from `macro_rules`

### Syntax

If you already know `macro_rules`, most of the syntax should be
familiar. We make a few additions to support more flexibility, however.

#### Basic structure


```
macro_rules! call {
  ($fun:expr ($($args:expr),*) => { $fun($($args),*) }
}
macaron! {
  pub call($fun:expr ($($args:expr),*) { $fun($($args),*) }
}

```

### Patterns

```
$name:fragspec - metavar definition
$(...) - metagroup
$[name](...) - named metagroup
```

### Transcriptions

```
$name - metavar interpolation
$(...) - metagroup
$[name]() - named metagroup
$[name][] - macaron splice
```

## Copyright and License

Copyright (c) 2021 James Laver, macaron contributors

[Licensed](LICENSE) under Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0),
with LLVM Exceptions (https://spdx.org/licenses/LLVM-exception.html).

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
licensed as above, without any additional terms or conditions.

One file in macaron-impl (diag.rs) is taken from the rust compiler's
`proc_macro` library and is thus is subject to the [Apache2/MIT
combo](https://github.com/rust-lang/rust/blob/master/COPYRIGHT). This
code will hopefully not stay there long.
