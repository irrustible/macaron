use macaron::macaron;

macaron! {
    /// it's very good
    pub macro likes($x:var, $y:var, $z:var) => {
        likes($x, $y);
        likes($x, $z);
    }
}

macaron! {
    // prelude: snake!, camel!, pascal!, shout!
    self;
    $name:spec // variable 
    $()?; // 0 or one  ("hesitation")
    $()+; // 1 or more ("repetition")
    $()*; // 0 or more ("deviation")
    ${}$; // eval expression
    $[]; // ???
    const x: usize = 1;
    // pub macros will be exported
    pub macro unwrapper($enum:ident :: $cons:ident) => {
        #[inline(always)] pub fn $cons:snake(self) {
            if let $enum::cons = self {} else { panic!("not $cons"); }
        }
    }
    macro unwrapper($enum:ident :: $cons:ident => $inner:ty) => {
        #[inline(always)] pub fn ${snake!($cons)}(self) -> $inner {
            if let $enum::cons(val) = self { val } else { panic!("not $op"); }
        }
    }
}
