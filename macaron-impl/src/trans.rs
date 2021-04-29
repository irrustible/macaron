use proc_macro2::{Literal, Punct, TokenTree, TokenStream};
use syn::{token, ext::IdentExt, parse::{Parse, ParseStream}, MacroDelimiter, Result, Token};
use crate::{*, parsing::*};
use quote::{TokenStreamExt, ToTokens};
use std::borrow::Cow;

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
pub enum Transcription {
    Group(Group<Transcription>),
    MetaGroup(MetaGroup<Transcription>),
    Splice(Splice),
    Fragment(FragmentTrans),
    Literal(Literal),
    Ident(Ident),
    Punct(Punct),
}

impl ToTokens for Transcription {
    fn to_tokens(&self, stream: &mut TokenStream) {
        match self {
            Transcription::Group(g) => g.to_tokens(stream),
            Transcription::MetaGroup(g) => g.to_tokens(stream),
            Transcription::Splice(s) => todo!(),
            Transcription::Fragment(f) => f.to_tokens(stream),
            Transcription::Ident(i) => i.to_tokens(stream),
            Transcription::Literal(l) => l.to_tokens(stream),
            Transcription::Punct(p) => stream.append(p.clone()),
        }
    }
}

impl Transcription {
    pub fn transcribe(&self, stream: &mut TokenStream, scope: &mut Scope) -> Result<()> {
        match self {
            Transcription::Group(g) =>
                self.transcribe_group(stream, scope, g)?,
            Transcription::MetaGroup(g) =>
                self.transcribe_metagroup(stream, scope, g)?,
            Transcription::Splice(s) =>
                self.transcribe_splice(stream, scope, s)?,
            Transcription::Fragment(v) =>
                self.transcribe_metavar(stream, scope, v)?,
            Transcription::Ident(i) => i.to_tokens(stream),
            Transcription::Literal(l) => l.to_tokens(stream),
            Transcription::Punct(p) => stream.append(p.clone()),
        }
        Ok(())
    }

    fn transcribe_metavar(
        &self, stream: &mut TokenStream, scope: &mut Scope, var: &FragmentTrans
    ) -> Result<()> {
        match scope.fragment(&var.name) {
            Some(f) => {
                f.to_tokens(stream);
                Ok(())
            },
            None => Err(syn::Error::new(var.name.ident.span(), "Fragment not found")),
        }
    }

    fn transcribe_group(
        &self, stream: &mut TokenStream, scope: &mut Scope, group: &Group<Transcription>
    ) -> Result<()> {
        let mut inner_stream = TokenStream::new();
        for t in group.values.iter() {
            t.transcribe(&mut inner_stream, scope)?;
        }
        // This is less efficient than it could be, because syn.
        let inner = |stream: &mut TokenStream| stream.append_all(inner_stream);
        match group.delim {
            MacroDelimiter::Brace(b) => b.surround(stream, inner),
            MacroDelimiter::Bracket(b) => b.surround(stream, inner),
            MacroDelimiter::Paren(p) => p.surround(stream, inner),
        }
        Ok(())
    }

    fn transcribe_metagroup(
        &self, stream: &mut TokenStream, scope: &mut Scope, group: &MetaGroup<Transcription>
    ) -> Result<()> {
        match scope.group(&group.name) {
            Some(g) => {
                for round in g.rounds.iter() {
                    let mut scope = Scope::Round(Cow::Borrowed(round));
                    for t in group.values.iter() {
                        t.transcribe(stream, &mut scope)?;
                    }
                }
                Ok(())
            }
            None => Err(syn::Error::new(group.name.ident.span(), "Metagroup not found")),
        }
    }

    fn transcribe_splice(
        &self, stream: &mut TokenStream, scope: &mut Scope, splice: &Splice
    ) -> Result<()> {
        todo!()
    }
}

#[derive(Clone)]
pub struct FragmentTrans {
    pub dollar: Token![$],
    pub name: Ident,
}

impl ToTokens for FragmentTrans {
    fn to_tokens(&self, stream: &mut TokenStream) {
        self.dollar.to_tokens(stream);
        self.name.to_tokens(stream);
    }
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
    if l.peek(syn::Ident) {
        Ok(Transcription::Fragment(FragmentTrans {
            dollar, name: syn::Ident::parse_any(input)?.into(),
        }))
    } else if l.peek(syn::Lit) {
        input.parse().map(Transcription::Literal)
    } else if l.peek(token::Bracket) {
        let (bracket, name) = bracket_one(input, syn::Ident::parse_any)?;
        let (paren, values) = paren_many(input)?;
        let (separator, multiplier) = MetaGroup::parse_suffix(input)?;
        Ok(Transcription::MetaGroup(MetaGroup {
            dollar, bracket, paren, multiplier, values, separator, name: name.into(),
        }))
    } else if l.peek(token::Brace) {
        // let (brace, values) = brace_many(input)?;
        // Ok(Transcription::Splice(Splice {
        //     dollar, bracket, brace, values, name: name.into(),
        // }))
        todo!()
    } else {
        Err(l.error())
    }
}
