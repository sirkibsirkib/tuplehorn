use std::collections::HashMap;

mod parse;

#[derive( Debug, Eq, PartialEq, Hash)]
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

#[derive(Debug)]
struct ProofTree<'a> {
    rule: &'a Rule,
    sub: Substitutions<'a>,
    lemmata: Vec<ProofTree<'a>>,
}

#[derive(Default,Debug)]
struct Substitutions<'a> {
    map: HashMap<u16,&'a Atom>,
}
impl<'a> Substitutions<'a> {
    fn sub(&mut self, k: u16, v: &'a Atom) -> bool {
        match self.map.get(&k).copied() {
            Some(v2) if v2 != v => false,
            _ => {
                self.map.insert(k, v);
                true
            }
        }
    }
    fn reset(&mut self) {
        self.map.clear()
    }
}

impl Atom {
    fn sub_vars<'a:'c,'b,'c>(&'a self, to_match: &'a Self, sub: &'b mut Substitutions<'c>) -> bool {
        match [self, to_match] {
            [Self::Id(a), Self::Id(b)] => a == b, 
            [Self::Tuple(a), Self::Tuple(b)] if a.len() == b.len() => {
                for (a,b) in a.iter().zip(b.iter()) {
                    if !a.sub_vars(b,sub) {
                        return false
                    }
                }
                true
            }
            [Self::Variable(a), Self::Variable(b)] if a == b => {
                true
            }
            [Self::Variable(a), b] => {
                sub.sub(*a, b)
            }
            _ => false
        }
    }
}

impl<'a> ProofTree<'a> {
    fn check(&self) -> bool {
        todo!()
    }
    fn create(rules: &'a [Rule], goal: &'a Atom) -> Option<Self> {
        let mut sub = Substitutions::default();
        for rule in rules {
            if rule.consequent.sub_vars(goal, &mut sub) {
                let maybe_lemmata = rule.antecedents.iter().map(|subgoal| {
                    ProofTree::create(rules, subgoal)
                }).collect();
                if let Some(lemmata) = maybe_lemmata {

                    return Some(Self {
                        rule, lemmata, sub
                    })
                }
            }
            sub.reset();
        }
        None
    }
}

fn main() {
    let rules = parse::wsr(parse::rules)(
    "
    (b 2).
    (a 0) :- (b 0).
    ",
    ).unwrap().1;
    println!("{:#?}", rules);
    let goal = parse::wsr(parse::atom)("
    (a lel)
    ").unwrap().1;
    println!("{:#?}", goal);
    let proof = ProofTree::create(&rules, &goal);
    println!("{:#?}", proof);
}
