use auto_from::From;
use crate::ast::*;
use crate::diag::*;
use proc_macro2 as pm2;
use proc_macro2::{Delimiter, Punct, TokenStream, TokenTree};
use syn::{Ident, MacroDelimiter, parse::{Parse, ParseStream}};
use std::borrow::Cow;
use std::collections::HashMap;
use quote::ToTokens;

#[derive(Default)]
pub struct Program {
    macarons: HashMap<String, Macaron>,
}

impl Program {
    pub fn check(&self) -> Result<(), Vec<Diagnostic>> {
        let mut diag = vec!();
        for m in self.macarons.values() {
            if let Err(e) = m.check_rule_visibility() {
                diag.push(e);
            }
        }
        if diag.is_empty() {
            Ok(())
        } else {
            Err(diag)
        }
    }
}

#[derive(Clone)]
pub struct Macaron {
    public: bool,
    rules: Vec<Rule>,
}

impl Macaron {
    pub fn check_rule_visibility(&self) -> Result<(), Diagnostic> {
        let mut rules = self.rules.iter();
        if let Some(first) = rules.next() {
            let mut pub_spans = vec!();
            if let Some(pub_token) = first.pub_token {
                for rule in rules {
                    if !rule.pub_token.is_some() {
                        pub_spans.push(rule.macro_token.span);
                    }
                }
                if !pub_spans.is_empty() {
                    Err(Diagnostic::spanned(
                        pub_spans, Level::Error, "Missing pub"
                    ).span_note(
                        pub_token.span, "Original (pub) macro definition here"
                    ))
                } else { Ok(()) }
            } else {
                for rule in rules {
                    if let Some(pub_token) = rule.pub_token {
                        pub_spans.push(pub_token.span);
                    }
                }
                if !pub_spans.is_empty() {
                    Err(Diagnostic::spanned(
                        pub_spans, Level::Error, "Unexpected pub"
                    ).span_note(
                        first.macro_token.span, "Original (non-pub) macro definition here"
                    ))
                } else { Ok(()) }
            }
        } else {
            Ok(())
        }
    }
    // pub fn check_metavars(&self, diag: &mut Vec<Diagnostic>) {
    // }
    // pub fn check_multiplicities(&self, diag: &mut Vec<Diagnostic>) {
    //     for rule in self.rules.iter() {
    //         let mut p = rule.patterns.iter();
    //         let mut t = rule.body.iter();
    //     }            
    // }
}

impl From<Rule> for Macaron {
    fn from(rule: Rule) -> Macaron {
        Macaron {
            public: rule.pub_token.is_some(),
            rules: vec!(rule),
        }
    }
}

#[derive(Clone)]
pub struct Binding {
    ident: Ident,
    tokens: Vec<TokenTree>,
}

#[derive(Clone, From)]
pub enum Match {
    Binding(Binding),
    Token(TokenTree),
    Ident(Ident),
    Punct(Punct),
    Group(GroupMatch),
    MetaGroup(MetaGroupMatch),
}

#[derive(Clone)]
pub struct GroupMatch {
    pub group: pm2::Group,
    pub delim: MacroDelimiter,
    pub matches: Vec<Match>,
}

#[derive(Clone)]
pub enum MetaGroupMatch {
    One(Vec<Match>),
    ZeroOne(Option<Vec<Match>>),
    ZeroMany(Vec<Vec<Match>>),
    OneMany(Vec<Vec<Match>>),
}

pub enum MatchError<'a> {
    Empty(Cow<'a, Pattern>),
    Leftovers(Cow<'a, [TokenTree]>),
    NoMatch(Cow<'a, Pattern>, Cow<'a, [TokenTree]>),
}

impl<'a> MatchError<'a> {
    fn rescope<'b: 'a>(self) -> MatchError<'b> {
        match self {
            MatchError::Empty(p) => MatchError::Empty(Cow::Owned(p.into_owned())),
            MatchError::Leftovers(t) => MatchError::Leftovers(Cow::Owned(t.into_owned())),
            MatchError::NoMatch(p, t) =>
                MatchError::NoMatch(Cow::Owned(p.into_owned()), Cow::Owned(t.into_owned())),
        }
    }
}

pub struct Scoop<T> {
    it: T,
    rest: Vec<TokenTree>,
}

pub struct InnerAttributes(Vec<syn::Attribute>);

pub struct OuterAttributes(Vec<syn::Attribute>);

impl Parse for InnerAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(InnerAttributes(input.call(syn::Attribute::parse_inner)?))
    }
}

impl ToTokens for InnerAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for i in self.0.iter() {
            i.to_tokens(tokens);
        }
    }
}

impl Parse for OuterAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(OuterAttributes(input.call(syn::Attribute::parse_outer)?))
    }
}

impl ToTokens for OuterAttributes {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        for i in self.0.iter() {
            i.to_tokens(tokens);
        }
    }
}

impl<T: Parse> Parse for Scoop<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let it = input.parse()?;
        let rest = input.cursor().token_stream().into_iter().collect();
        Ok(Scoop { it, rest })
    }
}

