use crate::*;
use syn::{parse::{Error, ParseBuffer, ParseStream}, braced, bracketed, parenthesized, MacroDelimiter, Result};

#[derive(Clone)]
pub struct Group<T> {
    pub delim: MacroDelimiter,
    pub values: Vec<T>,
}

impl Group<Pattern> {
    /// This is a bit quirky. In a rule, we don't want to capture
    /// anything except fragments and metagroups for efficiency
    /// because we're about to throw this group away. At the same
    /// time, in a metagroup round, we want to keep all the matches in
    /// their original structure while capturing fragments and
    /// metagroups.
    pub fn parse_match(&self, stream: ParseStream, captures: &mut Captures) -> Result<Group<Match>> {
        let buffer;
        match self.delim {
            MacroDelimiter::Paren(_) => {
                let delim = MacroDelimiter::Paren(parenthesized!(buffer in stream));
                self.parse_children(delim, buffer, captures)
            }
            MacroDelimiter::Bracket(_) => {
                let delim = MacroDelimiter::Bracket(bracketed!(buffer in stream));
                self.parse_children(delim, buffer, captures)
            }
            MacroDelimiter::Brace(_) => {
                let delim = MacroDelimiter::Brace(braced!(buffer in stream));
                self.parse_children(delim, buffer, captures)
            }
        }
    }

    fn parse_children(
        &self, delim: MacroDelimiter, buffer: ParseBuffer, captures: &mut Captures
    ) -> Result<Group<Match>> {
        let mut values = vec!();
        if captures.is_rule() {
            for p in self.values.iter() {
                match p.parse_match(&buffer, captures)? {
                    // We are not going to keep the child, so we just have to capture.
                    Match::Fragment(f) =>
                        captures.rule_mut().capture_fragment(f),
                    Match::MetaGroup(g) =>
                        captures.rule_mut().capture_metagroup(g),
                    _ => (),
                }
            }
        } else {
            for p in self.values.iter() {
                let ret = p.parse_match(&buffer, captures)?;
                match &ret {
                    // We have to clone so we can keep the child
                    Match::Fragment(f) =>
                        captures.round_mut().capture_fragment(f.clone()),
                    Match::MetaGroup(g) =>
                        captures.round_mut().capture_metagroup(g.clone()),
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

