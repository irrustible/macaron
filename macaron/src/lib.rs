
#[macro_export]
macro_rules! macaron {
    ( $( $tt:tt )* ) => { ::macaron_macros::bake! { @declare [$crate] [ $( $tt )* ] } }
}
