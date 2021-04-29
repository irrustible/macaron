use crate::*;
use proc_macro2::TokenStream;
use syn::{parse::{Error, ParseBuffer, ParseStream}, braced, bracketed, parenthesized, MacroDelimiter, Result};
use std::fmt::{self, Debug};
use quote::ToTokens;

#[derive(Clone)]
pub struct Group<T> {
    pub delim: MacroDelimiter,
    pub values: Vec<T>,
}

impl<T: ToTokens> ToTokens for Group<T> {
    fn to_tokens(&self, stream: &mut TokenStream) {
        match self.delim {
            MacroDelimiter::Brace(t) => t.surround(stream, |stream| {
                for v in self.values.iter() {
                    v.to_tokens(stream)
                }
            }),
            MacroDelimiter::Bracket(t) => t.surround(stream, |stream| {
                for v in self.values.iter() {
                    v.to_tokens(stream)
                }
            }),
            MacroDelimiter::Paren(t) => t.surround(stream, |stream| {
                for v in self.values.iter() {
                    v.to_tokens(stream)
                }
            }),
        }
    }
}

impl<T> Debug for Group<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match self.delim {
            MacroDelimiter::Paren(_) => f.write_str("()"),
            MacroDelimiter::Bracket(_) => f.write_str("[]"),
            MacroDelimiter::Brace(_) => f.write_str("{}"),
        }
    }
}

impl Group<Pattern> {
    /// This is a bit quirky. In a rule, we don't want to capture
    /// anything except fragments and metagroups for efficiency
    /// because we're about to throw this group away. At the same
    /// time, in a metagroup round, we want to keep all the matches in
    /// their original structure while capturing fragments and
    /// metagroups.
    pub fn parse_match(&self, stream: ParseStream, scope: &mut Scope) -> Result<Group<Match>> {
        let buffer;
        match self.delim {
            MacroDelimiter::Paren(_) => {
                let delim = MacroDelimiter::Paren(parenthesized!(buffer in stream));
                self.parse_children(delim, buffer, scope)
            }
            MacroDelimiter::Bracket(_) => {
                let delim = MacroDelimiter::Bracket(bracketed!(buffer in stream));
                self.parse_children(delim, buffer, scope)
            }
            MacroDelimiter::Brace(_) => {
                let delim = MacroDelimiter::Brace(braced!(buffer in stream));
                self.parse_children(delim, buffer, scope)
            }
        }
    }

    fn parse_children(
        &self, delim: MacroDelimiter, buffer: ParseBuffer, scope: &mut Scope
    ) -> Result<Group<Match>> {
        let mut values = vec!();
        if scope.is_rule() {
            for p in self.values.iter() {
                match p.parse_match(&buffer, scope)? {
                    // We are not going to keep the child, so we just have to capture.
                    Match::Fragment(f) =>
                        scope.rule_mut().capture_fragment(f),
                    Match::MetaGroup(g) =>
                        scope.rule_mut().capture_metagroup(g),
                    _ => (),
                }
            }
        } else {
            for p in self.values.iter() {
                let ret = p.parse_match(&buffer, scope)?;
                match &ret {
                    // We have to clone so we can keep the child
                    Match::Fragment(f) =>
                        scope.round_mut().capture_fragment(f.clone()),
                    Match::MetaGroup(g) =>
                        scope.round_mut().capture_metagroup(g.clone()),
                    _ => (),
                }
                values.push(ret);
            }
        }
        if buffer.is_empty() {
            Ok(Group { delim, values })
        } else {
            Err(Error::new(buffer.span(), "Expected end of group"))
        }
    }
}

pub struct GroupMatch {
    pub delim: MacroDelimiter,
    pub matches: Vec<Match>,
}

