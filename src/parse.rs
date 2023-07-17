use crate::Atom;
use crate::Rule;
use nom::branch::alt;
use nom::character::complete::alphanumeric1;
use nom::character::complete::multispace0;
use nom::character::complete::satisfy;
use nom::combinator::map as nommap;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::error::ParseError;
use nom::multi::many0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::terminated;

use nom::multi::many0_count;
use nom::multi::separated_list0;
use nom::sequence::pair;
use nom::{bytes::complete::tag, IResult};

pub fn wsl<'a, F, O, E>(inner: F) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    E: ParseError<&'a str>,
    F: FnMut(&'a str) -> IResult<&'a str, O, E> + 'a,
{
    preceded(multispace0, inner)
}

pub fn atom(s: &str) -> IResult<&str, Atom> {
    let t = |f: fn(&char) -> bool| {
        recognize(pair(
            satisfy(move |c| f(&c)),
            many0_count(alt((tag("_"), tag("-"), alphanumeric1))),
        ))
    };
    let id = nommap(t(char::is_ascii_lowercase), |c: &str| {
        Atom::Id(c.to_owned())
    });
    let tuple = delimited(
        tag("("),
        nommap(many0(atom), |mut x| {
            if x.len() == 1 {
                x.pop().unwrap()
            } else {
                Atom::Tuple(x)
            }
        }),
        wsl(tag(")")),
    );
    let var = nommap(t(char::is_ascii_uppercase), |c: &str| {
        Atom::Id(c.to_owned())
    });
    wsl(alt((id, tuple, var)))(s)
}

pub fn rule(s: &str) -> IResult<&str, Rule> {
    let (s, (consequent, maybe_antecedents)) = terminated(
        pair(atom, opt(preceded(wsl(tag(":-")), many0(atom)))),
        wsl(tag(".")),
    )(s)?;
    let antecedents = maybe_antecedents.unwrap_or_default();
    Ok((
        s,
        Rule {
            consequent,
            antecedents,
        },
    ))
}
