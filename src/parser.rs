use std::str;

use winnow::{
    ascii::{alphanumeric1, multispace0, multispace1, till_line_ending},
    combinator::{alt, delimited, opt, preceded, repeat, separated, separated_pair, terminated},
    error::InputError,
    token::any,
    Parser,
};

use crate::grammar;

pub fn assignment<'a>() -> impl Parser<&'a str, grammar::Assignment, InputError<&'a str>> {
    separated_pair(
        separated(0.., output(), (",", multispace0)),
        delimited(multispace0, "=", multispace0),
        invocation(),
    )
    .map(|(labels, value)| grammar::Assignment { labels, value })
}

pub fn output<'a>() -> impl Parser<&'a str, grammar::Output, InputError<&'a str>> {
    (
        opt(terminated(
            alphanumeric1.map(str::to_string),
            (":", multispace0),
        )),
        name(),
    )
        .map(|(label, name)| grammar::Output {
            label: grammar::Label { name: label },
            name,
        })
}

pub fn invocation<'a>() -> impl Parser<&'a str, grammar::Invocation, InputError<&'a str>> {
    (
        alphanumeric1,
        delimited("(", separated(0.., input(), (",", multispace0)), ")"),
        opt(preceded(
            multispace0,
            separated(0.., callback(), (",", multispace0)),
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

pub fn call<'a>() -> impl Parser<&'a str, grammar::Call, InputError<&'a str>> {
    (
        alphanumeric1,
        delimited("(", separated(0.., input(), (",", multispace0)), ")"),
    )
        .map(|(name, inputs)| grammar::Call {
            name: name.to_string(),
            inputs,
        })
}

pub fn comment<'a>() -> impl Parser<&'a str, grammar::Comment, InputError<&'a str>> {
    preceded(("#", multispace0), till_line_ending).map(|content: &str| grammar::Comment {
        content: content.to_string(),
    })
}

pub fn input<'a>() -> impl Parser<&'a str, grammar::Input, InputError<&'a str>> {
    (
        opt(terminated(
            alphanumeric1.map(str::to_string),
            (":", multispace0),
        )),
        expression(),
    )
        .map(|(name, value)| grammar::Input {
            label: grammar::Label { name },
            value,
        })
}

pub fn callback<'a>() -> impl Parser<&'a str, grammar::Callback, InputError<&'a str>> {
    (
        opt(terminated(
            alphanumeric1.map(str::to_string),
            (":", multispace0),
        )),
        delimited(("{", multispace0), code(), (multispace0, "}")),
    )
        .map(|(name, content)| grammar::Callback {
            label: grammar::Label { name },
            content,
        })
}

pub fn statement<'a>() -> impl Parser<&'a str, grammar::Statement, InputError<&'a str>> {
    alt((
        assignment().map(grammar::Statement::Assignment),
        invocation().map(grammar::Statement::Invocation),
        comment().map(grammar::Statement::Comment),
    ))
}

pub fn code<'a>() -> impl Parser<&'a str, grammar::Code, InputError<&'a str>> {
    separated(0.., statement(), multispace1).map(|statements| grammar::Code { statements })
}

pub fn skip<'a>() -> impl Parser<&'a str, grammar::Skip, InputError<&'a str>> {
    "_".map(|_| grammar::Skip {})
}

pub fn digits<'a>(radix: u32) -> impl Parser<&'a str, &'a str, InputError<&'a str>> {
    (
        any.verify(move |c: &char| c.is_digit(radix) || *c == '_'),
        repeat::<_, _, String, _, _>(
            0..,
            any.verify(move |c: &char| c.is_digit(radix) || *c == '_'),
        ),
    )
        .take()
}

pub fn integer<'a>() -> impl Parser<&'a str, grammar::Integer, InputError<&'a str>> {
    digits(10)
        .try_map(|out: &str| str::replace(&out, "_", "").parse())
        .map(|value: i32| grammar::Integer { value })
}

pub fn float<'a>() -> impl Parser<&'a str, grammar::Float, InputError<&'a str>> {
    alt((
        (
            digits(10),
            ".",
            digits(10),
        )
            .take(),
        (
            ".",
            digits(10),
        )
            .take(),
        (
            digits(10),
            ".",
        )
            .take(),
    ))
    .try_map(|out: &str| str::replace(&out, "_", "").parse())
    .map(|value: f64| grammar::Float { value })
}

pub fn name<'a>() -> impl Parser<&'a str, grammar::Name, InputError<&'a str>> {
    alphanumeric1.map(|value: &str| grammar::Name {
        value: value.to_string(),
    })
}

pub fn literal<'a>() -> impl Parser<&'a str, grammar::Literal, InputError<&'a str>> {
    alt((
        float().map(grammar::Literal::Float),
        integer().map(grammar::Literal::Integer),
        name().map(grammar::Literal::Name),
    ))
}

pub fn expression<'a>() -> impl Parser<&'a str, grammar::Expression, InputError<&'a str>> {
    alt((
        skip().map(grammar::Expression::Skip),
        literal().map(grammar::Expression::Literal),
        call().map(grammar::Expression::Call),
    ))
}
