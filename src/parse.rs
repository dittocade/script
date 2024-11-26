use std::str;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0, multispace1, not_line_ending, one_of},
    combinator::{map, map_res, opt, recognize},
    multi::{many1, separated_list0},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    IResult,
};

use crate::grammar;

pub fn assignment(inp: &str) -> IResult<&str, grammar::Assignment> {
    map(
        separated_pair(
            separated_list0(pair(tag(","), multispace0), output),
            delimited(multispace0, tag("="), multispace0),
            invocation
        ),
        |(labels, value)| grammar::Assignment {
            labels,
            value
        },
    )(inp)
}

pub fn output(inp: &str) -> IResult<&str, grammar::Output> {
    map(
        pair(
            opt(terminated(map(alphanumeric1, str::to_string), pair(tag(":"), multispace0))),
            name,
        ),
        |(label, name)| grammar::Output {
            label: grammar::Label { name: label },
            name,
        },
    )(inp)
}

pub fn invocation(inp: &str) -> IResult<&str, grammar::Invocation> {
    map(
        tuple((
            alphanumeric1,
            delimited(tag("("), separated_list0(pair(tag(","), multispace0), input), tag(")")),
            opt(preceded( multispace0, separated_list0(pair(tag(","), multispace0), callback)))
        )),
        |(name, inputs, callbacks)| grammar::Invocation {
            name: name.to_string(),
            inputs,
            callbacks: match callbacks {
                None => Vec::new(),
                Some(callbacks) => callbacks
            },
        },
    )(inp)
}

pub fn call(inp: &str) -> IResult<&str, grammar::Call> {
    map(
        pair(
            alphanumeric1,
            delimited(tag("("), separated_list0(pair(tag(","), multispace0), input), tag(")")),
        ),
        |(name, inputs)| grammar::Call {
            name: name.to_string(),
            inputs,
        },
    )(inp)
}

pub fn comment(inp: &str) -> IResult<&str, grammar::Comment> {
    map(
        preceded(
            pair(tag("#"), multispace0),
            not_line_ending,
        ),
        |content: &str| grammar::Comment { content: content.to_string() },
    )(inp)
}

pub fn input(inp: &str) -> IResult<&str, grammar::Input> {
    map(
        pair(
            opt(terminated(map(alphanumeric1, str::to_string), pair(tag(":"), multispace0))),
            expression,
        ),
        |(name, value)| grammar::Input {
            label: grammar::Label { name },
            value,
        },
    )(inp)
}

pub fn callback(inp: &str) -> IResult<&str, grammar::Callback> {
    map(
        pair(
            opt(terminated(map(alphanumeric1, str::to_string), pair(tag(":"), multispace0))),
            delimited(pair(tag("{"), multispace0), code, pair(multispace0, tag("}"))),
        ),
        |(name, content)| grammar::Callback {
            label: grammar::Label { name },
            content,
        },
    )(inp)
}

pub fn statement(inp: &str) -> IResult<&str, grammar::Statement> {
    alt((
        map(assignment, grammar::Statement::Assignment),
        map(invocation, grammar::Statement::Invocation),
        map(comment, grammar::Statement::Comment),
    ))(inp)
}


pub fn code(inp: &str) -> IResult<&str, grammar::Code> {
    map(separated_list0(multispace1, statement), |statements| grammar::Code {statements})(inp)
}

pub fn skip(inp: &str) -> IResult<&str, grammar::Skip> {
    map(tag("_"), |_| grammar::Skip {})(inp)
}

pub fn integer(inp: &str) -> IResult<&str, grammar::Integer> {
    map(
        map_res(recognize(many1(one_of("0123456789_"))), |out: &str| {
            str::replace(&out, "_", "").parse()
        }),
        |value: i32| grammar::Integer { value },
    )(inp)
}

pub fn float(inp: &str) -> IResult<&str, grammar::Float> {
    map(
        map_res(alt((
            recognize(tuple((many1(one_of("0123456789_")), tag("."), many1(one_of("0123456789_"))))),
            recognize(tuple((tag("."), many1(one_of("0123456789_"))))),
            recognize(tuple((many1(one_of("0123456789_")), tag(".")))),
        )), |out: &str| {
            str::replace(&out, "_", "").parse()
        }),
        |value: f64| grammar::Float { value },
    )(inp)
}

pub fn name(inp: &str) -> IResult<&str, grammar::Name> {
    map(
        alphanumeric1,
        |value: &str| grammar::Name { value: value.to_string() },
    )(inp)
}

pub fn literal(inp: &str) -> IResult<&str, grammar::Literal> {
    alt((
        map(float, grammar::Literal::Float),
        map(integer, grammar::Literal::Integer),
        map(name, grammar::Literal::Name),
    ))(inp)
}

pub fn expression(inp: &str) -> IResult<&str, grammar::Expression> {
    alt((
        map(call, grammar::Expression::Call),
        map(skip, grammar::Expression::Skip),
        map(literal, grammar::Expression::Literal),
    ))(inp)
}
