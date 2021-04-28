use auto_from::From;
use crate::ast::*;
use crate::ast::Ident;
use crate::diag::*;
use proc_macro2 as pm2;
use proc_macro2::{Delimiter, Punct, Span, TokenStream, TokenTree};
use syn::{bracketed, parenthesized, MacroDelimiter};
use syn::parse::{Parse, ParseStream};
use std::borrow::{Borrow, Cow};
use std::collections::HashMap;
// use quote::ToTokens;

/// Performs the expansion of a macaron call. Receives bracketed
/// program, bracketed call expression.
pub struct Expand {
    // program: Program,
    // fun: Ident,
    // args: Vec<Token
}

// impl Parse for Expand {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         // We take two bracketed expressions
//         let ruleset; let expr; let args;
//         let _bracket = bracketed!(ruleset in input);
//         let _bracket = bracketed!(expr in input);
//         // Read the program definitions.
//         let ruleset = Ruleset::parse(&ruleset)?;
//         // Then we have the call expression, enclosed in brackets.
//         // Find the macaron they're calling and get its rules
//         let ident = Ident::parse(&expr)?;
//         let macaron = ident.to_string();
//         let _paren = parenthesized!(args in input);
//         let rules: Vec<&Rule> = ruleset.rules.iter()
//             .filter(|r| r.name_string.as_str() == &macaron).collect();
//         if let Some(rule) = rules.first() {
//             if rule.pub_token.is_some() {
//                 // huzzah, we can match each rule in turn.
//                 for r in rules.iter() {
//                     if let match_patterns(&args, rule.patterns.as_slice());
//                 }
//                 Err(syn::parse::Error::new(
//                     Span::call_site(), format!("no match for macaron {}", &macaron)
//                 ))
//             } else {
//                 Err(syn::parse::Error::new(
//                     Span::call_site(), format!("the macaron {} is private", &macaron)
//                 ))
//             }
//         } else {
//             Err(syn::parse::Error::new(
//                 Span::call_site(), format!("the macaron {} is undefined", &macaron)
//             ))
//         }
//     }
// }

// #[derive(Clone)]
// pub enum Match<'a> {
//     MetaGroup(MatchGroup<'a>),
//     MetaVar()
// }

// #[derive(Clone)]
// pub struct Matches<'a>(Vec<(&'a Ident, Match<'a>)>);

// #[derive(Clone)]
// pub struct MatchGroup<'a> {
//     group: &'a MetaGroup<Pattern>,
//     matches: Matches<'a>,
// }

// pub struct 

// fn match_patterns<'a>(
//     stream: ParseStream<'a>,
//     patterns: &'a [Pattern]
// ) -> Result<Vec<Match<'a>>, Vec<Diagnostic>> {
//     let stream2 = stream.fork();
//     for p in patterns.iter() {
//         let c = stream2.cursor();
//         match p {
//             Pattern::Etc(e)       => todo!(),
//             Pattern::Group(g)     => todo!(),
//             Pattern::MetaGroup(g) => todo!(),
//             Pattern::MetaVar(v)   => todo!(),
//             Pattern::Ident(i)     => todo!(),
//             Pattern::Punct(p)     => todo!(),
//             Pattern::Literal(l)   => todo!(),
//         }
//     }
//     todo!()
// }

// pub struct Diagnostics(Vec<Diagnostic>);

// fn match_pattern<'a>(
//     stream: ParseStream<'a>,
//     patterns: &'a [Pattern]
// ) -> Result<Matches<'a>, Diagnostics> {
//     match p {
//         Pattern::Etc(e)       => todo!(),
//         Pattern::Group(g)     => todo!(),
//         Pattern::MetaGroup(g) => todo!(),
//         Pattern::MetaVar(v)   => todo!(),
//         Pattern::Ident(i)     => todo!(),
//         Pattern::Punct(p)     => todo!(),
//         Pattern::Literal(l)   => todo!(),
//     }
// }

// fn match_etc(
//     stream: ParseStream, rule: &Etc
// ) -> Result<Vec<Match<'a>>, Vec<Diagnostic>> {
    
// }
// fn match_group(stream: ParseStream, rule: &Rule) {
// }
// fn match_metagroup(stream: ParseStream, rule: &Rule) {
// }
// fn match_metavar(stream: ParseStream, rule: &Rule) {
// }
// fn match_ident(stream: ParseStream, rule: &Rule) {
    
// }
// fn match_punct(stream: ParseStream, rule: &Rule) {
// }

