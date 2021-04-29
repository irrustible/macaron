use proc_macro2::TokenStream;
use syn::parse::{Error, Parse, ParseBuffer, ParseStream};
use syn::{bracketed, parenthesized, token, Path, Token};
use crate::*;

/// Program definition, compiles to macro_rules
struct Define<'a> {
    pub at: Token![@],
    pub define: tokens::define,
    pub bracket: token::Bracket,
    pub buffer: ParseBuffer<'a>,
}

struct Call<'a> {
    pub at: Token![@],
    pub call: tokens::call,
    pub program_bracket: token::Bracket,
    pub program_buffer: ParseBuffer<'a>,
    pub call_bracket: token::Bracket,
    pub macaron: Ident,
    pub args_paren: token::Paren,
    pub args_buffer: ParseBuffer<'a>,
}

struct Use<'a> {
    pub call: Call<'a>,
    pub at: Token![@],
    pub use_token: Token![use],
    pub crate_name: Path,
    pub defs_bracket: token::Bracket,
    pub defs_buffer: ParseBuffer<'a>,
}

pub struct Baked(pub TokenStream);

impl Parse for Baked {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // A valid stream always starts @
        let at = input.parse::<Token![@]>()?;
        let l = input.lookahead1();
        if l.peek(tokens::define) {
            // If it's @define, it's macaron definitions
            Baked::parse_define(at, input)?.bake(input)
        } else if l.peek(tokens::call) {
            // If it's @call, it's a call to a generated macro
            let call = Baked::parse_call(at, input)?;
            // If the args begin @use, it's a request to import
            if call.args_buffer.peek(Token![@]) {
                Baked::parse_use(call)?.bake(input)
            } else {
                // It's just a call
                call.bake(input)
            }
        } else {
            Err(l.error())
        }
    }
}
impl Baked {
    fn parse_define(at: Token![@], input: ParseStream) -> syn::Result<Define> {
        let define = input.parse::<tokens::define>()?;
        let buffer;
        let bracket = bracketed!(buffer in input);
        if input.is_empty() {
            Ok(Define { at, define, bracket, buffer })
        } else {
            Err(Error::new(input.span(), "Expected end of arguments"))
        }
    }

    fn parse_call(at: Token![@], input: ParseStream) -> syn::Result<Call> {
        let call = input.parse::<tokens::call>()?;
        let program_buffer;
        let program_bracket = bracketed!(program_buffer in input);
        let call_buffer;
        let call_bracket = bracketed!(call_buffer in input);
        let macaron = call_buffer.parse::<Ident>()?;
        let args_buffer;
        let args_paren = parenthesized!(args_buffer in call_buffer);
        if input.is_empty() {
            Ok(Call {
                at, call, program_bracket, program_buffer, call_bracket,
                macaron, args_paren, args_buffer,
            })
        } else {
            Err(Error::new(input.span(), "Expected end of arguments"))
        }
    }

    fn parse_use(call: Call) -> syn::Result<Use> {
        let at = call.args_buffer.parse::<Token![@]>()?;
        let use_token = call.args_buffer.parse::<Token![use]>()?;
        let crate_name = call.args_buffer.parse::<Path>()?;
        let defs_buffer;
        let defs_bracket = bracketed!(defs_buffer in call.args_buffer);
        if call.args_buffer.is_empty() {
            Ok(Use {
                call, at, use_token, crate_name, defs_bracket, defs_buffer
            })
        } else {
            Err(Error::new(call.args_buffer.span(), "Expected end of arguments"))
        }
    }
}


impl<'a> Define<'a> {
    fn bake(self, input: ParseStream) -> syn::Result<Baked> {
        let mut program = Program::default();
        for d in Definitions::from(input) {
            match d? {
                Definition::Rule(rule) => program.append_rule(rule),
            }
        }
        let mut output = TokenStream::new();
        for m in program.macarons() {
            if m.is_public() {
                m.codegen(&mut output, &program);
            }
        }
        Ok(Baked(output))
    }
}

impl<'a> Call<'a> {
    fn bake(self, input: ParseStream) -> syn::Result<Baked> {
        todo!()
    }
}

impl<'a> Use<'a> {
    fn bake(self, input: ParseStream) -> syn::Result<Baked> {
        todo!()
    }
}

// We are presented with arbitrary input and have to figure it out
pub fn bake(stream: TokenStream) -> TokenStream {
    match syn::parse2(stream) {
        Ok(Baked(stream)) => stream,
        Err(_e)  => todo!(),
    }        
}
