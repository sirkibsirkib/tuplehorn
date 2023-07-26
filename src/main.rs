use eq_graph::EqGraph;
use std::collections::HashMap;

mod parse;

/* crucial idea: my var(0) is not the same as your var(0).
*/

type Id = String;
type Var = u32;
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
    fn visit_vars(&self, visit: &mut impl FnMut(Var));
    fn visit_vars_mut(&mut self, visit: &mut impl FnMut(&mut Var));
}
impl VisitVars for Rule {
    fn visit_vars(&self, visit: &mut impl FnMut(Var)) {
        for atom in Some(&self.consequent).into_iter().chain(self.antecedents.iter()) {
            atom.visit_vars(visit)
        }
    }
    fn visit_vars_mut(&mut self, visit: &mut impl FnMut(&mut Var)) {
        for atom in Some(&mut self.consequent).into_iter().chain(self.antecedents.iter_mut()) {
            atom.visit_vars_mut(visit)
        }
    }
}

impl VisitVars for Atom {
    fn visit_vars(&self, visit: &mut impl FnMut(Var)) {
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
    fn visit_vars_mut(&mut self, visit: &mut impl FnMut(&mut Var)) {
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

impl Rule {
    fn normalize_antecedent_set(&mut self) {
        self.antecedents.sort();
        self.antecedents.dedup();
    }
}

#[derive(Debug)]
struct EqClash<'a>([&'a Atom; 2]);

fn equate<'a, 'b>(
    eg: &'a mut EqGraph<&'b Atom>,
    a: &'b Atom,
    b: &'b Atom,
) -> Result<(), EqClash<'b>> {
    if !eg.relate(a, b) {
        // no change
        return Ok(());
    }
    let class = eg.equivalents(&a).copied().collect::<Vec<&Atom>>();
    for &x in &class {
        for &y in &class {
            let clash: bool = match [x, y] {
                [Atom::Id(xid), Atom::Id(yid)] if xid != yid => true,
                [Atom::Id(_), Atom::Tuple(_)] | [Atom::Tuple(_), Atom::Id(_)] => true,
                [Atom::Tuple(xargs), Atom::Tuple(yargs)] => {
                    if xargs.len() != yargs.len() {
                        true
                    } else {
                        for (xarg, yarg) in xargs.iter().zip(yargs.iter()) {
                            equate(eg, xarg, yarg)?;
                        }
                        false
                    }
                }
                _ => false,
            };
            if clash {
                return Err(EqClash([x, y]));
            }
        }
    }
    Ok(())
}
fn unify(atom: &Atom, eq_graph: &EqGraph<&Atom>) -> Result<(), ()> {
    todo!()
}

#[derive(Default)]
struct VarAllocator {
    next: Var,
    buf: HashMap<Var, Var>,
}
impl VarAllocator {
    fn refresh<T: VisitVars>(&mut self, t: &mut T) {
        self.buf.clear();
        t.visit_vars_mut(&mut |v: &mut Var| {
            *v = *self.buf.entry(*v).or_insert_with(|| {
                self.next += 1;
                self.next - 1
            });
        });
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
    let mut var_allocator = VarAllocator::default();
    for rule in rules.iter_mut() {
        var_allocator.refresh(rule);
        rule.normalize_antecedent_set();
    }
    println!("{:#?}", rules);

    let kb = Kb::default();

    for _ in 0..1 {
        for rule in &rules {
            println!("rule {:?}", rule);
            let mut ci = combo_iter::BoxComboIter::new(&kb.vec, rule.antecedents.len());
            while let Some(fact_combo) = ci.next() {
                println!("fact combo {:?}", fact_combo);
                let mut eq = EqGraph::default();
                let mut clash = None;
                for (fact, antecedent) in fact_combo.iter().zip(rule.antecedents.iter()) {
                    if let Err(c) = equate(&mut eq, fact, antecedent) {
                        clash = Some(c);
                    }
                }
                println!("ULTIMATELY clash {:?} we get eq {:?} ", clash, eq);
            }
            // let's derive using this rule!
        }
    }
}
