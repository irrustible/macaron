//! Syntax:
//!
//! 
//!
//!
//!
//!
//!
//!
//!
//!
//!
//!
//! Prelude:
//!
//! Identifiers:
//! 
//! * ident!($($names)*) - concatenate items to make an ident
//! * snake!($name) - snake_case
//! * camel!($name) - camelCase
//! * pascal!($name) - PascalCase
//! * shout!($name) - SHOUTY_CASE
//!
//! Notes
//! 
//!
//!
//!
//!
//!
//!
//!
use proc_macro2 as pm2;
use pm2::{TokenStream};
use syn::buffer::TokenBuffer;

pub mod ast;
pub mod compiler;
pub mod diag;
pub use diag::*;
mod parsing;

pub fn macaron_define(stream: TokenStream) -> Result<TokenStream, Vec<Diagnostic>> {
    let buffer = TokenBuffer::new2(stream);
    Ok(TokenStream::new())
}

pub fn macaron_expand(stream: TokenStream) -> Result<TokenStream, Vec<Diagnostic>> {
    let buffer = TokenBuffer::new2(stream);
    Ok(TokenStream::new())
}
