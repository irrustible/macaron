use crate::*;
use std::collections::HashMap;
use syn::parse::ParseStream;

pub struct Module {
    macros: HashMap<String, Rule>,
}
    

pub struct Macro {
    rules: Vec<Rule>,
}

impl Macro {
    pub fn is_public(&self) -> bool {
        if let Some(rule) = self.rules.first() {
            rule.pub_token.is_some()
        } else { false }
    }
    pub fn match_trans(&self, stream: ParseStream) -> syn::Result<()> {
        // for rule in self.rules.iter() {
        //     let strm = stream.fork();
        //     // rule.
        // }
        todo!()
    }
}
