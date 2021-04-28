use crate::{*, parsing::*};
use syn::{Result, Token};
use syn::parse::{Parse, ParseStream};

pub struct Definitions<'a> {
    stream: ParseStream<'a>,
}

impl<'a> From<ParseStream<'a>> for Definitions<'a> {
    fn from(stream: ParseStream<'a>) -> Self {
        Definitions { stream }
    }
}

impl<'a> Iterator for Definitions<'a> {
    type Item = Result<Definition>;
    fn next(&mut self) -> Option<Result<Definition>> {
        if self.stream.is_empty() {
            None
        } else {
            Some(self.stream.parse())
        }
    }
}

#[derive(Clone)]
pub enum Definition {
    // Include(Include),
    // Use(Use),
    Rule(Rule),
}

impl Parse for Definition {
    fn parse(input: ParseStream) -> Result<Self> {
        let l = input.lookahead1();
        // if l.peek(Token![@]) {
        //     Ok(Definition::Include(Include {
        //         at: input.parse()?,
        //         include: input.parse()?,
        //     }))
        // } else if l.peek(Token![use]) {
        //     todo!()
        // } else
        if l.peek(Token![pub]) {
            let pub_token = Some(input.parse::<Token![pub]>()?);
            let macro_token = input.parse::<Token![macro]>()?;
            let name: Ident = input.parse::<syn::Ident>()?.into();
            let (paren, patterns) = paren_many(input)?;
            let (brace, body) = brace_many(input)?;
            Ok(Definition::Rule(Rule {
                macro_token,
                name,
                paren,
                patterns,
                brace,
                body,
                pub_token,
            }))
        } else if l.peek(Token![macro]) {
            let macro_token = input.parse::<Token![macro]>()?;
            let name = input.parse::<syn::Ident>()?.into();
            let (paren, patterns) = paren_many(input)?;
            let (brace, body) = brace_many(input)?;
            Ok(Definition::Rule(Rule {
                macro_token,
                name,
                paren,
                patterns,
                brace,
                body,
                pub_token: None,
            }))
        } else {
            Err(l.error())
        }
    }
}

// #[derive(Clone)]
// pub struct Include {
//     pub at: Token![@],
//     pub include: tokens::include,
// }
// #[derive(Clone)]
// pub struct Use {}

