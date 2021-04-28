use crate::*;
use syn::{parse::{Error, ParseStream}, token, Token};
use std::collections::HashMap;

#[derive(Clone)]
pub struct Rule {
    pub pub_token: Option<Token![pub]>,
    pub macro_token: Token![macro],
    pub name: Ident,
    pub paren: token::Paren,
    pub patterns: Vec<Pattern>,
    pub brace: token::Brace,
    pub body: Vec<Transcription>,
}
 
impl Rule {
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
    pub fn parse_match<'a>(&'a self, stream: ParseStream<'a>) -> syn::Result<RuleMatch> {
        let mut scope = Scope::Rule(RuleMatch::default());
        for p in self.patterns.iter() {
            p.parse_match(stream, &mut scope)?;
        }
        if stream.is_empty() {
            Ok(scope.into_rule())
        } else {
            Err(Error::new(stream.span(), "Expected end of stream."))
        }
    }
}

#[derive(Default)]
pub struct RuleMatch {
    pub groups:    HashMap<Ident, MetaGroupMatch>,
    pub fragments: HashMap<Ident, Fragment>,
}
    
impl RuleMatch {
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
