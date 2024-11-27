use crate::grammar;
use std::str;

use winnow::{
    ascii::{alphanumeric1, multispace0, multispace1, till_line_ending},
    combinator::{alt, delimited, opt, preceded, repeat, separated, separated_pair, terminated},
    error::InputError,
    token::any,
    PResult, Parser,
};

pub fn assignment<'a>(inp: &mut &'a str) -> PResult<grammar::Assignment, InputError<&'a str>> {
    separated_pair(
        separated(0.., output, (",", multispace0)),
        delimited(multispace0, "=", multispace0),
        invocation,
    )
    .map(|(labels, value)| grammar::Assignment { labels, value })
    .parse_next(inp)
}

pub fn output<'a>(inp: &mut &'a str) -> PResult<grammar::Output, InputError<&'a str>> {
    (
        opt(terminated(
            alphanumeric1.map(str::to_string),
            (":", multispace0),
        )),
        name,
    )
        .map(|(label, name)| grammar::Output {
            label: grammar::Label { name: label },
            name,
        })
        .parse_next(inp)
}

pub fn invocation<'a>(inp: &mut &'a str) -> PResult<grammar::Invocation, InputError<&'a str>> {
    (
        alphanumeric1,
        delimited("(", separated(0.., input, (",", multispace0)), ")"),
        opt(preceded(
            multispace0,
            separated(0.., callback, (",", multispace0)),
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
        .parse_next(inp)
}

pub fn call<'a>(inp: &mut &'a str) -> PResult<grammar::Call, InputError<&'a str>> {
    (
        alphanumeric1,
        delimited("(", separated(0.., input, (",", multispace0)), ")"),
    )
        .map(|(name, inputs)| grammar::Call {
            name: name.to_string(),
            inputs,
        })
        .parse_next(inp)
}

pub fn comment<'a>(inp: &mut &'a str) -> PResult<grammar::Comment, InputError<&'a str>> {
    preceded(("#", multispace0), till_line_ending)
        .map(|content: &str| grammar::Comment {
            content: content.to_string(),
        })
        .parse_next(inp)
}

pub fn input<'a>(inp: &mut &'a str) -> PResult<grammar::Input, InputError<&'a str>> {
    (
        opt(terminated(
            alphanumeric1.map(str::to_string),
            (":", multispace0),
        )),
        expression,
    )
        .map(|(name, value)| grammar::Input {
            label: grammar::Label { name },
            value,
        })
        .parse_next(inp)
}

pub fn callback<'a>(inp: &mut &'a str) -> PResult<grammar::Callback, InputError<&'a str>> {
    (
        opt(terminated(
            alphanumeric1.map(str::to_string),
            (":", multispace0),
        )),
        delimited(("{", multispace0), code, (multispace0, "}")),
    )
        .map(|(name, content)| grammar::Callback {
            label: grammar::Label { name },
            content,
        })
        .parse_next(inp)
}

pub fn statement<'a>(inp: &mut &'a str) -> PResult<grammar::Statement, InputError<&'a str>> {
    alt((
        assignment.map(grammar::Statement::Assignment),
        invocation.map(grammar::Statement::Invocation),
        comment.map(grammar::Statement::Comment),
    ))
    .parse_next(inp)
}

pub fn code<'a>(inp: &mut &'a str) -> PResult<grammar::Code, InputError<&'a str>> {
    separated(0.., statement, multispace1)
        .map(|statements| grammar::Code { statements })
        .parse_next(inp)
}

pub fn skip<'a>(inp: &mut &'a str) -> PResult<grammar::Skip, InputError<&'a str>> {
    "_".map(|_| grammar::Skip {}).parse_next(inp)
}

pub fn digits<'a>(inp: &mut &'a str) -> PResult<&'a str, InputError<&'a str>> {
    (
        any.verify(move |c: &char| c.is_digit(10) || *c == '_'),
        repeat::<_, _, String, _, _>(0.., any.verify(move |c: &char| c.is_digit(10) || *c == '_')),
    )
        .take()
        .parse_next(inp)
}

pub fn integer<'a>(inp: &mut &'a str) -> PResult<grammar::Integer, InputError<&'a str>> {
    digits
        .try_map(|out: &str| str::replace(&out, "_", "").parse())
        .map(|value: i32| grammar::Integer { value })
        .parse_next(inp)
}

pub fn float<'a>(inp: &mut &'a str) -> PResult<grammar::Float, InputError<&'a str>> {
    alt((
        (digits, ".", digits).take(),
        (".", digits).take(),
        (digits, ".").take(),
    ))
    .try_map(|out: &str| str::replace(&out, "_", "").parse())
    .map(|value: f64| grammar::Float { value })
    .parse_next(inp)
}

pub fn name<'a>(inp: &mut &'a str) -> PResult<grammar::Name, InputError<&'a str>> {
    alphanumeric1
        .map(|value: &str| grammar::Name {
            value: value.to_string(),
        })
        .parse_next(inp)
}

pub fn literal<'a>(inp: &mut &'a str) -> PResult<grammar::Literal, InputError<&'a str>> {
    alt((
        float.map(grammar::Literal::Float),
        integer.map(grammar::Literal::Integer),
        name.map(grammar::Literal::Name),
    ))
    .parse_next(inp)
}

pub fn expression<'a>(inp: &mut &'a str) -> PResult<grammar::Expression, InputError<&'a str>> {
    alt((
        skip.map(grammar::Expression::Skip),
        literal.map(grammar::Expression::Literal),
    ))
    .parse_next(inp)
}
