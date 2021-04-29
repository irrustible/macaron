use crate::*;
use std::borrow::Cow;

pub enum Scope<'a> {
    Rule(Cow<'a, RuleMatch>),
    Round(Cow<'a, RoundMatch>),
}

impl<'a> Scope<'a> {

    pub fn is_rule(&self) -> bool {
        matches!(self, Scope::Rule(_))
    }

    pub fn is_round(&self) -> bool {
        matches!(self, Scope::Round(_))
    }

    pub fn into_round(self) -> Result<RoundMatch, RuleMatch> {
        match self {
            Scope::Round(r) => Ok(r.into_owned()),
            Scope::Rule(r) => Err(r.into_owned()),
        }
    }

    /// warning: panics if not a rule
    pub fn into_rule(self) -> Result<RuleMatch, RoundMatch> {
        match self {
            Scope::Rule(r) => Ok(r.into_owned()),
            Scope::Round(r) => Err(r.into_owned()),
        }
    }

    /// warning: panics if not a round
    pub fn round_mut(&mut self) -> &mut RoundMatch {
        match self {
            Scope::Round(r) => r.to_mut(),
            _ => panic!("Attempted to take a RoundMatch from a Scope::Rule!"),
        }
    }

    /// warning: panics if not a rule
    pub fn rule_mut(&mut self) -> &mut RuleMatch {
        match self {
            Scope::Rule(r) => r.to_mut(),
            _ => panic!("Attempted to take a RuleMatch from a Scope::Round!"),
        }
    }

    /// Look up a fragment by name
    pub fn fragment(&self, name: &Ident) -> Option<&Fragment> {
        match self {
            Scope::Rule(r) => r.fragment(name),
            Scope::Round(r) => r.fragment(name),
        }
    }

    /// Look up a metagroup by name
    pub fn group(&self, name: &Ident) -> Option<&MetaGroupMatch> {
        match self {
            Scope::Rule(r) => r.group(name),
            Scope::Round(r) => r.group(name),
        }
    }
}
