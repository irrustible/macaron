use syn::{parse::{Parse, ParseStream, Error}, Result};

#[derive(Clone)]
pub struct Literal {
    pub lit: proc_macro2::Literal,
    pub string: String,
}

impl Literal {
    pub fn as_str(&self) -> &str {
        self.string.as_str()
    }
    pub fn parse_match(&self, stream: ParseStream) -> Result<Literal> {
        stream.step(|cursor| {
            if let Some((l,m)) = cursor.literal() {
                let l = Literal::from(l);
                if l.as_str() == self.as_str() {
                    Ok((l, m))
                } else {
                    Err(Error::new(stream.span(), "Expected matching ident"))
                }
            } else {
                Err(Error::new(stream.span(), "Expected ident"))
            }
        })
    }
}

impl Parse for Literal {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let lit = proc_macro2::Literal::parse(input)?;
        let string = lit.to_string();
        Ok(Literal { lit, string })
    }
}

impl From<proc_macro2::Literal> for Literal {
    fn from(lit: proc_macro2::Literal) -> Self {
        let string = lit.to_string();
        Literal { lit, string }
    }
}
