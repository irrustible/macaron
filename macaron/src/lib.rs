extern crate proc_macro;
use proc_macro::TokenStream;
use proc_macro2;
use macaron_impl;

#[proc_macro]
pub fn macaron(stream: TokenStream) -> TokenStream {
    macaron_impl::macaron_define(proc_macro2::TokenStream::from(stream)).into()
}

#[doc(hidden)]
#[proc_macro]
pub fn macaron_expand(stream: TokenStream) -> TokenStream {
    macaron_impl::macaron_expand(proc_macro2::TokenStream::from(stream)).into()
}
