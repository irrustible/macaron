//! Prelude:
//!
//! Identifiers:
//!
//! * ident!($($names)*) - concatenate items to make an ident
//! * snake!($name) - snake_case
//! * camel!($name) - camelCase
//! * pascal!($name) - PascalCase
//! * shout!($name) - SHOUTY_CASE

#[macro_export]
macro_rules! macaron {
    ($( $tt:tt )*) => { macaron_macros::bake!(@define module_path!() [$( $tt )*]); }
}

