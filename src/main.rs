mod atom;

use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
enum Fact {
    Tuple(Vec<Fact>),
    Literal(String),
    Variable(String),
}

struct Rule<'a> {
    consequent: &'a Fact,
    antecedents: Vec<&'a Fact>,
}

#[derive(Default)]
struct VarAssignments<'a> {
    pairs: Vec<(&'a String, &'a Fact)>,
}
impl<'a> VarAssignments<'a> {
    fn save(&self) -> usize {
        self.pairs.len()
    }
    fn load(&mut self, index: usize) {
        self.pairs.truncate(index)
    }
    fn get(&'a self, var1: &'a String) -> Option<&'a Fact> {
        self.pairs
            .iter()
            .find(|(var2, _)| &var1 == var2)
            .map(|(_, fact)| fact)
            .copied()
    }
    fn push(&'a mut self, var: &'a String, fact: &'a Fact) {
        self.pairs.push((var, fact))
    }
}

struct Variables<'a>(&'a [String]);
impl Fact {
    fn try_unify(&self, other: &Self, v2f: &mut VarAssignments) -> bool {
        todo!()
    }
}
impl Rule<'_> {
    fn infer_new(&self, facts: &HashSet<Fact>) -> Option<Fact> {
        let mut v2f = VarAssignments::default();
        self.infer_rec(facts, &mut v2f, self.antecedents.as_slice())
    }
    fn infer_rec(
        &self,
        facts: &HashSet<Fact>,
        v2f: &mut VarAssignments,
        tail: &[&Fact],
    ) -> Option<Fact> {
        match tail {
            [] => {
                todo!() // must concretize using variables!
            }
            [a, new_tail @ ..] => {
                for fact in facts {
                    let token = v2f.save();
                    if a.try_unify(fact, v2f) {
                        if let Some(new) = self.infer_rec(facts, v2f, new_tail) {
                            return Some(new);
                        }
                    }
                    v2f.load(token);
                }
            }
        }
        None
    }
}

fn main() {
    let mut facts = HashSet::<Fact>::default();

    'saturate: loop {
        for fact in &facts {
            if let Some(rule) = fact.as_rule() {
                if let Some(fact) = rule.infer_new(&facts) {
                    facts.insert(fact);
                    continue 'saturate;
                }
            }
        }
    }
}
