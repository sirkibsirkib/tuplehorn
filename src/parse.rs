use crate::Atom;
use crate::Rule;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::character::complete::satisfy;
use nom::character::complete::{alphanumeric1, u16 as nomu16};
use nom::combinator::map as nommap;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::error::ParseError;
use nom::multi::many0;
use nom::multi::many0_count;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::preceded;
use nom::sequence::terminated;
use nom::{bytes::complete::tag, IResult};

pub fn wsl<'a, F, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: FnMut(&'a str) -> IResult<&'a str, O, E> + 'a,
{
    preceded(multispace0, inner)
}

pub fn wsr<'a, F, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: FnMut(&'a str) -> IResult<&'a str, O, E> + 'a,
{
    terminated(inner, multispace0)
}

pub fn atom(s: &str) -> IResult<&str, Atom> {
    let t = |f: fn(&char) -> bool| {
        recognize(pair(
            satisfy(move |c| f(&c)),
            many0_count(alt((tag("_"), tag("-"), alphanumeric1))),
        ))
    };
    let id = nommap(t(char::is_ascii_lowercase), |c: &str| Atom::Id(c.to_owned()));
    let tuple = delimited(
        tag("("),
        nommap(many0(atom), |mut x| if x.len() == 1 { x.pop().unwrap() } else { Atom::Tuple(x) }),
        wsl(tag(")")),
    );
    let var = nommap(nomu16, Atom::Variable);
    wsl(alt((id, tuple, var)))(s)
}

pub fn rules(s: &str) -> IResult<&str, Vec<Rule>> {
    many0(rule)(s)
}

pub fn rule(s: &str) -> IResult<&str, Rule> {
    let (s, (consequent, maybe_antecedents)) =
        terminated(pair(atom, opt(preceded(wsl(tag(":-")), many0(atom)))), wsl(tag(".")))(s)?;
    let antecedents = maybe_antecedents.unwrap_or_default();
    Ok((s, Rule { consequent, antecedents }))
}