//         if let Some(pat) = self.patterns.first() {
//             let ret = match pat {
//             };
//             if ret.is_ok() {
//                 self.patterns = &self.patterns[1..];
//             }
//             Some(ret)
//         } else {
//             self.match_end()
//         }

// #[derive(Default)]
// pub struct Program {
//     macarons: HashMap<String, Macaron>,
// }

// impl Program {
//     pub fn check(&self) -> Result<(), Vec<Diagnostic>> {
//         let mut diag = vec!();
//         for m in self.macarons.values() {
//             if let Err(e) = m.check_rule_visibility() {
//                 diag.push(e);
//             }
//         }
//         if diag.is_empty() {
//             Ok(())
//         } else {
//             Err(diag)
//         }
//     }
// }

// #[derive(Clone)]
// pub struct Macaron {
//     public: bool,
//     rules: Vec<Rule>,
// }

// impl Macaron {
//     pub fn check_rule_visibility(&self) -> Result<(), Diagnostic> {
//         let mut rules = self.rules.iter();
//         if let Some(first) = rules.next() {
//             let mut pub_spans = vec!();
//             if let Some(pub_token) = first.pub_token {
//                 for rule in rules {
//                     if !rule.pub_token.is_some() {
//                         pub_spans.push(rule.macro_token.span);
//                     }
//                 }
//                 if !pub_spans.is_empty() {
//                     Err(Diagnostic::spanned(
//                         pub_spans, Level::Error, "Missing pub"
//                     ).span_note(
//                         pub_token.span, "Original (pub) macro definition here"
//                     ))
//                 } else { Ok(()) }
//             } else {
//                 for rule in rules {
//                     if let Some(pub_token) = rule.pub_token {
//                         pub_spans.push(pub_token.span);
//                     }
//                 }
//                 if !pub_spans.is_empty() {
//                     Err(Diagnostic::spanned(
//                         pub_spans, Level::Error, "Unexpected pub"
//                     ).span_note(
//                         first.macro_token.span, "Original (non-pub) macro definition here"
//                     ))
//                 } else { Ok(()) }
//             }
//         } else {
//             Ok(())
//         }
//     }
//     // pub fn check_metavars(&self, diag: &mut Vec<Diagnostic>) {
//     // }
//     // pub fn check_multiplicities(&self, diag: &mut Vec<Diagnostic>) {
//     //     for rule in self.rules.iter() {
//     //         let mut p = rule.patterns.iter();
//     //         let mut t = rule.body.iter();
//     //     }            
//     // }
// }

// impl From<Rule> for Macaron {
//     fn from(rule: Rule) -> Macaron {
//         Macaron {
//             public: rule.pub_token.is_some(),
//             rules: vec!(rule),
//         }
//     }
// }

// #[derive(Clone)]
// pub struct Binding {
//     ident: Ident,
//     tokens: Vec<TokenTree>,
// }

#[derive(Clone, From)]
pub enum Match {
//     Binding(Binding),
//     Token(TokenTree),
    Ident(Ident),
//     Punct(Punct),
//     Group(GroupMatch),
//     MetaGroup(MetaGroupMatch),
}

// impl From<syn::Ident> for Match {
//     fn from(i: syn::Ident) -> Self {
//         Match::Ident(Ident::from(i))
//     }
// }

// #[derive(Clone)]
// pub struct GroupMatch {
//     pub group: pm2::Group,
//     pub delim: MacroDelimiter,
//     pub matches: Vec<Match>,
// }

// #[derive(Clone)]
// pub enum MetaGroupMatch {
//     One(Vec<Match>),
//     ZeroOne(Option<Vec<Match>>),
//     ZeroMany(Vec<Vec<Match>>),
//     OneMany(Vec<Vec<Match>>),
// }

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

// pub struct Scoop<T> {
//     it: T,
//     rest: Vec<TokenTree>,
// }

// pub struct InnerAttributes(Vec<syn::Attribute>);

// pub struct OuterAttributes(Vec<syn::Attribute>);

// impl Parse for InnerAttributes {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         Ok(InnerAttributes(input.call(syn::Attribute::parse_inner)?))
//     }
// }

// impl ToTokens for InnerAttributes {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         for i in self.0.iter() {
//             i.to_tokens(tokens);
//         }
//     }
// }

// impl Parse for OuterAttributes {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         Ok(OuterAttributes(input.call(syn::Attribute::parse_outer)?))
//     }
// }

