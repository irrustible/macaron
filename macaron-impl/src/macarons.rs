use crate::*;
use std::{borrow::Cow, collections::HashMap};
use syn::parse::{ParseStream, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};

#[derive(Default)]
pub struct Program {
    macarons: HashMap<Ident, Macaron>,
}

impl Program {
    pub fn append_rule(&mut self, rule: Rule) {
        if let Some(mac) = self.macarons.get_mut(&rule.name) {
            mac.append_rule(rule);
        } else {
            self.macarons.insert(rule.name.clone(), Macaron::from(rule));
        }
    }
    pub fn macarons(&self) -> impl Iterator<Item=&Macaron> {
        self.macarons.values()
    }
}    

impl ToTokens for Program {
    fn to_tokens(&self, stream: &mut TokenStream) {
        for m in self.macarons.values() {
            m.to_tokens(stream);
        }
    }
}

pub struct Macaron {
    rules: Vec<Rule>,
}

impl ToTokens for Macaron {
    fn to_tokens(&self, stream: &mut TokenStream) {
        for r in self.rules.iter() {
            r.to_tokens(stream)
        }
    }
}

impl Macaron {
    pub fn is_public(&self) -> bool {
        self.rules.first().map(|r| r.pub_token.is_some()).unwrap_or(false)
    }
    pub fn name(&self) -> &Ident {
        &self.rules[0].name
    }
    pub fn match_trans(&self, stream: ParseStream) -> syn::Result<()> {
        for rule in self.rules.iter() {
            let stream = stream.fork();
            if let Ok(matches)  = rule.parse_match(&stream) {
                let mut scope = Scope::Rule(Cow::Owned(matches));
                let mut out = TokenStream::new();
                for t in rule.body.iter() {
                    t.transcribe(&mut out, &mut scope)?;
                }
                return Ok(());
            }
        }
        Err(Error::new(stream.span(), "no match!"))
    }
    pub fn append_rule(&mut self, rule: Rule) {
        self.rules.push(rule);
    }
    pub fn codegen<P: ToTokens>(&self, stream: &mut TokenStream, program: &P) {
        let mut name = TokenStream::new();
        self.name().ident.to_tokens(&mut name);
        quote!(
            macro_rules! #name {
                ( $( $tt:tt )* ) => {
                    ::macaron_macros::bake(
                        @call [#program] [#name($( $tt )*)]
                    )
                }
            }
        ).to_tokens(stream)
    }
}

impl From<Rule> for Macaron {
    fn from(rule: Rule) -> Self {
        Macaron { rules: vec!(rule) }
    }
}
 
