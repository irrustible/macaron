use syn::{token, parse::{discouraged::Speculative, ParseStream, Result}};
use crate::*;
use smallvec::SmallVec;
use std::collections::HashMap;

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

/// Matching a metagroup does not recurse into it. We must match it
/// potentially many times according to its multiplier.
#[derive(Clone)]
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
        let mut captures = Captures::Round(RoundMatch::default());
        for p in self.values.iter() {
            let ret = p.parse_match(&fork, &mut captures)?;
            captures.round_mut().capture_match(ret.clone());
        }
        stream.advance_to(&fork);
        Ok(captures.into_round())
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

#[derive(Clone, Default)]
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
            self.fragments.insert(name.clone(), fragment.fragment);
        }
    }
}