struct Matcher<'a> {
    patterns: &'a [Pattern],
    tokens:   &'a [TokenTree],
}

impl<'a> Matcher<'a> {
    fn match_metagroup_round(
        &mut self, g: &'a MetaGroup<Pattern>
    ) -> Result<Vec<Match>, MatchError<'a>> {
        let patterns = g.values.as_slice();
        let mut matcher = Matcher { patterns, tokens: self.tokens };
        let mut matches = vec!();
        while let Some(m) = matcher.next() {
            match m {
                Ok(m) => { matches.push(m); }
                Err(e) => { return Err(e.rescope()); }
            }
        }
        self.tokens = matcher.tokens;
        Ok(matches)
    }

    // TODO: Separators
    fn match_metagroup(
        &mut self, pat: &'a Pattern, g: &'a MetaGroup<Pattern>
    ) -> Result<Match, MatchError<'a>> {
        match g.multiplier {
            Multiplier::One(_) => {
                let vals = self.match_metagroup_round(g)?;
                self.patterns = &self.patterns[1..];
                Ok(Match::MetaGroup(MetaGroupMatch::One(vals)))
            }
            Multiplier::ZeroOne(_) => {
                match self.match_metagroup_round(g) {
                    Ok(vals) => {
                        self.patterns = &self.patterns[1..];
                        Ok(Match::MetaGroup(MetaGroupMatch::ZeroOne(Some(vals))))
                    }
                    Err(_) => {
                        self.patterns = &self.patterns[1..];
                        Ok(Match::MetaGroup(MetaGroupMatch::ZeroOne(None)))
                    }
                }
            }
            Multiplier::ZeroMany(_) => {
                let mut results = vec!();
                while let Ok(vals) = self.match_metagroup_round(g) {
                    results.push(vals);
                }
                Ok(Match::MetaGroup(MetaGroupMatch::ZeroMany(results)))
            }
            Multiplier::OneMany(_) => {
                let mut results = vec!();
                while let Ok(vals) = self.match_metagroup_round(g) {
                    results.push(vals);
                }
                if results.is_empty() {
                    Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
                } else {
                    Ok(Match::MetaGroup(MetaGroupMatch::ZeroMany(results)))
                }
            }
        }
    }

    fn match_metavar_token<T: Parse + syn::token::Token>(
        &mut self, pat: &'a Pattern, var: &'a MetaVarPattern
     ) -> Result<Match, MatchError<'a>> {
        if let Ok(_) = syn::parse2::<T>(TokenStream::from(self.tokens[0].clone())) {
            let ident = var.name.clone();
            let tokens = vec!(self.tokens[0].clone());
            self.tokens = &self.tokens[1..];
            Ok(Match::Binding(Binding { ident, tokens }))
        } else {
            Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
        }
    }

    fn match_metavar_scoop<T: Parse + ToTokens>(
        &mut self, pat: &'a Pattern, var: &MetaVarPattern
    ) -> Result<Match, MatchError<'a>> {
        let tokens = self.tokens.to_owned().into_iter().collect();
        if let Ok(path) = syn::parse2::<Scoop<T>>(tokens) {
            let drop = self.tokens.len() - path.rest.len();
            self.tokens = &self.tokens[drop..];
            let mut stream = TokenStream::new();
            path.it.to_tokens(&mut stream);
            let ident = var.name.clone();
            Ok(Match::Binding(Binding { ident, tokens: stream.into_iter().collect() }))
        } else {
            Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
        }
    }

    fn match_metavar(
        &mut self, pat: &'a Pattern, var: &'a MetaVarPattern
    ) -> Result<Match, MatchError<'a>> {
        if let Some((tt, rest)) = self.tokens.split_first() {
            match var.spec {
                FragSpec::Block(_)           => self.match_metavar_scoop::<syn::Block>(pat, var),
                FragSpec::Expr (_)           => self.match_metavar_scoop::<syn::Expr>(pat, var),
                FragSpec::Item (_)           => self.match_metavar_scoop::<syn::Item>(pat, var),
                FragSpec::Lifetime(_)        => self.match_metavar_scoop::<syn::Lifetime>(pat, var),
                FragSpec::Meta(_)            => self.match_metavar_scoop::<syn::Meta>(pat, var),
                FragSpec::Pat (_)            => self.match_metavar_scoop::<syn::Pat>(pat, var),
                FragSpec::Path(_)            => self.match_metavar_scoop::<syn::Path>(pat, var),
                FragSpec::Stmt(_)            => self.match_metavar_scoop::<syn::Stmt>(pat, var),
                FragSpec::Ty(_)              => self.match_metavar_scoop::<syn::Type>(pat, var),
                FragSpec::Vis(_)             => self.match_metavar_scoop::<syn::Visibility>(pat, var),
                FragSpec::Name(_)            => self.match_metavar_token::<Ident>(pat, var),
                FragSpec::OuterAttrs(_)      => self.match_metavar_scoop::<OuterAttributes>(pat, var),
                FragSpec::InnerAttrs(_)      => self.match_metavar_scoop::<InnerAttributes>(pat, var),
                FragSpec::GenericArgument(_) => self.match_metavar_scoop::<syn::GenericArgument>(pat, var),
                FragSpec::GenericParam(_)    => self.match_metavar_scoop::<syn::GenericParam>(pat, var),
                FragSpec::Tt  (_) => {
                    self.tokens = rest;
                    Ok(Match::Binding(Binding { ident: var.name.clone(), tokens: vec!(tt.clone()) }))
                }
                FragSpec::Ident(_)    => {
                    if let TokenTree::Ident(_) = tt {
                        self.tokens = rest;
                        Ok(Match::Binding(Binding { ident: var.name.clone(), tokens: vec!(tt.clone()) }))
                    } else {
                        Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
                    }
                }
                FragSpec::Literal(_)  => {
                    if let TokenTree::Literal(_) = tt {
                        self.tokens = rest;
                        Ok(Match::Binding(Binding { ident: var.name.clone(), tokens: vec!(tt.clone()) }))
                    } else {
                        Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
                    }
                }
            }
        } else {
            Err(MatchError::Empty(Cow::Borrowed(pat)))
        }
    }
            
    fn match_delim(&self, delim: &Delimiter, mdelim: &MacroDelimiter) -> bool {
        match (delim, mdelim) {
            (Delimiter::Parenthesis, MacroDelimiter::Paren(_))   => true,
            (Delimiter::Brace,       MacroDelimiter::Brace(_))   => true,
            (Delimiter::Bracket,     MacroDelimiter::Bracket(_)) => true,
            _ => false,
        }
    }

    fn match_group(&mut self, pat: &'a Pattern, g: &'a Group<Pattern>) -> Result<Match, MatchError<'a>> {
        if let Some((tt, rest)) = self.tokens.split_first() {
            match tt {
                TokenTree::Group(h) => {
                    if self.match_delim(&h.delimiter(), &g.delim) {
                        let tokens: Vec<TokenTree> = h.stream().into_iter().collect();
                        let matcher = Matcher {
                            patterns: g.values.as_slice(),
                            tokens: tokens.as_slice(),
                        };
                        let mut matches = vec!();
                        for m in matcher {
                            m.map(|m| matches.push(m)).map_err(|e| e.rescope())?;
                        }
                        self.tokens = rest;
                        Ok(Match::Group(GroupMatch {
                            matches,
                            delim: g.delim.clone(),
                            group: h.clone(),
                        }))
                    } else {
                        Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
                    }
                }
                _ => Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens))),
            }
        } else {
            Err(MatchError::Empty(Cow::Borrowed(pat)))
        }
    }

    fn match_ident(&mut self, pat: &'a Pattern, i: &'a Ident) -> Result<Match, MatchError<'a>> {
        match self.tokens.split_first() {
            Some((TokenTree::Ident(j),rest)) if j.to_string() == i.to_string() => {
                self.tokens = rest;
                Ok(Match::Ident(j.clone()))
            }
            other => self.nope(pat, other),
        }
    }

    fn match_punct(&mut self, pat: &'a Pattern, p: &'a Punct) -> Result<Match, MatchError<'a>> {
        match self.tokens.split_first() {
            Some((TokenTree::Punct(q),rest)) if q.as_char() == p.as_char() => {
                self.tokens = rest;
                Ok(Match::Punct(q.clone()))
            }
            other => self.nope(pat, other),
        }
    }

    fn match_end(&mut self) -> Option<Result<Match, MatchError<'a>>> {
        if self.tokens.is_empty() {
            None
        } else {
            Some(Err(MatchError::Leftovers(self.tokens.into())))
        }
    }

    fn nope(
        &self, pat: &'a Pattern, tt: Option<(&'a TokenTree, &'a [TokenTree])>
    ) -> Result<Match, MatchError<'a>> {
        if tt.is_some() {
            Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
        } else {
            Err(MatchError::Empty(Cow::Borrowed(pat)))
        }
    }
}

impl<'a> Iterator for Matcher<'a> {
    type Item = Result<Match, MatchError<'a>>;
    fn next(&mut self) -> Option<Result<Match, MatchError<'a>>> {
        if let Some(pat) = self.patterns.first() {
            match pat {
                Pattern::Group(g) => Some(self.match_group(pat, g)),
                Pattern::MetaGroup(g) => Some(self.match_metagroup(pat, g)),
                Pattern::MetaVar(v) => Some(self.match_metavar(pat, v)),
                Pattern::Ident(i) => Some(self.match_ident(pat, i)),
                Pattern::Punct(p) => Some(self.match_punct(pat, p)),
            }
        } else {
            self.match_end()
        }
    }
}

impl Program {
    pub fn from_rules(rules: Vec<Rule>) {
        let mut program = Program::default();
        // let mut warnings = Vec::new();
        for rule in rules {
            let name = rule.name.to_string();
            if let Some(m) = program.macarons.get_mut(&name) {
                m.rules.push(rule);
            } else {
                program.macarons.insert(name, rule.into());
            }
        }
    }
}