// impl ToTokens for OuterAttributes {
//     fn to_tokens(&self, tokens: &mut TokenStream) {
//         for i in self.0.iter() {
//             i.to_tokens(tokens);
//         }
//     }
// }

// impl<T: Parse> Parse for Scoop<T> {
//     fn parse(input: ParseStream) -> syn::Result<Self> {
//         let it = input.parse()?;
//         let rest = input.cursor().token_stream().into_iter().collect();
//         Ok(Scoop { it, rest })
//     }
// }

pub struct Matches<'a> {
    pub vars:   Vec<(Cow<'a, MetaVarPattern>, Cow<'a, [TokenTree]>)>,
    pub groups: Vec<(Cow<'a, MetaGroup<Pattern>>, Vec<Matches<'a>>)>,
}

// impl<'a> Matches<'a> {
//     pub fn var(&'a self, name: &str) -> Option<(&'a MetaVarPattern, &'a [TokenTree])> {
//         for (p, v) in self.vars.iter() {
//             if p.as_ref().name.string.as_str() == name {
//                 return Some((p.as_ref(), v.as_ref()));
//             }
//         }
//         None
//     }

//     pub fn group(&'a self, name: &str) -> Option<(&'a MetaGroup<Pattern>, &'a [Matches])> {
//         for (p, v) in self.groups.iter() {
//             if p.as_ref().name.string.as_str() == name {
//                 return Some((p.as_ref(), v.as_ref()));
//             }
//         }
//         None
//     }

//     fn rescope<'b: 'a>(self) -> Matches<'b> {
//         Matches {
//             vars: self.vars.into_iter()
//                 .map(|(pat, tree)| (Cow::Owned(pat.into_owned()), Cow::Owned(tree.into_owned())))
//                 .collect(),
//             groups: self.groups.into_iter()
//                 .map(|(pat, matches)| {
//                     ( Cow::Owned(pat.into_owned()),
//                       matches.into_iter().map(|m| m.rescope()).collect() )
//                 }).collect(),
//         }
//     }
// }

struct Matcher<'a, P> {
    patterns: P,
    stream:   ParseStream<'a>,
}

// impl<'a, P> Matcher<'a, P>
// where P: Iterator<Item=&'a Pattern> {
    
//     fn match_ident(
//         &'a mut self, pat: &'a Pattern, i: &'a Ident
//     ) -> Result<Match, MatchError<'a>> {
//         if let Some((ident, _)) = self.stream.cursor().ident() {
//             if ident.to_string().as_str() == i.string {
//                 Ok(Match::Ident(syn::Ident::parse(self.stream).unwrap()))
//             } else {
//                 todo!()
//             }
//         } else {
//             todo!()
//         }
//         // match self.tokens.split_first() {
//         //     Some((TokenTree::Ident(j),rest)) if j.to_string() == i.to_string() => {
//         //         self.tokens = rest;
//         //         Ok(Match::Ident(j.clone()))
//         //     }
//         //     other => self.nope(pat, other),
//         // }
//     }

//     fn match_punct(
//         &mut self, pat: &'a Pattern, p: &'a Punct
//     ) -> Result<(), MatchError<'a>> {
//         // match self.tokens.split_first() {
//         //     Some((TokenTree::Punct(q),rest)) if q.as_char() == p.as_char() => {
//         //         self.tokens = rest;
//         //         Ok(Match::Punct(q.clone()))
//         //     }
//         //     other => self.nope(pat, other),
//         // }
//         todo!()
//     }

// }

// impl<'a, P> Iterator for Matcher<'a, P>
// where P: Iterator<Item=&'a Pattern> {
    
//     fn next(
// }

// impl<'a> Matcher<'a> {
//     fn match_metagroup_round(
//         &mut self, g: &'a MetaGroup<Pattern>
//     ) -> Result<Vec<Match>, MatchError<'a>> {
//         let patterns = g.values.as_slice();
//         let mut matcher = Matcher { patterns, tokens: self.tokens };
//         let mut matches = vec!();
//         while let Some(m) = matcher.next() {
//             match m {
//                 Ok(m) => { matches.push(m); }
//                 Err(e) => { return Err(e.rescope()); }
//             }
//         }
//         self.tokens = matcher.tokens;
//         Ok(matches)
//     }

