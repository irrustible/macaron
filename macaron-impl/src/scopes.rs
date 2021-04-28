use crate::*;

pub enum Scope {
    Rule(RuleMatch),
    Round(RoundMatch),
}

impl Scope {

    pub fn is_rule(&self) -> bool {
        matches!(self, Scope::Rule(_))
    }

    pub fn is_round(&self) -> bool {
        matches!(self, Scope::Round(_))
    }

    /// warning: panics if not a rule
    pub fn into_rule(self) -> RuleMatch {
        match self {
            Scope::Rule(r) => r,
            _ => panic!("Attempted to take a RuleMatch from a Scope::Round!"),
        }
    }

    /// warning: panics if not a round
    pub fn into_round(self) -> RoundMatch {
        match self {
            Scope::Round(r) => r,
            _ => panic!("Attempted to take a RoundMatch from a Scope::Rule!"),
        }
    }

    /// warning: panics if not a rule
    pub fn rule_mut(&mut self) -> &mut RuleMatch {
        match self {
            Scope::Rule(r) => r,
            _ => panic!("Attempted to take a RuleMatch from a Scope::Round!"),
        }
    }

    /// warning: panics if not a round
    pub fn round_mut(&mut self) -> &mut RoundMatch {
        match self {
            Scope::Round(r) => r,
            _ => panic!("Attempted to take a RoundMatch from a Scope::Rule!"),
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
