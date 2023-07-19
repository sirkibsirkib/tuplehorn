use core::cell::Cell;
use core::iter::repeat;

use std::collections::HashMap;

use std::rc::Rc;

mod parse;

/*
proof refers to a RULE (consequent and antecedents)
defines a substitution that applies to all rule terms
identifies another proof per antecedent
- whose consequent
*/

type Id = String;
type Var = u16;
#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Atom {
    Tuple(Vec<Atom>),
    Id(Id),
    Var(Var),
}

// pub struct AnnotatedRule {
//     rule: Rule,
//     antecedent_vars: HashSet<Vr>,
//     consequent_vars: HashSet<u16>,
// }

pub struct Rule {
    consequent: Atom,
    antecedents: Vec<Atom>,
}

// #[derive(Debug)]
// struct ProofTree<'a> {
//     rule: &'a Rule,
//     rule_sub: Substitutions<'a>,
//     lemmata: Vec<ProofTree<'a>>,
// }

// #[derive(Default, Eq, PartialEq)]
// struct Substitutions<'a> {
//     subs: Vec<(Var, &'a Atom)>,
// }
// impl AnnotatedRule {
//     fn new(rule: Rule) -> Self {
//         let mut antecedent_vars = HashSet::default();
//         let mut consequent_vars = HashSet::default();
//         rule.consequent.visit_vars(&mut |var| drop(consequent_vars.insert(var)));
//         for antecedent in &rule.antecedents {
//             antecedent.visit_vars(&mut |var| drop(antecedent_vars.insert(var)));
//         }
//         Self { rule, antecedent_vars, consequent_vars }
//     }
// }
// impl<'a> Substitutions<'a> {
//     fn put(&mut self, k: u16, v: &'a Atom) -> bool {
//         for (k2, v2) in &self.subs {
//             if k == *k2 {
//                 if &v == v2 {
//                     return true;
//                 } else {
//                     return false;
//                 }
//             }
//         }
//         self.subs.push((k, v));
//         true
//     }
//     fn correct_root(&'a self, atom: &'a Atom) -> &'a Atom {
//         match atom {
//             Atom::Var(var) => self.get(*var).unwrap_or(atom),
//             _ => atom,
//         }
//     }
//     fn get(&self, var: u16) -> Option<&Atom> {
//         self.subs.iter().find(|(var2, _)| *var2 == var).map(|(_, atom)| atom).copied()
//     }
//     fn reset(&mut self) {
//         self.subs.clear()
//     }
//     fn unifies(&'a self, mut a: &'a Atom, mut b: &'a Atom) -> bool {
//         a = self.correct_root(a);
//         b = self.correct_root(b);
//         match [a, b] {
//             [Atom::Tuple(a), Atom::Tuple(b)] if a.len() == b.len() => {
//                 a.iter().zip(b.iter()).all(|(a, b)| self.unifies(a, b))
//             }
//             [Atom::Id(a), Atom::Id(b)] if a == b => true,
//             [Atom::Var(a), Atom::Var(b)] if a == b => true,
//             _ => false,
//         }
//     }
//     fn consistent_with(&self, other: &Self) -> bool {
//         if self.subs.len() > other.subs.len() {
//             return other.consistent_with(self);
//         }
//         for (var, atom) in self.subs.iter() {
//             match other.get(*var) {
//                 Some(atom2) if atom != &atom2 => return false,
//                 _ => {}
//             }
//         }
//         true
//     }
// }

impl Atom {
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
    // fn make_match<'a: 'c, 'b, 'c, 'd: 'c, 'e, 'f: 'c + 'd>(
    //     a: &'a Self,
    //     b: &'d Self,
    //     read_sub: &'e Substitutions<'f>,
    // ) -> bool {
    //     if let Self::Var(var) = read {
    //         if let Some(subbed) = read_sub.map.get(var) {
    //             read = subbed;
    //         }
    //     }
    //     match [write, read] {
    //         [Self::Id(w), Self::Id(r)] => w == r,
    //         [Self::Tuple(ws), Self::Tuple(rs)] if ws.len() == rs.len() => {
    //             for (w, r) in ws.iter().zip(rs.iter()) {
    //                 if !Self::make_match(w, write_sub, r, read_sub) {
    //                     return false;
    //                 }
    //             }
    //             true
    //         }
    //         [Self::Var(w), Self::Var(r)] if w == r => true,
    //         [Self::Var(w_var), r] => write_sub.sub(*w_var, r),
    //         _ => false,
    //     }
    // }
}

fn print_indent<T>(history: &[T]) {
    for _ in 0..history.len() {
        print!("  ");
    }
}

