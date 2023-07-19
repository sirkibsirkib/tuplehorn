use std::collections::HashMap;

mod parse;

/* crucial idea: my var(0) is not the same as your var(0).
*/

type Id = String;
type Var = u16;
#[derive(Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Atom {
    Tuple(Vec<Atom>),
    Id(Id),
    Var(Var),
}

pub struct Rule {
    consequent: Atom,
    antecedents: Vec<Atom>,
}

trait VisitVars {
    fn visit_vars(&self, visit: &mut impl FnMut(u16));
    fn visit_vars_mut(&mut self, visit: &mut impl FnMut(&mut u16));
    fn normalize_vars(&mut self) {
        let mut fresh_iter = (0 as Var)..;
        let mut rename = HashMap::<Var, Var>::default();
        self.visit_vars_mut(&mut |var: &mut Var| {
            *var = *rename.entry(*var).or_insert_with(|| fresh_iter.next().unwrap());
        })
    }
}
impl VisitVars for Rule {
    fn visit_vars(&self, visit: &mut impl FnMut(u16)) {
        for atom in Some(&self.consequent).into_iter().chain(self.antecedents.iter()) {
            atom.visit_vars(visit)
        }
    }
    fn visit_vars_mut(&mut self, visit: &mut impl FnMut(&mut u16)) {
        for atom in Some(&mut self.consequent).into_iter().chain(self.antecedents.iter_mut()) {
            atom.visit_vars_mut(visit)
        }
    }
}

impl VisitVars for Atom {
    fn visit_vars(&self, visit: &mut impl FnMut(u16)) {
        match self {
            Self::Id(_) => {}
            Self::Var(v) => visit(*v),
            Self::Tuple(args) => {
                for arg in args {
                    arg.visit_vars(visit)
                }
            }
        }
    }
    fn visit_vars_mut(&mut self, visit: &mut impl FnMut(&mut u16)) {
        match self {
            Self::Id(_) => {}
            Self::Var(v) => visit(v),
            Self::Tuple(args) => {
                for arg in args {
                    arg.visit_vars_mut(visit)
                }
            }
        }
    }
}

impl Atom {
    fn contains_var(&self, var: Var) -> bool {
        let mut saw = false;
        self.visit_vars(&mut |var2| {
            if var2 == var {
                saw = true;
            }
        });
        saw
    }
    fn unify(atoms: [&Self; 2]) -> Option<HashMap<Var, Atom>> {
        match atoms {
            [Atom::Id(x), Atom::Id(y)] if x == y => return Some(Default::default()),
            [Atom::Var(x), Atom::Var(y)] if x == y => return Some(Default::default()),
            [Atom::Var(var), atom] | [atom, Atom::Var(var)] => {
                if atom.contains_var(*var) {
                    return None;
                } else {
                    let mut ret = HashMap::default();
                    ret.insert(*var, atom.clone());
                    return Some(ret);
                }
            }
            [Atom::Tuple(x), Atom::Tuple(y)] => {
                if x.len() != y.len() {
                    None
                } else {
                    let mut ret = HashMap::default();
                    for (x, y) in x.iter().zip(y.iter()) {
                        let inner = Atom::unify([x, y])?;
                        for (var, atom) in inner {
                            if let Some(prev) = ret.insert(var, atom.clone()) {
                                if prev != atom {
                                    return None;
                                }
                            }
                        }
                    }
                    Some(ret)
                }
            }
            _ => None,
        }
    }
}

fn print_indent<T>(history: &[T]) {
    for _ in 0..history.len() {
        print!("  ");
    }
}

impl std::fmt::Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Var(v) => v.fmt(f),
            Self::Id(i) => write!(f, "{}", i),
            Self::Tuple(args) => {
                write!(f, "(")?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, " ")?;
                    }
                    write!(f, "{:?}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}
impl std::fmt::Debug for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.consequent)?;
        if !self.antecedents.is_empty() {
            write!(f, " :-")?;
            for antecedent in &self.antecedents {
                write!(f, " {:?}", antecedent)?;
            }
        }
        write!(f, ".")
    }
}

#[derive(Debug, Default)]
struct Kb {
    vec: Vec<Atom>,
    index: HashMap<Atom, usize>,
}
impl Kb {
    fn insert(&mut self, atom: Atom) -> bool {
        if self.index.get(&atom).is_some() {
            return false;
        }
        self.index.insert(atom.clone(), self.vec.len());
        self.vec.push(atom);
        true
    }
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
enum Ctx {
    Rule,
    Antecedent { index: usize },
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct EqNode<'a> {
    atom: &'a Atom,
    ctx: Ctx,
}

#[derive(Eq, PartialEq, Ord, PartialOrd)]
struct EqEdge<'a> {
    // in normal form: nodes[0] < nodes[1]
    nodes: [EqNode<'a>; 2],
}

#[derive(Default)]
struct EqGraph<'a> {
    // in normal form: edge[i] < edge[i+1]
    edges: Vec<EqEdge<'a>>,
}

impl Rule {
    fn normalize_antecedent_set(&mut self) {
        self.antecedents.sort();
        self.antecedents.dedup();
    }
}

fn main() {
    let rules = "
    8 :- (auth 3) (3 say 8).
    (auth amy).
    (amy say (bob cool)).
    win :- (bob cool).
    ";
    let mut rules = parse::wsr(parse::rules)(rules).unwrap().1;
    for rule in rules.iter_mut() {
        rule.normalize_vars();
        rule.normalize_antecedent_set();
    }
    println!("{:#?}", rules);
    {
        let fail = [
            &parse::wsr(parse::atom)("((x 5) 0 0 1 1 (5 y))").unwrap().1,
            &parse::wsr(parse::atom)("(7     7 8 8 9 9    )").unwrap().1,
        ];
        let success = [
            &parse::wsr(parse::atom)("((x 5) 0 0 1 1 (6 y))").unwrap().1,
            &parse::wsr(parse::atom)("(7     7 8 8 9 9    )").unwrap().1,
        ];
    }

    let mut kb = Kb::default();
    return;
    for _ in 0..1000 {
        for rule in &rules {
            let mut ci = combo_iter::BoxComboIter::new(&kb.vec, rule.antecedents.len());
            while let Some(fact_combo) = ci.next() {
                // let eq_graph = EqGraph::default();
                println!("{:?}", fact_combo);
            }
            // let's derive using this rule!
        }
    }
}
