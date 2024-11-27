use core::f64;

use crate::grammar;
use chumsky::prelude::*;

pub fn digits<'a>(radix: u32) -> impl Parser<'a, &'a str, &'a str> {
    text::int(radix)
        .then(
            any()
                .filter(move |c: &char| c.is_digit(radix) || *c == '_')
                .repeated()
                .collect::<String>(),
        )
        .to_slice()
}

pub fn integer<'a>() -> impl Parser<'a, &'a str, grammar::Integer> {
    just("-")
        .or(just("+"))
        .or(just(""))
        .then(digits(10))
        .to_slice()
        .map(|s| grammar::Integer {
            value: s.replace("_", "").parse().unwrap(),
        })
}

pub fn float<'a>() -> impl Parser<'a, &'a str, grammar::Float> {
    (just("-").or(just("+")).or(just("")))
        .then(digits(10).then(just(".")).then(digits(10)))
        .to_slice()
        .map(|s| s.replace("_", "").parse().unwrap())
        .or((just("+").or_not().then(just("inf")).to(f64::INFINITY))
            .or(just("-").then(just("inf")).to(f64::NEG_INFINITY))
            .or(just("nan").to(f64::NAN)))
        .map(|value| grammar::Float { value })
}

pub fn name<'a>() -> impl Parser<'a, &'a str, grammar::Name> {
    text::ascii::ident().map(|s: &str| grammar::Name {
        value: s.to_string(),
    })
}

pub fn literal<'a>() -> impl Parser<'a, &'a str, grammar::Literal> {
    (float().map(grammar::Literal::Float))
        .or(integer().map(grammar::Literal::Integer))
        .or(name().map(grammar::Literal::Name))
}

pub fn label<'a>() -> impl Parser<'a, &'a str, grammar::Label> {
    text::ascii::ident()
        .then_ignore(just(":").padded())
        .map(str::to_string)
        .or_not()
        .map(|name| grammar::Label { name })
}

pub fn input<'a>() -> impl Parser<'a, &'a str, grammar::Input> {
    label()
        .then(expression())
        .map(|(label, value)| grammar::Input { label, value })
}

pub fn call<'a>() -> impl Parser<'a, &'a str, grammar::Call> {
    Box::new(
        text::ascii::ident()
            .then(
                input()
                    .separated_by(just(",").padded())
                    .allow_trailing()
                    .collect::<Vec<_>>()
                    .delimited_by(just("("), just(")")),
            )
            .map(|(name, inputs)| grammar::Call {
                name: name.to_string(),
                inputs,
            }),
    )
}

pub fn skip<'a>() -> impl Parser<'a, &'a str, grammar::Skip> {
    just("_").map(|_| grammar::Skip {})
}

pub trait ExpressionParser: Sized {
    fn parser<'a>() -> Boxed<'a, 'a, &'a str, Self, extra::Default>;
}

impl ExpressionParser for grammar::Expression {
    fn parser<'a>() -> Boxed<'a, 'a, &'a str, Self, extra::Default> {
        (skip().map(grammar::Expression::Skip))
            .or(literal().map(grammar::Expression::Literal))
            .or(call().map(grammar::Expression::Call))
            .boxed()
    }
}

pub fn expression<'a>() -> impl Parser<'a, &'a str, grammar::Expression> {
    ExpressionParser::parser()
}