// impl<'a> ProofTree<'a> {
// fn check(&self) -> bool {
//     self.lemmata.len() == self.rule.antecedents.len()
//         && self.lemmata.iter().zip(self.rule.antecedents.iter()).all(|(lemma, antecedent)| {
//             self.rule_sub.consistent_with(&lemma.rule_sub)
//                 && self.rule_sub.unifies(antecedent, &lemma.rule.consequent)
//         })
// }
// fn create(ars: &'a [AnnotatedRule], goal: &'a Atom) -> Option<Self> {
//     let new_var_ids = HashMap::<u16,>
//     for ar in ars {
//         // fir a
//     }
//     None
// }
// fn create<'b>(
//     history: &mut Vec<&'a Atom>,
//     rules: &'a [Rule],
//     goal: &'a Atom,
//     sub: &'b mut Substitutions<'a>,
// ) -> Option<Self> {
//     if history.iter().find(|x| **x == goal).is_some() {
//         return None;
//     }
//     history.push(goal);
//     print_indent(history);
//     println!("goal: {:?} {:?}", goal, goal_sub);
//     let mut rule_sub = Substitutions::default();
//     for rule in rules {
//         print_indent(history);
//         println!("consider {:?}", rule);
//         rule_sub.reset();
//         for var in rule.consequent.vars() {
//             if let Some(val) = goal_sub.map.get(&var) {
//                 rule_sub.sub(var, val);
//             }
//         }
//         if Atom::make_match(&rule.consequent, &mut rule_sub, goal, goal_sub) {
//             print_indent(history);
//             println!("match rule {:?} {:?}", rule, rule_sub);
//             let maybe_lemmata: Option<_> = rule
//                 .antecedents
//                 .iter()
//                 .map(|subgoal| ProofTree::create(history, rules, subgoal, &rule_sub))
//                 .collect();

//             if let Some(lemmata) = maybe_lemmata {
//                 print_indent(history);
//                 println!("YES");
//                 history.pop();
//                 return Some(Self { rule, lemmata, rule_sub });
//             }
//         }
//     }
//     print_indent(history);
//     println!("NO");
//     history.pop();
//     None
// }
// }

// impl std::fmt::Debug for Substitutions<'_> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "[")?;
//         for (i, (k, v)) in self.subs.iter().enumerate() {
//             if i > 0 {
//                 write!(f, ", ")?;
//             }
//             write!(f, "{:?}/{:?}", k, v)?;
//         }
//         write!(f, "]")
//     }
// }

// struct Proof<'a> {
//     rule: &'a Rule,
//     var_ids: HashMap<Var, Rc<Cell<Id>>>,
// }
// impl<'a> Proof<'a> {
//     fn new(
//         rules: &'a [Rule],
//         goal: &'a Atom,
//         parent_var_ids: &HashMap<Var, Rc<Cell<Id>>>,
//     ) -> Option<Self> {
//         for rule in rules {

//         }
//         todo!()
//     }
// }

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
    // fn iter_combo(&self, len: usize) -> AtomCombo {
    //     AtomCombo {
    //         kb: self,
    //         next_indices: Some(repeat(0).take(len).collect()),
    //         buf: Vec::with_capacity(len),
    //     }
    // }
}

// struct AtomCombo<'a> {
//     kb: &'a Kb,
//     next_indices: Option<Box<[usize]>>,
//     buf: Vec<&'a Atom>,
// }
// impl<'a> AtomCombo<'a> {
//     fn next(&mut self) -> Option<&[&Atom]> {
//         let next_indices = self.next_indices.as_mut()?;
//         self.buf.clear();
//         // if this is in bounds, prepare the buffer
//         for &idx in next_indices.iter() {
//             if let Some(atom) = self.kb.vec.get(idx) {
//                 self.buf.push(atom);
//             } else {
//                 // ok this was the end
//                 self.next_indices = None;
//                 return None;
//             }
//         }
//         // increment next indices
//         if let Some(last) = next_indices.last_mut() {
//             *last += 1;
//             if *last >= self.kb.vec.len() {
//                 // roll over!
//             }
//         }
//         for i in (0..next_indices.len()).rev() {
//             next_indices[i] += 1;
//             if next_indices[i] >= self.kb.vec.len() {
//                 next_indices[i] = 0;

//             }
//         }
//         Some(&self.buf)
//     }
// }

fn main() {
    let rules = "
    0 :- (auth 1) (1 say 0).
    (auth amy).
    (amy say (bob cool)).
    win :- (bob cool).
    ";
    let rules = parse::wsr(parse::rules)(rules).unwrap().1;
    println!("{:#?}", rules);

    let mut kb = Kb::default();

    for _ in 0..1000 {
        for rule in &rules {
            let mut ci = combo_iter::BoxComboIter::new(&kb.vec, rule.antecedents.len());
            while let Some(fact_combo) = ci.next() {
                println!("{:?}", fact_combo);
            }
            // let's derive using this rule!
        }
    }
    // let ars: Vec<_> = rules.into_iter().map(AnnotatedRule::new).collect();
    // let goal = parse::wsr(parse::atom)("win").unwrap().1;
    // println!("{:#?}", goal);
    // let goal_sub = Substitutions::default();
    // let mut history = vec![];
    // let proof = ProofTree::create(&mut history, &rules, &goal, &sub);
    // println!("{:#?}", proof);
}
