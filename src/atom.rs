use crate::{Fact, Rule};
use std::collections::HashSet;

impl Fact {
    pub fn as_rule(&self) -> Option<Rule> {
        if let Fact::Tuple(args) = self {
            if let [consequent, Fact::Literal(i), antecedents @ ..] = args.as_slice() {
                if i == "if" {
                    return Some(Rule {
                        consequent,
                        antecedents: antecedents.iter().collect(),
                    });
                }
            }
        }
        None
    }
    fn visit_facts<'a>(&'a self, visit: &mut impl FnMut(&'a Fact)) {
        visit(self);
        if let Fact::Tuple(args) = self {
            for arg in args {
                arg.visit_facts(visit)
            }
        }
    }
}
impl Rule<'_> {
    fn visit_facts<'a>(&'a self, visit: &mut impl FnMut(&'a Fact)) {
        self.consequent.visit_facts(visit);
        for antecedent in &self.antecedents {
            antecedent.visit_facts(visit)
        }
    }
    pub fn variables(&self) -> HashSet<&String> {
        let mut vars = HashSet::default();
        // for x in self.con
        self.visit_facts(&mut |fact| {
            if let Fact::Variable(var) = fact {
                vars.insert(var);
            }
        });
        vars
    }
}