//     // TODO: Separators
//     fn match_metagroup(
//         &mut self, pat: &'a Pattern, g: &'a MetaGroup<Pattern>
//     ) -> Result<Match, MatchError<'a>> {
//         match g.multiplier {
//             Multiplier::One(_) => {
//                 let vals = self.match_metagroup_round(g)?;
//                 Ok(Match::MetaGroup(MetaGroupMatch::One(vals)))
//             }
//             Multiplier::ZeroOne(_) => {
//                 match self.match_metagroup_round(g) {
//                     Ok(vals) => {
//                         Ok(Match::MetaGroup(MetaGroupMatch::ZeroOne(Some(vals))))
//                     }
//                     Err(_) => {
//                         Ok(Match::MetaGroup(MetaGroupMatch::ZeroOne(None)))
//                     }
//                 }
//             }
//             Multiplier::ZeroMany(_) => {
//                 let mut results = vec!();
//                 while let Ok(vals) = self.match_metagroup_round(g) {
//                     results.push(vals);
//                 }
//                 Ok(Match::MetaGroup(MetaGroupMatch::ZeroMany(results)))
//             }
//             Multiplier::OneMany(_) => {
//                 let mut results = vec!();
//                 while let Ok(vals) = self.match_metagroup_round(g) {
//                     results.push(vals);
//                 }
//                 if results.is_empty() {
//                     Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
//                 } else {
//                     Ok(Match::MetaGroup(MetaGroupMatch::ZeroMany(results)))
//                 }
//             }
//         }
//     }

//     fn match_metavar_token<T: Parse + syn::token::Token>(
//         &mut self, pat: &'a Pattern, var: &'a MetaVarPattern
//      ) -> Result<Match, MatchError<'a>> {
//         if let Ok(_) = syn::parse2::<T>(TokenStream::from(self.tokens[0].clone())) {
//             let ident = var.name.clone();
//             let tokens = vec!(self.tokens[0].clone());
//             self.tokens = &self.tokens[1..];
//             Ok(Match::Binding(Binding { ident, tokens }))
//         } else {
//             Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
//         }
//     }

//     fn match_metavar_scoop<T: Parse + ToTokens>(
//         &mut self, pat: &'a Pattern, var: &MetaVarPattern
//     ) -> Result<Match, MatchError<'a>> {
//         let tokens = self.tokens.to_owned().into_iter().collect();
//         if let Ok(path) = syn::parse2::<Scoop<T>>(tokens) {
//             let drop = self.tokens.len() - path.rest.len();
//             let mut stream = TokenStream::new();
//             path.it.to_tokens(&mut stream);
//             let ident = var.name.clone();
//             self.tokens = &self.tokens[drop..];
//             Ok(Match::Binding(Binding { ident, tokens: stream.into_iter().collect() }))
//         } else {
//             Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
//         }
//     }

//     fn match_metavar(
//         &mut self, pat: &'a Pattern, var: &'a MetaVarPattern
//     ) -> Result<Match, MatchError<'a>> {
//         if let Some((tt, rest)) = self.tokens.split_first() {
//             match var.spec {
//                 FragSpec::Block(_)           => self.match_metavar_scoop::<syn::Block>(pat, var),
//                 FragSpec::Expr (_)           => self.match_metavar_scoop::<syn::Expr>(pat, var),
//                 FragSpec::Item (_)           => self.match_metavar_scoop::<syn::Item>(pat, var),
//                 FragSpec::Lifetime(_)        => self.match_metavar_scoop::<syn::Lifetime>(pat, var),
//                 FragSpec::Meta(_)            => self.match_metavar_scoop::<syn::Meta>(pat, var),
//                 FragSpec::Pat (_)            => self.match_metavar_scoop::<syn::Pat>(pat, var),
//                 FragSpec::Path(_)            => self.match_metavar_scoop::<syn::Path>(pat, var),
//                 FragSpec::Stmt(_)            => self.match_metavar_scoop::<syn::Stmt>(pat, var),
//                 FragSpec::Ty(_)              => self.match_metavar_scoop::<syn::Type>(pat, var),
//                 FragSpec::Vis(_)             => self.match_metavar_scoop::<syn::Visibility>(pat, var),
//                 FragSpec::Name(_)            => self.match_metavar_token::<Ident>(pat, var),
//                 FragSpec::OuterAttrs(_)      => self.match_metavar_scoop::<OuterAttributes>(pat, var),
//                 FragSpec::InnerAttrs(_)      => self.match_metavar_scoop::<InnerAttributes>(pat, var),
//                 FragSpec::GenericArgument(_) => self.match_metavar_scoop::<syn::GenericArgument>(pat, var),
//                 FragSpec::GenericParam(_)    => self.match_metavar_scoop::<syn::GenericParam>(pat, var),
//                 FragSpec::Tt  (_) => {
//                     let tokens = vec!(tt.clone());
//                     self.tokens = rest;
//                     Ok(Match::Binding(Binding { ident: var.name.clone(), tokens }))
//                 }
//                 FragSpec::Ident(_)    => {
//                     if let TokenTree::Ident(_) = tt {
//                         let tokens = vec!(tt.clone());
//                         self.tokens = rest;
//                         Ok(Match::Binding(Binding { ident: var.name.clone(), tokens }))
//                     } else {
//                         Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
//                     }
//                 }
//                 FragSpec::Literal(_)  => {
//                     if let TokenTree::Literal(_) = tt {
//                         let tokens = vec!(tt.clone());
//                         self.tokens = rest;
//                         Ok(Match::Binding(Binding { ident: var.name.clone(), tokens }))
//                     } else {
//                         Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
//                     }
//                 }
//             }
//         } else {
//             Err(MatchError::Empty(Cow::Borrowed(pat)))
//         }
//     }
            
