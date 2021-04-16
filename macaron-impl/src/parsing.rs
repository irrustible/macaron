use syn::*;
use syn::parse::{Parse, ParseStream};

pub fn paren_many<T: Parse>(input: ParseStream) -> Result<(token::Paren, Vec<T>)> {
    let content;
    let paren = parenthesized!(content in input);
    let mut values = Vec::new();
    while !input.is_empty() {
        values.push(content.parse()?);
    }
    Ok((paren, values))
}

pub fn bracket_many<T: Parse>(input: ParseStream) -> Result<(token::Bracket, Vec<T>)> {
    let content;
    let bracket = bracketed!(content in input);
    let mut values = Vec::new();
    while !input.is_empty() {
        values.push(content.parse()?);
    }
    Ok((bracket, values))
}

pub fn bracket_one<T: Parse>(
    input: ParseStream,
    parser: fn(_: ParseStream<'_>) -> Result<T>
) -> Result<(token::Bracket, T)> {
    let content;
    let bracket = bracketed!(content in input);
    let val = content.call(parser)?;
    if input.is_empty() {
        Ok((bracket, val))
    } else {
        Err(input.error("expected closing bracket!"))
    }
}

pub fn brace_many<T: Parse>(input: ParseStream) -> Result<(token::Brace, Vec<T>)> {
    let content;
    let brace = braced!(content in input);
    let mut values = Vec::new();
    while !input.is_empty() {
        values.push(content.parse()?);
    }
    Ok((brace, values))
}
