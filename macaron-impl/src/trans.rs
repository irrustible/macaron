use proc_macro2::{Literal, Punct, TokenTree, TokenStream};
use syn::{token, ext::IdentExt, parse::{Parse, ParseStream}, MacroDelimiter, Result, Token};
use crate::{*, parsing::*};
use quote::{TokenStreamExt, ToTokens};

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

impl Transcription {
    pub fn transcribe(&self, stream: &mut TokenStream, scope: &mut Scope) -> Result<()> {
        match self {
            Transcription::Group(g) =>
                self.transcribe_group(stream, scope, g)?,
            Transcription::MetaGroup(g) =>
                self.transcribe_metagroup(stream, scope, g)?,
            Transcription::MetaSplice(s) =>
                self.transcribe_metasplice(stream, scope, s)?,
            Transcription::MetaVar(v) =>
                self.transcribe_metavar(stream, scope, v)?,
            Transcription::Ident(i) => i.to_tokens(stream),
            Transcription::Punct(p) => stream.append(p.clone()),
        }
        Ok(())
    }

    fn transcribe_metavar(
        &self, stream: &mut TokenStream, scope: &mut Scope, var: &MetaVarTrans
    ) -> Result<()> {
        match scope.fragment(&var.name) {
            Some(f) => {
                f.to_tokens(stream);
                Ok(())
            },
            None => todo!(),
        }
    }

    fn transcribe_group(
        &self, stream: &mut TokenStream, scope: &mut Scope, group: &Group<Transcription>
    ) -> Result<()> {
        let mut inner_stream = TokenStream::new();
        for t in group.values.iter() {
            t.transcribe(&mut inner_stream, scope)?;
        }
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
                    let mut scope = Scope::Round(round.clone());
                    for t in group.values.iter() {
                        t.transcribe(stream, &mut scope)?;
                    }
                }
                Ok(())
            }
            None => todo!(),
        }
    }

    fn transcribe_metasplice(
        &self, stream: &mut TokenStream, scope: &mut Scope, group: &MetaSplice
    ) -> Result<()> {
        todo!()
    }
}

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
    if l.peek(syn::Ident) {
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
