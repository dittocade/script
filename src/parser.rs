use std::str;

use winnow::{
    branch::alt,
    bytes::{one_of, tag},
    ascii::{alphanumeric1, multispace0, multispace1, not_line_ending},
    combinator::{opt, repeat},
    error::Error,
    multi::separated0,
    sequence::{delimited, preceded, separated_pair, terminated},
    Parser,
};

use crate::grammar;

pub fn assignment<'a>() -> impl Parser<&'a str, grammar::Assignment, Error<&'a str>> {
    separated_pair(
        separated0(output(), (tag(","), multispace0)),
        delimited(multispace0, tag("="), multispace0),
        invocation(),
    )
    .map(|(labels, value)| grammar::Assignment { labels, value })
}

pub fn output<'a>() -> impl Parser<&'a str, grammar::Output, Error<&'a str>> {
    (
        opt(terminated(
            alphanumeric1.map(str::to_string),
            (tag(":"), multispace0),
        )),
        name(),
    )
        .map(|(label, name)| grammar::Output {
            label: grammar::Label { name: label },
            name,
        })
}

pub fn invocation<'a>() -> impl Parser<&'a str, grammar::Invocation, Error<&'a str>> {
    (
        alphanumeric1,
        delimited(
            tag("("),
            separated0(input(), (tag(","), multispace0)),
            tag(")"),
        ),
        opt(preceded(
            multispace0,
            separated0(callback(), (tag(","), multispace0)),
        )),
    )
        .map(|(name, inputs, callbacks)| grammar::Invocation {
            name: name.to_string(),
            inputs,
            callbacks: match callbacks {
                None => Vec::new(),
                Some(callbacks) => callbacks,
            },
        })
}

pub fn call<'a>() -> impl Parser<&'a str, grammar::Call, Error<&'a str>> {
    (
        alphanumeric1,
        delimited(
            tag("("),
            separated0(input(), (tag(","), multispace0)),
            tag(")"),
        ),
    )
        .map(|(name, inputs)| grammar::Call {
            name: name.to_string(),
            inputs,
        })
}

pub fn comment<'a>() -> impl Parser<&'a str, grammar::Comment, Error<&'a str>> {
    preceded((tag("#"), multispace0), not_line_ending).map(|content: &str| grammar::Comment {
        content: content.to_string(),
    })
}

pub fn input<'a>() -> impl Parser<&'a str, grammar::Input, Error<&'a str>> {
    (
        opt(terminated(
            alphanumeric1.map(str::to_string),
            (tag(":"), multispace0),
        )),
        expression(),
    )
        .map(|(name, value)| grammar::Input {
            label: grammar::Label { name },
            value,
        })
}

pub fn callback<'a>() -> impl Parser<&'a str, grammar::Callback, Error<&'a str>> {
    (
        opt(terminated(
            alphanumeric1.map(str::to_string),
            (tag(":"), multispace0),
        )),
        delimited((tag("{"), multispace0), code(), (multispace0, tag("}"))),
    )
        .map(|(name, content)| grammar::Callback {
            label: grammar::Label { name },
            content,
        })
}

pub fn statement<'a>() -> impl Parser<&'a str, grammar::Statement, Error<&'a str>> {
    alt((
        assignment().map(grammar::Statement::Assignment),
        invocation().map(grammar::Statement::Invocation),
        comment().map(grammar::Statement::Comment),
    ))
}

pub fn code<'a>() -> impl Parser<&'a str, grammar::Code, Error<&'a str>> {
    separated0(statement(), multispace1).map(|statements| grammar::Code { statements })
}

pub fn skip<'a>() -> impl Parser<&'a str, grammar::Skip, Error<&'a str>> {
    tag("_").map(|_| grammar::Skip {})
}

pub fn integer<'a>() -> impl Parser<&'a str, grammar::Integer, Error<&'a str>> {
    repeat::<_, _, String, _, _>(1.., one_of("0123456789_"))
        .recognize()
        .try_map(|out: &str| str::replace(&out, "_", "").parse())
        .map(|value: i32| grammar::Integer { value })
}

pub fn float<'a>() -> impl Parser<&'a str, grammar::Float, Error<&'a str>> {
    alt((
        (
            repeat::<_, _, String, _, _>(1.., one_of("0123456789_")),
            tag("."),
            repeat::<_, _, String, _, _>(1.., one_of("0123456789_")),
        )
            .recognize(),
        (
            tag("."),
            repeat::<_, _, String, _, _>(1.., one_of("0123456789_")),
        )
            .recognize(),
        (
            repeat::<_, _, String, _, _>(1.., one_of("0123456789_")),
            tag("."),
        )
            .recognize(),
    ))
    .try_map(|out: &str| str::replace(&out, "_", "").parse())
    .map(|value: f64| grammar::Float { value })
}

pub fn name<'a>() -> impl Parser<&'a str, grammar::Name, Error<&'a str>> {
    alphanumeric1.map(|value: &str| grammar::Name {
        value: value.to_string(),
    })
}

pub fn literal<'a>() -> impl Parser<&'a str, grammar::Literal, Error<&'a str>> {
    alt((
        float().map(grammar::Literal::Float),
        integer().map(grammar::Literal::Integer),
        name().map(grammar::Literal::Name),
    ))
}

pub fn expression<'a>() -> impl Parser<&'a str, grammar::Expression, Error<&'a str>> {
    alt((
        skip().map(grammar::Expression::Skip),
        literal().map(grammar::Expression::Literal),
        call().map(grammar::Expression::Call),
    ))
}
