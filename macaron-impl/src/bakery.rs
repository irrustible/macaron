use proc_macro::{TokenStream};
use syn::parse::{Parse, ParseBuffer, ParseStream};
use syn::{bracketed, token, Path, Token};
use std::collections::HashMap;
use crate::syntax::{*, defs::*};

struct Define<'a> {
    pub at: Token![@],
    pub define: tokens::define,
    pub crate_name: Path,
    pub bracket: token::Bracket,
    pub stream: ParseBuffer<'a>,
}

struct Call<'a> {
    pub at: Token![@],
    pub call: tokens::call,
    pub crate_name: Path,
    pub program_bracket: token::Bracket,
    pub program_stream: ParseBuffer<'a>,
    pub call_bracket: token::Bracket,
    pub call_stream: ParseBuffer<'a>,
    
}
// struct Import<'a> {
//     pub at: Token![@],
//     pub expand: tokens::expand,
//     pub crate_name: Path,
//     pub program_bracket: token::Bracket,
//     pub program_stream: ParseStream<'a>,
//     pub call_bracket: token::Bracket,
//     pub at2: Token![@],
//     pub import: tokens::import,
// }

pub struct Baked(pub TokenStream);

impl Parse for Baked {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let at = input.parse::<Token![@]>()?;
        let l = input.lookahead1();
        if l.peek(tokens::define) {
            let define = input.parse::<tokens::define>()?;
            let crate_name = input.parse::<Path>()?;
            let stream;
            let bracket = bracketed!(stream in input);
            Define { at, define, crate_name, bracket, stream }.bake(input)
        } else if l.peek(tokens::call) {
            let call = input.parse::<tokens::call>()?;
            let crate_name = input.parse::<Path>()?;
            let program_stream;
            let program_bracket = bracketed!(program_stream in input);
            let call_stream;
            let call_bracket = bracketed!(call_stream in input);
            Call {
                at, call, crate_name, program_bracket,
                program_stream, call_bracket, call_stream,
            }.bake(input)
        } else {
            Err(l.error())
        }
    }
}

impl<'a> Define<'a> {
    fn bake(self, input: ParseStream) -> syn::Result<Baked> {
        let defs = Definitions::from(input);
        let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();
        // let mut uses = vec!();
        // let mut includes = vec!();
        for d in defs {
            match d? {
                // Definition::Use(u) => uses.push(u),
                // Definition::Include(i) => includes.push(i),
                Definition::Rule(r) => {
                    if let Some(rs) = rules.get_mut(r.as_str()) {
                        rs.push(r);
                    } else {
                        rules.insert(r.as_str().to_string(), vec!(r));
                    }
                }
            }
        }
        todo!()
    }
}

impl<'a> Call<'a> {
    fn bake(self, input: ParseStream) -> syn::Result<Baked> {
        todo!()
    }
}

// We are presented with arbitrary input and have to figure it out
pub fn bake(stream: TokenStream) -> TokenStream {
    todo!()
}
