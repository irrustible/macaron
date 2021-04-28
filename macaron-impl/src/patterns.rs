use syn::{ext::IdentExt, bracketed, token, MacroDelimiter, Result, Token};
use syn::parse::{Error, Parse, ParseStream};
use crate::{*, parsing::*};
use proc_macro2::{TokenTree, Punct};

#[derive(Clone)]
pub enum Pattern {
    Group(Group<Pattern>),
    MetaGroup(MetaGroup<Pattern>),
    Fragment(FragPat),
    Literal(Literal),
    Ident(Ident),
    Punct(Punct),
}

impl Pattern {
    pub fn parse_match(&self, stream: ParseStream, captures: &mut Captures) -> Result<Match> {
        match self {
            Pattern::Group(g) => g.parse_match(stream, captures).map(Match::Group),
            Pattern::MetaGroup(group) => group.parse_match(stream),
                // Ok(Match::MetaGroup(MetaGroupMatcher { group, rounds: 0 })),
            Pattern::Fragment(f) => f.parse_match(stream).map(Match::Fragment),
            Pattern::Literal(l) => l.parse_match(stream).map(Match::Literal),
            Pattern::Ident(i) => i.parse_match(stream).map(Match::Ident),
            Pattern::Punct(p) => self.match_punct(p, stream),
        }
    }

    fn match_punct(&self, punct: &Punct, stream: ParseStream) -> Result<Match> {
        stream.step(|cursor| {
            if let Some((p,next)) = cursor.punct() {
                if p.as_char() == punct.as_char() {
                    Ok((Match::Punct(p), next))
                } else {
                    Err(Error::new(stream.span(), "Expected matching punct"))
                }
            } else {
                Err(Error::new(stream.span(), "Expected punct"))
            }
        })
    }
}

impl Parse for Pattern {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![$]) {
            parse_meta_pattern(input)
        } else if input.peek(token::Paren) {
            let (paren, values) = paren_many(input)?;
            let delim = MacroDelimiter::Paren(paren);
            Ok(Pattern::Group(Group { delim, values }))
        } else if input.peek(token::Bracket) {
             let (bracket, values) = bracket_many(input)?;
            let delim = MacroDelimiter::Bracket(bracket);
            Ok(Pattern::Group(Group { delim, values }))
        } else if input.peek(token::Brace) {
            let (brace, values) = brace_many(input)?;
            let delim = MacroDelimiter::Brace(brace);
            Ok(Pattern::Group(Group { delim, values }))
        } else if input.peek(syn::Ident::peek_any) {
            let ident = syn::Ident::parse_any(input)?;
            Ok(Pattern::Ident(ident.into()))
        } else if input.peek(syn::Lit) {
            let lit = input.parse::<Literal>()?;
            Ok(Pattern::Literal(lit.into()))
        } else {
            input.step(|cursor| {
                match cursor.token_tree() {
                    Some((TokenTree::Punct(p), next)) => Ok((Pattern::Punct(p), next)),
                    Some((it, _)) => panic!("Parsed non-punct in Pattern fallback: {}", it),
                    None => Err(cursor.error("end of input")),
                }
            })
        }
    }
}

// a metavar or metagroup
fn parse_meta_pattern(input: ParseStream) -> Result<Pattern> {
    let dollar = input.parse::<Token![$]>()?;
    let l = input.lookahead1();
    if l.peek(Token![:]) {
        Ok(Pattern::Fragment(FragPat {
            dollar, name: None,
            colon: input.parse()?,
            spec:  input.parse()?,
        }))
    } else if Ident::peekahead(&l) {
        let name = Some(input.parse::<Ident>()?);
        Ok(Pattern::Fragment(FragPat {
            dollar, name,
            colon: input.parse()?,
            spec:  input.parse()?,
        }))
    } else if l.peek(token::Bracket) {
        let content;
        let bracket = bracketed!(content in input);
        let name = content.call(syn::Ident::parse_any)?.into();
        if content.is_empty() {
            let (paren, values) = paren_many(input)?;
            let (separator, multiplier) = MetaGroup::parse_suffix(input)?;
            Ok(Pattern::MetaGroup(MetaGroup {
                dollar, bracket, name, paren, multiplier, values, separator, 
            }))
        } else {
            Err(content.error("Expected closing bracket!"))
        }
    } else {
        Err(l.error())
    }
}
