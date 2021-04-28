use proc_macro2::{TokenTree, TokenStream};
use crate::*;
use crate::syntax::*;
use syn::{parse::{Parse, ParseStream, Result}, Token};
use quote::ToTokens;
use crate::matches::*;

#[derive(Clone)]
pub enum FragSpec {
    /// [builtin]
    Block   (tokens::block),
    /// [builtin]
    Expr    (tokens::expr),
    /// [builtin] An identifier or keyword
    Ident   (tokens::ident),
    /// [builtin]
    Item    (tokens::item),
    /// [builtin]
    Lifetime(tokens::lifetime),
    /// [builtin]
    Literal (tokens::literal),
    /// [builtin]
    Meta    (tokens::meta),
    /// [builtin]
    Pat     (tokens::pat),
    /// [builtin]
    Path    (tokens::path),
    /// [builtin]
    Stmt    (tokens::stmt),
    /// [builtin]
    Tt      (tokens::tt),
    /// [builtin]
    Ty      (tokens::ty),
    /// [builtin]
    Vis     (tokens::vis),

    /// [custom] Generic Arguments
    GenericArgument(tokens::genarg),
    /// [custom] Generic Parameter
    GenericParam   (tokens::genparam),
    /// [custom] Bang attributes
    InnerAttrs     (tokens::inattrs),
    /// [custom] Actually an identifier, not a keyword
    Name           (tokens::name),
    /// [custom] Regular attribute
    OuterAttrs     (tokens::attrs),
}

impl Parse for FragSpec {
    fn parse(input: ParseStream) -> Result<Self> {
        use FragSpec::*;
        let l = input.lookahead1();
        if l.peek(tokens::block) {
            input.parse().map(Block)
        } else if l.peek(tokens::expr) {
            input.parse().map(Expr)
        } else if l.peek(tokens::ident) {
            input.parse().map(Ident)
        } else if l.peek(tokens::item) {
            input.parse().map(Item)
        } else if l.peek(tokens::lifetime) {
            input.parse().map(Lifetime)
        } else if l.peek(tokens::literal) {
            input.parse().map(Literal)
        } else if l.peek(tokens::meta) {
            input.parse().map(Meta)
        } else if l.peek(tokens::pat) {
            input.parse().map(Pat)
        } else if l.peek(tokens::path) {
            input.parse().map(Path)
        } else if l.peek(tokens::stmt) {
            input.parse().map(Stmt)
        } else if l.peek(tokens::tt) {
            input.parse().map(Tt)
        } else if l.peek(tokens::ty) {
            input.parse().map(Ty)
        } else if l.peek(tokens::vis) {
            input.parse().map(Vis)
        } else if l.peek(tokens::attrs) {
            input.parse().map(OuterAttrs)
        } else if l.peek(tokens::inattrs) {
            input.parse().map(InnerAttrs)
        } else if l.peek(tokens::name) {
            input.parse().map(Name)
        } else if l.peek(tokens::genarg) {
            input.parse().map(GenericArgument)
        } else if l.peek(tokens::genparam) {
            input.parse().map(GenericParam)
        } else {
            Err(l.error())
        }
    }
}

#[derive(Clone)]
pub enum Fragment {
    Block   (syn::Block),
    Expr    (syn::Expr),
    Ident   (Ident),
    Item    (syn::Item),
    Lifetime(syn::Lifetime),
    Literal (syn::Lit),
    Meta    (syn::Meta),
    Pat     (syn::Pat),
    Path    (syn::Path),
    Stmt    (syn::Stmt),
    Tt      (TokenTree),
    Ty      (syn::Type),
    Vis     (syn::Visibility),
    GenArg  (syn::GenericArgument),
    GenParam(syn::GenericParam),
    Attrs   (Attributes),
}

impl Fragment {
    pub fn match_spec(spec: &FragSpec, stream: ParseStream) -> Result<Fragment> {
        match spec {
            FragSpec::Block(_) =>
                stream.parse().map(Fragment::Block),
            FragSpec::Expr(_) =>
                stream.parse().map(Fragment::Expr),
            FragSpec::Ident(_) =>
                Ident::parse_any(stream).map(Fragment::Ident),
            FragSpec::Item(_) =>
                stream.parse().map(Fragment::Item),
            FragSpec::Lifetime(_) =>
                stream.parse().map(Fragment::Lifetime),
            FragSpec::Literal(_) =>
                stream.parse().map(Fragment::Literal),
            FragSpec::Meta(_) =>
                stream.parse().map(Fragment::Meta),
            FragSpec::Pat(_) =>
                stream.parse().map(Fragment::Pat),
            FragSpec::Path(_) =>
                stream.parse().map(Fragment::Path),
            FragSpec::Stmt(_) =>
                stream.parse().map(Fragment::Stmt),
            FragSpec::Tt(_) =>
                stream.parse().map(Fragment::Tt),
            FragSpec::Ty(_) =>
                stream.parse().map(Fragment::Ty),
            FragSpec::Vis(_) =>
                stream.parse().map(Fragment::Vis),
            FragSpec::GenericArgument(_) =>
                stream.parse().map(Fragment::GenArg),
            FragSpec::GenericParam(_) =>
                stream.parse().map(Fragment::GenParam),
            FragSpec::InnerAttrs(_) =>
                syn::Attribute::parse_inner(stream).map(|a| Fragment::Attrs(Attributes(a))),
            FragSpec::Name(_) =>
                stream.parse().map(Fragment::Ident),
            FragSpec::OuterAttrs(_) =>
                syn::Attribute::parse_outer(stream).map(|a| Fragment::Attrs(Attributes(a))),
        }
    }
}

impl ToTokens for Fragment {
    fn to_tokens(&self, tree: &mut TokenStream) {
        use Fragment::*;
        match self {
            Block   (f) => f.to_tokens(tree),
            Expr    (f) => f.to_tokens(tree),
            Ident   (f) => f.to_tokens(tree),
            Item    (f) => f.to_tokens(tree),
            Lifetime(f) => f.to_tokens(tree),
            Literal (f) => f.to_tokens(tree),
            Meta    (f) => f.to_tokens(tree),
            Pat     (f) => f.to_tokens(tree),
            Path    (f) => f.to_tokens(tree),
            Stmt    (f) => f.to_tokens(tree),
            Tt      (f) => f.to_tokens(tree),
            Ty      (f) => f.to_tokens(tree),
            Vis     (f) => f.to_tokens(tree),
            GenArg  (f) => f.to_tokens(tree),
            GenParam(f) => f.to_tokens(tree),
            Attrs   (f) => f.to_tokens(tree),
        }
    }
}

#[derive(Clone)]
/// Matching a metafrag returns the matched data
pub struct FragmentMatch {
    pub name: Option<Ident>,
    pub fragment: Fragment,
}

impl ToTokens for FragmentMatch {
    fn to_tokens(&self, tree: &mut TokenStream) {
        self.fragment.to_tokens(tree)
    }
}

#[derive(Clone)]
pub struct FragPat {
    pub dollar: Token![$],
    pub name: Option<Ident>,
    pub colon: Token![:],
    pub spec: FragSpec,
}

impl FragPat {
    pub fn parse_match(&self, stream: ParseStream) -> Result<FragmentMatch> {
        Ok(FragmentMatch {
            name: self.name.clone(),
            fragment: Fragment::match_spec(&self.spec, stream)?,
        })
   }
}
