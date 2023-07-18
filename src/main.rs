use std::collections::BTreeMap;
use std::collections::HashSet;

mod parse;

#[derive(Eq, PartialEq, Hash)]
pub enum Atom {
    Tuple(Vec<Atom>),
    Id(String),
    Variable(u16),
}

pub struct Rule {
    consequent: Atom,
    antecedents: Vec<Atom>,
}

#[derive(Debug)]
struct ProofTree<'a> {
    rule: &'a Rule,
    rule_sub: Substitutions<'a>,
    lemmata: Vec<ProofTree<'a>>,
}

#[derive(Default, Eq, PartialEq)]
struct Substitutions<'a> {
    subs: Vec<(u16, &'a Atom)>,
}
impl<'a> Substitutions<'a> {
    fn sub(&mut self, k: u16, v: &'a Atom) -> bool {
        for (k2, v2) in &self.subs {
            if k == *k2 {
                if &v == v2 {
                    return true;
                } else {
                    return false;
                }
            }
        }
        self.subs.push((k, v));
        true
    }
    fn reset(&mut self) {
        self.subs.clear()
    }
}

impl Atom {
    fn vars(&self) -> HashSet<u16> {
        let mut x = HashSet::default();
        self.visit_vars(&mut |var| drop(x.insert(var)));
        x
    }
    fn visit_vars(&self, visit: &mut impl FnMut(u16)) {
        match self {
            Self::Id(_) => {}
            Self::Variable(v) => visit(*v),
            Self::Tuple(args) => {
                for arg in args {
                    arg.visit_vars(visit)
                }
            }
        }
    }
    fn make_match<'a: 'c, 'b, 'c, 'd: 'c, 'e, 'f: 'c + 'd>(
        write: &'a Self,
        write_sub: &'b mut Substitutions<'c>,
        mut read: &'d Self,
        read_sub: &'e Substitutions<'f>,
    ) -> bool {
        if let Self::Variable(var) = read {
            if let Some(subbed) = read_sub.map.get(var) {
                read = subbed;
            }
        }
        match [write, read] {
            [Self::Id(w), Self::Id(r)] => w == r,
            [Self::Tuple(ws), Self::Tuple(rs)] if ws.len() == rs.len() => {
                for (w, r) in ws.iter().zip(rs.iter()) {
                    if !Self::make_match(w, write_sub, r, read_sub) {
                        return false;
                    }
                }
                true
            }
            [Self::Variable(w), Self::Variable(r)] if w == r => true,
            [Self::Variable(w_var), r] => write_sub.sub(*w_var, r),
            _ => false,
        }
    }
}

fn print_indent<T>(history: &[T]) {
    for _ in 0..history.len() {
        print!("  ");
    }
}

impl<'a> ProofTree<'a> {
    fn check(&self) -> bool {
        todo!()
    }
    fn create<'b>(
        history: &mut Vec<&'a Atom>,
        rules: &'a [Rule],
        goal: &'a Atom,
        sub: &'b mut Substitutions<'a>,
    ) -> Option<Self> {
        if history.iter().find(|x| **x == goal).is_some() {
            return None;
        }
        history.push(goal);
        print_indent(history);
        println!("goal: {:?} {:?}", goal, goal_sub);
        let mut rule_sub = Substitutions::default();
        for rule in rules {
            print_indent(history);
            println!("consider {:?}", rule);
            rule_sub.reset();
            for var in rule.consequent.vars() {
                if let Some(val) = goal_sub.map.get(&var) {
                    rule_sub.sub(var, val);
                }
            }
            if Atom::make_match(&rule.consequent, &mut rule_sub, goal, goal_sub) {
                print_indent(history);
                println!("match rule {:?} {:?}", rule, rule_sub);
                let maybe_lemmata: Option<_> = rule
                    .antecedents
                    .iter()
                    .map(|subgoal| ProofTree::create(history, rules, subgoal, &rule_sub))
                    .collect();

                if let Some(lemmata) = maybe_lemmata {
                    print_indent(history);
                    println!("YES");
                    history.pop();
                    return Some(Self { rule, lemmata, rule_sub });
                }
            }
        }
        print_indent(history);
        println!("NO");
        history.pop();
        None
    }
}

impl std::fmt::Debug for Substitutions<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, (k, v)) in self.map.iter().enumerate() {
            if i > 0 {
                write!(f, ", ")?;
            }
            write!(f, "{:?}/{:?}", k, v)?;
        }
        write!(f, "]")
    }
}
impl std::fmt::Debug for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Variable(v) => v.fmt(f),
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

fn main() {
    let rules = "
    0 :- (auth 1) (1 say 0) .
    (auth amy).
    (amy say (bob cool)).
    win :- (bob cool).
    ";
    let rules = parse::wsr(parse::rules)(rules).unwrap().1;
    println!("{:#?}", rules);
    let goal = parse::wsr(parse::atom)("win").unwrap().1;
    println!("{:#?}", goal);
    let goal_sub = Substitutions::default();
    let mut history = vec![];
    let proof = ProofTree::create(&mut history, &rules, &goal, &sub);
    println!("{:#?}", proof);
}
