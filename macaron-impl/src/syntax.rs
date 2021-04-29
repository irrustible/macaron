use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::{Parse, ParseStream}, Result, Token};
use std::fmt::{self, Debug};

#[derive(Clone)]
pub enum Multiplier {
    One     (Token![$]),
    ZeroOne (Token![?]),
    ZeroMany(Token![*]),
    OneMany (Token![+]),
}

impl Debug for Multiplier {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match self {
            Multiplier::One(_) => f.write_str("One"),
            Multiplier::ZeroOne(_) => f.write_str("ZeroOne"),
            Multiplier::ZeroMany(_) => f.write_str("ZeroMany"),
            Multiplier::OneMany(_) => f.write_str("OneMany"),
        }
    }
}

impl ToTokens for Multiplier {
    fn to_tokens(&self, stream: &mut TokenStream) {
        match self {
            Multiplier::One(t) => t.to_tokens(stream),
            Multiplier::ZeroOne(t) => t.to_tokens(stream),
            Multiplier::ZeroMany(t) => t.to_tokens(stream),
            Multiplier::OneMany(t) => t.to_tokens(stream),
        }
    }
}

impl Multiplier {
    pub fn may_be_empty(&self) -> bool {
        use Multiplier::*;
        match self {
            One(_) => false,
            OneMany(_) => false,
            _ => true,
        }
    }
    pub fn unbounded(&self) -> bool {
        use Multiplier::*;
        match self {
            OneMany(_) => true,
            ZeroMany(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone)]
pub struct Attributes(pub Vec<syn::Attribute>);

impl ToTokens for Attributes {
    fn to_tokens(&self, tree: &mut TokenStream) {
        for a in self.0.iter() {
            a.to_tokens(tree);
        }
    }
}

impl Parse for Multiplier {
    fn parse(input: ParseStream) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(Token![$]) {
            input.parse().map(Multiplier::One)
        } else if l.peek(Token![?]) {
            input.parse().map(Multiplier::ZeroOne)
        } else if l.peek(Token![*]) {
            input.parse().map(Multiplier::ZeroMany)
        } else if l.peek(Token![+]) {
            input.parse().map(Multiplier::OneMany)
        } else {
            Err(l.error())
        }
    }
}

// technically, anything except a delimiter or multiplier, but we'll
// start here...
#[derive(Clone)]
pub enum Separator {
    Comma    (Token![,]),
    Semicolon(Token![;]),
}

impl ToTokens for Separator {
    fn to_tokens(&self, stream: &mut TokenStream) {
        match self {
            Separator::Comma(t) => t.to_tokens(stream),
            Separator::Semicolon(t) => t.to_tokens(stream),
        }
    }
}

impl Debug for Separator {
    fn fmt(&self, f: &mut fmt::Formatter) -> std::result::Result<(), fmt::Error> {
        match self {
            Separator::Comma(_) => f.write_str("Comma"),
            Separator::Semicolon(_) => f.write_str("Semicolon"),
        }
    }
}
impl Parse for Separator {
    fn parse(input: ParseStream) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(Token![,]) {
            input.parse().map(Separator::Comma)
        } else if l.peek(Token![;]) {
            input.parse().map(Separator::Semicolon)
        } else {
            Err(l.error())
        }
    }
} 

