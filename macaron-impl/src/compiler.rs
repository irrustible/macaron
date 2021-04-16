use crate::ast::*;
use crate::diag::*;
use proc_macro2::Span;
use std::collections::HashMap;

#[derive(Default)]
pub struct Program {
    macarons: HashMap<String, Macaron>,
}

impl Program {
    
}

#[derive(Clone)]
pub struct Macaron {
    public: bool,
    rules: Vec<Rule>,
}

impl Macaron {
    pub fn check(&self) -> Result<(), Vec<Diagnostic>> {
        let mut diag = Vec::new();
        let mut rules = self.rules.iter();
        if let Some(first) = rules.next() {
            let mut pub_spans = vec!();
            if let Some(pub_token) = first.pub_token {
                for rule in rules {
                    if !rule.pub_token.is_some() {
                        pub_spans.push(rule.macro_token.span);
                    }
                }
                if !pub_spans.is_empty() {
                    diag.push(Diagnostic::spanned(
                        pub_spans, Level::Error, "Missing pub"
                    ).span_note(
                        pub_token.span, "Original (pub) macro definition here"
                    ));
                }
            } else {
                for rule in rules {
                    if let Some(pub_token) = rule.pub_token {
                        pub_spans.push(pub_token.span);
                    }
                }
                if !pub_spans.is_empty() {
                    diag.push(Diagnostic::spanned(
                        pub_spans, Level::Error, "Unexpected pub"
                    ).span_note(
                        first.macro_token.span, "Original (non-pub) macro definition here"
                    ));
                }
            }
            if diag.is_empty() {
                Ok(())
            } else {
                Err(diag)
            }
        } else {
            Ok(())
        }
    }
}

impl From<Rule> for Macaron {
    fn from(rule: Rule) -> Macaron {
        Macaron {
            public: rule.pub_token.is_some(),
            rules: vec!(rule),
        }
    }
}

impl Program {
    pub fn from_rules(rules: Vec<Rule>) {
        let mut program = Program::default();
        // let mut warnings = Vec::new();
        for rule in rules {
            let name = rule.name.to_string();
            if let Some(m) = program.macarons.get_mut(&name) {
                m.rules.push(rule);
            } else {
                program.macarons.insert(name, rule.into());
            }
        }
    }
}
