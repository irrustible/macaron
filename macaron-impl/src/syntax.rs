use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{parse::{Parse, ParseStream}, Result, Token};

#[derive(Clone)]
pub enum Multiplier {
    One     (Token![$]),
    ZeroOne (Token![?]),
    ZeroMany(Token![*]),
    OneMany (Token![+]),
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

