use crate::parse::rule;
use std::collections::HashMap;

mod parse;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Atom {
    Tuple(Vec<Atom>),
    Id(String),
    Variable(u16),
}

#[derive(Debug)]
pub struct Rule {
    consequent: Atom,
    antecedents: Vec<Atom>,
}

struct ProofTree<'a> {
    consequent: Atom,
    rule: &'a Rule,
    antecedents: Vec<ProofTree<'a>>,
    substitutions: HashMap<&'a String, Atom>,
}

impl ProofTree<'_> {
    fn check(&self) -> bool {
        todo!()
    }
    fn create(rules: &[Rule], atom: &Atom) -> Option<Self> {
        todo!()
    }
}

fn main() {
    let rule = rule(" A  :- (x) (x y (z ((q(h))))).");
    println!("{:?}", rule);
}
