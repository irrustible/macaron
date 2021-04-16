use syn::*;
use proc_macro2::{Punct, TokenTree};
use syn::ext::IdentExt;
use syn::parse::{Parse, ParseStream};
use crate::parsing::*;

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

#[derive(Clone)]
pub enum Pattern {
    Group(Group<Pattern>),
    MetaGroup(MetaGroup<Pattern>),
    NamedMetaGroup(NamedMetaGroup<Pattern>),
    MetaVar(MetaVarPattern),
    Ident(Ident),
    Punct(Punct),
}

#[derive(Clone)]
pub enum Transcription {
    Group(Group<Transcription>),
    MetaGroup(MetaGroup<Transcription>),
    NamedMetaGroup(NamedMetaGroup<Transcription>),
    MetaSplice(MetaSplice),
    MetaVar(MetaVarTrans),
    Ident(Ident),
    Punct(Punct),
}

#[derive(Clone)]
pub enum Multiplier {
    One     (Token![$]),
    ZeroOne (Token![?]),
    ZeroMany(Token![*]),
    OneMany (Token![+]),
}

#[derive(Clone)]
pub struct Group<T> {
    pub delim: MacroDelimiter,
    pub values: Vec<T>,
}

#[derive(Clone)]
pub struct MetaGroup<T> {
    pub dollar:     token::Dollar,
    pub paren:      token::Paren,
    pub separator:  Option<Separator>,
    pub multiplier: Multiplier,
    pub values:     Vec<T>,
}

#[derive(Clone)]
pub struct NamedMetaGroup<T> {
    pub dollar:     token::Dollar,
    pub bracket:    token::Bracket,
    pub name:       Ident,
    pub paren:      token::Paren,
    pub separator:  Option<Separator>,
    pub multiplier: Multiplier,
    pub values:     Vec<T>,
}

#[derive(Clone)]
pub struct MetaSplice {
    pub dollar:     token::Dollar,
    pub bracket:    token::Bracket,
    pub name:       Ident,
    pub bracket2:   token::Bracket,
    pub values:     Vec<Transcription>,
}

#[derive(Clone)]
pub struct MetaVarTrans {
    pub dollar: Token![$],
    pub name: Ident,
}

#[derive(Clone)]
pub struct MetaVarPattern {
    pub dollar: Token![$],
    pub name: Ident,
    pub colon: Token![:],
    pub spec: FragSpec,
}

// technically, anything except a delimiter or multiplier, but we'll
// start here...
#[derive(Clone)]
pub enum Separator {
    Comma(Token![,]),
    Semicolon(Token![;]),
}

mod kw {
    // builtin fragspecs
    syn::custom_keyword!(block);
    syn::custom_keyword!(expr);
    syn::custom_keyword!(ident);
    syn::custom_keyword!(item);
    syn::custom_keyword!(lifetime);
    syn::custom_keyword!(literal);
    syn::custom_keyword!(meta);
    syn::custom_keyword!(pat);
    syn::custom_keyword!(path);
    syn::custom_keyword!(stmt);
    syn::custom_keyword!(tt);
    syn::custom_keyword!(ty);
    syn::custom_keyword!(vis);
    // extra fragspecs
    syn::custom_keyword!(attrs);
    syn::custom_keyword!(inattrs);
    syn::custom_keyword!(name);
    syn::custom_keyword!(genarg);
    syn::custom_keyword!(genparam);
}

#[derive(Clone)]
pub enum FragSpec {
    /// [builtin]
    Block   (kw::block),
    /// [builtin]
    Expr    (kw::expr),
    /// [builtin] An identifier or keyword
    Ident   (kw::ident),
    /// [builtin]
    Item    (kw::item),
    /// [builtin]
    Lifetime(kw::lifetime),
    /// [builtin]
    Literal (kw::literal),
    /// [builtin]
    Meta    (kw::meta),
    /// [builtin]
    Pat     (kw::pat),
    /// [builtin]
    Path    (kw::path),
    /// [builtin]
    Stmt    (kw::stmt),
    /// [builtin]
    Tt      (kw::tt),
    /// [builtin]
    Ty      (kw::ty),
    /// [builtin]
    Vis     (kw::vis),

