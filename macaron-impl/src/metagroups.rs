use crate::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use smallvec::SmallVec;
use std::{borrow::Cow, collections::HashMap, fmt::{self, Debug}};
use syn::{token, parse::{discouraged::Speculative, ParseStream, Result}};

#[derive(Clone)]
pub struct MetaGroup<T> {
    pub dollar:      token::Dollar,
    pub bracket:     token::Bracket,
    pub name:        Ident,
    pub paren:       token::Paren,
    pub separator:   Option<Separator>,
    pub multiplier:  Multiplier,
    pub values:      Vec<T>,
}

impl<T: ToTokens> ToTokens for MetaGroup<T> {
    fn to_tokens(&self, stream: &mut TokenStream) {
        self.dollar.to_tokens(stream);
        self.bracket.surround(stream, |stream| {
            self.name.to_tokens(stream);
        });
        self.paren.surround(stream, |stream| {
            for v in self.values.iter() {
                v.to_tokens(stream);
            }
        });
        if let Some(s) = &self.separator {
            s.to_tokens(stream);
        }
        self.multiplier.to_tokens(stream)
    }
}

impl<T> Debug for MetaGroup<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        f.write_str("MetaGroup<T>")
    }
}

/// Matching a metagroup does not recurse into it. We must match it
/// potentially many times according to its multiplier.
#[derive(Clone, Debug)]
pub struct MetaGroupMatch {
    pub name: Ident,
    pub multiplier: Multiplier,
    pub rounds: SmallVec<[RoundMatch; 1]>,
}

impl MetaGroup<Pattern> {
    pub fn parse_match(&self, stream: ParseStream) -> Result<Match> {
        let mut rounds: SmallVec<[RoundMatch; 1]> = SmallVec::new();
        loop {
            match self.parse_match_round(stream) {
                Ok(round) => rounds.push(round),
                Err(e) => {
                    if rounds.len() > 1 || self.multiplier.may_be_empty() {
                        return Ok(Match::MetaGroup(MetaGroupMatch {
                            name: self.name.clone(),
                            multiplier: self.multiplier.clone(),
                            rounds
                        }));
                    } else {
                        return Err(e);
                    }
                }
                
            }
        }
    }
    pub fn parse_match_round(&self, stream: ParseStream) -> Result<RoundMatch> {
        let fork = stream.fork();
        let mut scope = Scope::Round(Cow::Owned(RoundMatch::default()));
        for p in self.values.iter() {
            let ret = p.parse_match(&fork, &mut scope)?;
            scope.round_mut().capture_match(ret.clone());
        }
        stream.advance_to(&fork);
        Ok(scope.into_round().unwrap())
    }

    pub fn parse_suffix(input: ParseStream) -> Result<(Option<Separator>, Multiplier)> {
        let l = input.lookahead1();
        if let Ok(multiplier) = input.parse::<Multiplier>() {
            Ok((None,multiplier))
        } else if let Ok(separator) = input.parse::<Separator>() {
            let multiplier = input.parse::<Multiplier>()?;
            Ok((Some(separator),multiplier))
        } else {
            Err(l.error())
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct RoundMatch {
    pub matches:   Vec<Match>,
    pub groups:    HashMap<Ident, MetaGroupMatch>,
    pub fragments: HashMap<Ident, Fragment>,
}

impl RoundMatch {
    pub fn capture_match(&mut self, m: Match) {
        match &m {
            Match::Fragment(f) => self.capture_fragment(f.clone()),
            Match::MetaGroup(g) => self.capture_metagroup(g.clone()),
            _ => (),
        }
        self.matches.push(m);
    }
    pub fn capture_metagroup(&mut self, group: MetaGroupMatch) {
        self.groups.insert(group.name.clone(), group);
    }
    pub fn capture_fragment(&mut self, fragment: FragmentMatch) {
        if let Some(name) = fragment.name {
            self.fragments.insert(name, fragment.fragment);
        }
    }
    pub fn fragment(&self, name: &Ident) -> Option<&Fragment> {
        self.fragments.get(name)
    }
    pub fn group(&self, name: &Ident) -> Option<&MetaGroupMatch> {
        self.groups.get(name)
    }
}

