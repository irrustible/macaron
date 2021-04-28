use proc_macro2::TokenStream;
use syn::{ext::IdentExt, Result};
use syn::parse::{Error, Lookahead1, Parse, ParseStream};
use quote::ToTokens;
use std::hash::{Hash, Hasher};

#[derive(Clone)]
pub struct Ident {
    pub ident: syn::Ident,
    pub string: String,
}

impl PartialEq for Ident {
    fn eq(&self, other: &Self) -> bool {
        self.string == other.string
    }
}

impl Eq for Ident {}

impl Hash for Ident {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.string.hash(state);
    }
}

impl Ident {
    pub fn peekahead(input: &Lookahead1) -> bool {
        input.peek(syn::Ident)
    }
    pub fn peek(input: ParseStream) -> bool {
        input.peek(syn::Ident)
    }
    pub fn peek2(input: ParseStream) -> bool {
        input.peek2(syn::Ident)
    }
    pub fn peek3(input: ParseStream) -> bool {
        input.peek3(syn::Ident)
    }
    pub fn as_str(&self) -> &str {
        self.string.as_str()
    }
    pub fn parse_any(input: ParseStream) -> syn::Result<Ident> {
        let ident = syn::Ident::parse_any(input)?;
        let string = ident.to_string();
        Ok(Ident { ident, string })
    }
    pub fn parse_match(&self, stream: ParseStream) -> Result<Ident> {
        stream.step(|cursor| {
            if let Some((i,j)) = cursor.ident() {
                let i = Ident::from(i);
                if i.as_str() == self.as_str() {
                    Ok((i, j))
                } else {
                    Err(Error::new(stream.span(), "Expected matching ident"))
                }
            } else {
                Err(Error::new(stream.span(), "Expected ident"))
            }
        })
    }
}

impl Parse for Ident {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let ident = syn::Ident::parse(input)?;
        let string = ident.to_string();
        Ok(Ident { ident, string })
    }
}

impl ToTokens for Ident {
    fn to_tokens(&self, tree: &mut TokenStream) {
        self.ident.to_tokens(tree)
    }
}

impl From<syn::Ident> for Ident {
    fn from(ident: syn::Ident) -> Ident {
        let string = ident.to_string();
        Ident { ident, string }
    }
}