    /// [custom] Attributes
    OuterAttrs(kw::attrs),
    /// [custom] Bang attributes
    InnerAttrs(kw::inattrs),
    /// [custom] Actually an identifier, not a keyword
    Name(kw::name),
    /// [custom] Generic Arguments
    GenericArgument(kw::genarg),
    /// [custom] Generic Parameter
    GenericParam(kw::genparam),
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

impl Parse for Rule {
    fn parse(input: ParseStream) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(Token![pub]) {
            let pub_token = Some(input.parse::<Token![pub]>()?);
            let macro_token = input.parse::<Token![macro]>()?;
            let name = input.parse::<Ident>()?;
            let (paren, patterns) = paren_many(input)?;
            let (brace, body) = brace_many(input)?;
            Ok(Rule {
                pub_token, macro_token, name, paren, patterns, brace, body,
            })
        } else if l.peek(Token![macro]) {
            let macro_token = input.parse::<Token![macro]>()?;
            let name = input.parse::<Ident>()?;
            let (paren, patterns) = paren_many(input)?;
            let (brace, body) = brace_many(input)?;
            Ok(Rule {
                macro_token, name, paren, patterns, brace, body, pub_token: None,
            })
        } else {
            Err(l.error())
        }
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
        } else if input.peek(Ident::peek_any) {
            let ident = input.call(Ident::parse_any)?;
            Ok(Pattern::Ident(ident))
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
        } else if input.peek(Ident::peek_any) {
            let ident = input.call(Ident::parse_any)?;
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

impl Parse for FragSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        let l = input.lookahead1();
        if l.peek(kw::block) {
            input.parse().map(FragSpec::Block)
        } else if l.peek(kw::expr) {
            input.parse().map(FragSpec::Expr)
        } else if l.peek(kw::ident) {
            input.parse().map(FragSpec::Ident)
        } else if l.peek(kw::item) {
            input.parse().map(FragSpec::Item)
        } else if l.peek(kw::lifetime) {
            input.parse().map(FragSpec::Lifetime)
        } else if l.peek(kw::literal) {
            input.parse().map(FragSpec::Literal)
        } else if l.peek(kw::meta) {
            input.parse().map(FragSpec::Meta)
        } else if l.peek(kw::pat) {
            input.parse().map(FragSpec::Pat)
        } else if l.peek(kw::path) {
            input.parse().map(FragSpec::Path)
        } else if l.peek(kw::stmt) {
            input.parse().map(FragSpec::Stmt)
        } else if l.peek(kw::tt) {
            input.parse().map(FragSpec::Tt)
        } else if l.peek(kw::ty) {
            input.parse().map(FragSpec::Ty)
        } else if l.peek(kw::vis) {
            input.parse().map(FragSpec::Vis)
        } else if l.peek(kw::attrs) {
            input.parse().map(FragSpec::OuterAttrs)
        } else if l.peek(kw::inattrs) {
            input.parse().map(FragSpec::InnerAttrs)
        } else if l.peek(kw::name) {
            input.parse().map(FragSpec::Name)
        } else if l.peek(kw::genarg) {
            input.parse().map(FragSpec::GenericArgument)
        } else if l.peek(kw::genparam) {
            input.parse().map(FragSpec::GenericParam)
        } else {
            Err(l.error())
        }
    }
}

fn parse_metagroup_suffix(input: ParseStream) -> Result<(Option<Separator>, Multiplier)> {
    let l = input.lookahead1();
    if let Ok(multiplier) = input.parse::<Multiplier>() {
        Ok((None,multiplier))
    } else if let Ok(separator) = input.parse::<Separator>() {
        let multiplier = input.parse::<Multiplier>()?;
        Ok((Some(separator),multiplier))
    } else {
        Err(l.error())
    }
}


// a metavar, metagroup or named metagroup
fn parse_meta_pattern(input: ParseStream) -> Result<Pattern> {
    let dollar = input.parse::<Token![$]>()?;
    let l = input.lookahead1();
    // metavars and named metagroups have relaxed naming restrictions
    // because macro_rules does this for metavars already.
    if l.peek(Ident::peek_any) {
        Ok(Pattern::MetaVar(MetaVarPattern {
            dollar,
            name:  input.call(Ident::parse_any)?,
            colon: input.parse()?,
            spec:  input.parse()?,
        }))
    } else if l.peek(token::Paren) {
        let (paren, values) = paren_many(input)?;
        let (separator, multiplier) = parse_metagroup_suffix(input)?;
        Ok(Pattern::MetaGroup(MetaGroup {
            dollar, paren, multiplier, values, separator, 
        }))
    } else if l.peek(token::Bracket) {
        let content;
        let bracket = bracketed!(content in input);
        let name = content.call(Ident::parse_any)?;
        if content.is_empty() {
            let (paren, values) = paren_many(input)?;
            let (separator, multiplier) = parse_metagroup_suffix(input)?;
            Ok(Pattern::NamedMetaGroup(NamedMetaGroup {
                dollar, bracket, name, paren, multiplier, values, separator, 
            }))
        } else {
            Err(content.error("Expected closing bracket!"))
        }
    } else {
        Err(l.error())
    }
}

// a metavar, metagroup, named metagroup or macaron interpolation
fn parse_meta_transcription(input: ParseStream) -> Result<Transcription> {
    let dollar = input.parse::<Token![$]>()?;
    let l = input.lookahead1();
    // metavars and named metagroups have relaxed naming restrictions
    // because macro_rules does this for metavars already.
    if l.peek(Ident::peek_any) {
        Ok(Transcription::MetaVar(MetaVarTrans {
            dollar,
            name:  input.call(Ident::parse_any)?,
        }))
    } else if l.peek(token::Paren) {
        let (paren, values) = paren_many(input)?;
        let (separator, multiplier) = parse_metagroup_suffix(input)?;
        Ok(Transcription::MetaGroup(MetaGroup {
            dollar, paren, multiplier, values, separator, 
        }))
    } else if l.peek(token::Bracket) {
        let (bracket, name) = bracket_one(input, Ident::parse_any)?;
        let l = input.lookahead1();
        if l.peek(token::Paren) {
            let (paren, values) = paren_many(input)?;
            let (separator, multiplier) = parse_metagroup_suffix(input)?;
            Ok(Transcription::NamedMetaGroup(NamedMetaGroup {
                dollar, bracket, name, paren, multiplier, values, separator, 
            }))
        } else if l.peek(token::Bracket) {
            let (bracket2, values) = bracket_many(input)?;
            Ok(Transcription::MetaSplice(MetaSplice {
                dollar, bracket, name, bracket2, values,
            }))
        } else {
            Err(l.error())
        }
    } else {
        Err(l.error())
    }
}
