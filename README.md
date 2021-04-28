# macaron

A sweeter replacement for `macro_rules`.

## Status: pre-alpha

Doesn't work. Hopefully soon.

## Whaaat?

`macro_rules` has a number of irritating limitations. Here are a few:

* No concept of precedence amongst matches.
* Faux hygiene that's more annoying than helpful.
* Inability to compose names.
* Insufficient flexibility for fragment specifiers.
* Places where you'd like to put a macro but can't.
* Group repetition requires use of the iterated variable.
* etc.

On the other hand, writing proc macros is basically awful, so we'd
like a middle ground - something like `macro_rules` but more powerful!

## Differences from `macro_rules`

Macaron is like a `macro_rules++` - it takes the same basic idea and
makes it more delicious:

* Full order-based match precedence.
* No hygiene at all
* More fragment specifiers.
* Named groups.

Coming soon:

* Flexible transcription-time splicing.
* Builtin functions.
* Syntactical equality.

### Syntax

If you already know `macro_rules`, most of the syntax should be
familiar. We make a few additions to support more flexibility, however.

#### Basic structure

Whereas a `macro_rules` block defines how a single macro should match,
a `macaron` body may contain definitions for many macros/macarons.

```
macro_rules! call {
  ($fun:expr ($($args:expr),*) => { $fun($($args),*) }
}
macaron! {
  pub(super) macro call($fun:expr ($($args:expr),*) { $fun($($args),*) }
}

```

Definition syntax is a hybrid between regular rust format (visibility,
keyword, name etc.) and the existing macro format (a list of patterns
to transcriptions). `fn` becomes `macro`, the arguments become a
pattern list and the body becomes a transcription.

Macros may have multiple matches by simply defining the macro multiple
times. The effective visibility is the visibility of the first
definition.

Macaron compiles down to `macro_rules` invocations, so the visibility
rules are the same as for macros, but we use different syntax:

* `pub(super)` means visible to the module containing the `macaron` block.
* `pub` will prefix `#[macro_export]`to the generated code.
* The default (no) visibility means available to macros in this macaron block.

The other (rust-legal) visibilities are less useful:

* `pub(crate)` is an error - it can't be implemented as it stands.
* `pub(self)` is as useless as usual, equivalent to leaving it off.


### Patterns

The major difference here is the replacement of anonymous metagroups
with named metagroups.

New patterns:

### Transcriptions

As before, anonymous metagroups are replaced with named metagroups.

```
$[name](BODY) - iterate over metagroup `name`, substituting BODY
${EXPR} - splice result of EXPR
```

Splices are analogous to expanding into a macro call, except they are
expanded during transcription and they may call other macros defined
in the block even if they are not public.

Splices are more useful than expanding to a macro definition because
they can be used in more places. The following example is only
possible because of splice:

```rust
macaron! {

    // if `#[pin]` or `#[unpin]` is present in the attributes,
    // strip it out, leaving other attributes intact.
    pub(super) pinless( $[attrs]($a:attr)* $[thing](...) ) { ${ strip_pin!($[attrs]($a)*) } $thing }

    // empty is easy
    macro strip_pin() { }
    // strip out the tokens we don't want
    macro strip_pin( #[pin] $[rest](...) )   { ${ strip_pin!($rest) } }
    macro strip_pin( #[unpin] $[rest](...) ) { ${ strip_pin!($rest) } }
    // anything else should be left and we should check the rest
    macro strip_pin($attr:attr $[rest](...)) { $attr ${ strip_pin!($rest) } }

}
```

### Hygiene and structure checks

lol no. you're on your own.

### Metagroups

### Pattern Matching

Rules are permitted to overlap in `macaron`. Ambiguity is resolved by
declaration order - the first matching rule is the one that will be
transcribed.

We have also made some changes to the patterns. The most major of
which is the replacement of anonymous metagroups with named
metagroups. Additionally, we now allow anonymous matches, which are
covered in more detail when we talk about transcription.

```
$:FRAGSPEC - metafrag - anonymously match FRAGSPEC
$[name](PATTERN) - metagroup named `name` matching PATTERN
```

We have also implemented some new fragment specifiers for metavars

* `attr` - outer (regular) attribute
* `inattr` - inner (bang) attribute
* `name` - an ident that is a valid identifier, not a keyword
* `genarg` - a generic argument
* `genparam` - a generic parameter declaration

### Splices


## Implementation

Macaron's compilation flow is a bit weird because that's how rust
makes us do it. We have a two-stage compilation process:

* Compilation of macarons to macro_rules (`macaron!`)
* Expansion of macarons with resulting macro_rules (`macaron_expand!`)

`macaron`:

1. Parses the program AST.
2. Assembles macarons from rules.
3. Performs error and warning checks.
4. Generates `macro_rules` that invoke `macaron_expand` with:
  * The entire macaron program.
  * The call arguments and name of the macaron.

`macaron_expand`:

1. Parses the program.
2. Locates the called macaron.
3. Matches the call arguments against each rule.
4. Transcribe the matching rule's body.

The matching process in `macaron_expand` is a bit complicated. We need
to use syn, mostly because of metavar fragment specifiers being
arbitrarily complex. But we also have to decide what to match on the
basis of patterns - i.e. consult with some data.

Syn's parsing is based on an efficiently retraversible
`ParseBuffer`. Unfortunately there is no way of creating one that
allows you to use closure or pass extra data through. Converting to
and from `TokenStream` could be quite expensive, so we had to move the
matching process into the parsing stage...

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
