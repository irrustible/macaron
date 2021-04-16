use crate::ast::*;
use crate::diag::*;
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
        for rule in self.rules.iter() {
            self.validate_rule(rule, &mut diag);
        }
        Ok(())
    }
    pub fn validate_rule(&self, rule: &Rule, diag: &mut Vec<Diagnostic>) {
    
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
