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
use proc_macro2::TokenStream;
use macaron_impl;
// #[macro_export]
// macro_rules! macaron {
//     ( $( $t:tt )* ) => { $crate::bake(@mix [$crate] [ $( $tt)* ]) }
// }

#[doc(hidden)]
#[proc_macro]
pub fn bake(stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let stream = TokenStream::from(stream);
    // bakery::bake(stream)
    TokenStream::new().into()
}