//     fn match_delim(&self, delim: &Delimiter, mdelim: &MacroDelimiter) -> bool {
//         match (delim, mdelim) {
//             (Delimiter::Parenthesis, MacroDelimiter::Paren(_))   => true,
//             (Delimiter::Brace,       MacroDelimiter::Brace(_))   => true,
//             (Delimiter::Bracket,     MacroDelimiter::Bracket(_)) => true,
//             _ => false,
//         }
//     }

//     fn match_group(&mut self, pat: &'a Pattern, g: &'a Group<Pattern>) -> Result<Match, MatchError<'a>> {
//         if let Some((tt, rest)) = self.tokens.split_first() {
//             match tt {
//                 TokenTree::Group(h) if self.match_delim(&h.delimiter(), &g.delim) => {
//                     let tokens: Vec<TokenTree> = h.stream().into_iter().collect();
//                     let matcher = Matcher {
//                         patterns: g.values.as_slice(),
//                         tokens: tokens.as_slice(),
//                     };
//                     let mut matches = vec!();
//                     for m in matcher {
//                         m.map(|m| matches.push(m)).map_err(|e| e.rescope())?;
//                     }
//                     self.tokens = rest;
//                     Ok(Match::Group(GroupMatch {
//                         matches,
//                         delim: g.delim.clone(),
//                         group: h.clone(),
//                     }))
//                 }
//                 _ => Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens))),
//             }
//         } else {
//             Err(MatchError::Empty(Cow::Borrowed(pat)))
//         }
//     }

//     fn match_ident(&mut self, pat: &'a Pattern, i: &'a Ident) -> Result<Match, MatchError<'a>> {
//         match self.tokens.split_first() {
//             Some((TokenTree::Ident(j),rest)) if j.to_string() == i.to_string() => {
//                 self.tokens = rest;
//                 Ok(Match::Ident(j.clone()))
//             }
//             other => self.nope(pat, other),
//         }
//     }

//     fn match_punct(&mut self, pat: &'a Pattern, p: &'a Punct) -> Result<Match, MatchError<'a>> {
//         match self.tokens.split_first() {
//             Some((TokenTree::Punct(q),rest)) if q.as_char() == p.as_char() => {
//                 self.tokens = rest;
//                 Ok(Match::Punct(q.clone()))
//             }
//             other => self.nope(pat, other),
//         }
//     }

//     fn match_end(&mut self) -> Option<Result<Match, MatchError<'a>>> {
//         if self.tokens.is_empty() {
//             None
//         } else {
//             Some(Err(MatchError::Leftovers(self.tokens.into())))
//         }
//     }

//     fn nope(
//         &self, pat: &'a Pattern, tt: Option<(&'a TokenTree, &'a [TokenTree])>
//     ) -> Result<Match, MatchError<'a>> {
//         if tt.is_some() {
//             Err(MatchError::NoMatch(Cow::Borrowed(pat), Cow::Borrowed(&self.tokens)))
//         } else {
//             Err(MatchError::Empty(Cow::Borrowed(pat)))
//         }
//     }
// }


// impl Program {
//     pub fn from_rules(rules: Vec<Rule>) {
//         let mut program = Program::default();
//         // let mut warnings = Vec::new();
//         for rule in rules {
//             let name = rule.name.to_string();
//             if let Some(m) = program.macarons.get_mut(&name) {
//                 m.rules.push(rule);
//             } else {
//                 program.macarons.insert(name, rule.into());
//             }
//         }
//     }
// }
