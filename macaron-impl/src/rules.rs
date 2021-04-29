use crate::*;
use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::{Error, ParseStream}, token, Token};
use std::collections::HashMap;
use std::borrow::Cow;

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
 
impl ToTokens for Rule {
    fn to_tokens(&self, stream: &mut TokenStream) {
        if let Some(p) = self.pub_token {
            p.to_tokens(stream);
        }
        self.macro_token.to_tokens(stream);
        self.name.to_tokens(stream);
        self.paren.surround(stream, |stream| {
            for p in self.patterns.iter() {
                p.to_tokens(stream);
            }
        });
        self.brace.surround(stream, |stream| {
            for p in self.body.iter() {
                p.to_tokens(stream);
            }
        });
    }
}

impl Rule {
    pub fn as_str(&self) -> &str {
        self.name.as_str()
    }
    pub fn parse_match<'a>(&'a self, stream: ParseStream<'a>) -> syn::Result<RuleMatch> {
        let mut scope = Scope::Rule(Cow::Owned(RuleMatch::default()));
        for p in self.patterns.iter() {
            p.parse_match(stream, &mut scope)?;
        }
        if stream.is_empty() {
            Ok(scope.into_rule().unwrap())
        } else {
            Err(Error::new(stream.span(), "Expected end of stream."))
        }
    }
}

#[derive(Clone, Default, Debug)]
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
