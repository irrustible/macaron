use proc_macro2::{Literal, Punct, TokenTree};
use syn::{token, ext::IdentExt, parse::{Parse, ParseStream}, MacroDelimiter, Result, Token};
use crate::{*, parsing::*};

#[derive(Clone)]
pub struct Splice {
    dollar: Token![$],
    brace: token::Brace,
    expr: SpliceExpr,
}


#[derive(Clone)]
pub enum SpliceExpr {
    
}

#[derive(Clone)]
pub struct MetaSplice {
    pub dollar:      token::Dollar,
    pub bracket:     token::Bracket,
    pub name:        Ident,
    pub bracket2:    token::Bracket,
    pub values:      Vec<Transcription>,
}

#[derive(Clone)]
pub enum Transcription {
    Group(Group<Transcription>),
    MetaGroup(MetaGroup<Transcription>),
    MetaSplice(MetaSplice),
    MetaVar(MetaVarTrans),
    Ident(Ident),
    Punct(Punct),
}

// impl Transcription {
//     pub fn transcribe(&self, matches: &Matches) {
//     }
// }

#[derive(Clone)]
pub struct MetaVarTrans {
    pub dollar: Token![$],
    pub name: Ident,
}

fn peek_etc(input: ParseStream) -> bool {
    input.peek(Token![.]) && input.peek2(Token![.]) && input.peek3(Token![.])
}

impl Parse for Transcription {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![$]) {
            parse_meta_transcription(input)
        } else if input.peek(token::Paren) {
            let (paren, values) = paren_many(input)?;
            let delim = MacroDelimiter::Paren(paren);
            Ok(Transcription::Group(Group { delim, values }))
        } else if input.peek(token::Bracket) {
            let (bracket, values) = bracket_many(input)?;
            let delim = MacroDelimiter::Bracket(bracket);
            Ok(Transcription::Group(Group { delim, values }))
        } else if input.peek(token::Brace) {
            let (brace, values) = brace_many(input)?;
            let delim = MacroDelimiter::Brace(brace);
            Ok(Transcription::Group(Group { delim, values }))
        } else if input.peek(syn::Ident::peek_any) {
            let ident = syn::Ident::parse_any(input)?.into();
            Ok(Transcription::Ident(ident))
        } else {
            input.step(|cursor| {
                match cursor.token_tree() {
                    Some((TokenTree::Punct(p), next)) =>
                        Ok((Transcription::Punct(p), next)),
                    Some((it, _)) =>
                        panic!("Parsed non-punct in Transcription fallback: {}", it),
                    None => Err(cursor.error("end of input")),
                }
            })
        }
    }
}


// a metavar, metagroup or splice
fn parse_meta_transcription(input: ParseStream) -> Result<Transcription> {
    let dollar = input.parse::<Token![$]>()?;
    let l = input.lookahead1();
    // metavars and named metagroups have relaxed naming restrictions
    // because macro_rules does this for metavars already.
    if l.peek(syn::Ident::peek_any) {
        Ok(Transcription::MetaVar(MetaVarTrans {
            dollar, name: syn::Ident::parse_any(input)?.into(),
        }))
    } else if l.peek(token::Bracket) {
        let (bracket, name) = bracket_one(input, syn::Ident::parse_any)?;
        let l = input.lookahead1();
        if l.peek(token::Paren) {
            let (paren, values) = paren_many(input)?;
            let (separator, multiplier) = MetaGroup::parse_suffix(input)?;
            Ok(Transcription::MetaGroup(MetaGroup {
                dollar, bracket, paren, multiplier, values, separator, name: name.into(),
            }))
        } else if l.peek(token::Bracket) {
            let (bracket2, values) = bracket_many(input)?;
            Ok(Transcription::MetaSplice(MetaSplice {
                dollar, bracket, bracket2, values, name: name.into(),
            }))
        } else {
            Err(l.error())
        }
    } else if l.peek(token::Brace) {
        // splice
        todo!()
    } else {
        Err(l.error())
    }
}
