//! Prelude:
//!
//! Identifiers:
//!
//! * ident!($($names)*) - concatenate items to make an ident
//! * snake!($name) - snake_case
//! * camel!($name) - camelCase
//! * pascal!($name) - PascalCase
//! * shout!($name) - SHOUTY_CASE
extern crate proc_macro;

#[doc(hidden)]
#[proc_macro]
pub fn bake(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stream = proc_macro2::TokenStream::from(stream);
    macaron_impl::bakery::bake(stream).into()
}
