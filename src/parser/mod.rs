use crate::lexer::token::*;
use grammar::*;
use winnow::{
    combinator::{
        alt, delimited, opt, repeat, separated, separated_foldl1, separated_foldr1, seq, terminated,
    },
    error::StrContext,
    token::one_of,
    Parser, Result,
};

pub mod grammar;

pub fn name(i: &mut Tokens) -> Result<String> {
    Kind::Name.map(|&v| v.value.to_string()).parse_next(i)
}

pub fn names(i: &mut Tokens) -> Result<Vec<String>> {
    separated(.., name, Kind::Comma).parse_next(i)
}

pub fn integer(i: &mut Tokens) -> Result<i32> {
    Kind::Integer.map(|&v| v.value).parse_to().parse_next(i)
}

pub fn float(i: &mut Tokens) -> Result<f64> {
    Kind::Float.map(|&v| v.value).parse_to().parse_next(i)
}

pub fn boolean(i: &mut Tokens) -> Result<bool> {
    Kind::Boolean
        .map(|&v| v.value)
        .map(|v| match v {
            "True" => true,
            "False" => false,
            _ => unreachable!(),
        })
        .parse_next(i)
}

pub fn string(i: &mut Tokens) -> Result<String> {
    Kind::String.map(|&v| v.value[1..v.value.len() - 1].to_string()).parse_next(i)
}

pub fn modifier(i: &mut Tokens) -> Result<Modifier> {
    Kind::Modifier
        .map(|v| match v.value {
            "$" => Modifier::Global,
            "!" => Modifier::Saved,
            _ => unreachable!(),
        })
        .parse_next(i)
}

pub fn variable(i: &mut Tokens) -> Result<Expression> {
    seq! {Expression::Variable {
        modifier: opt(modifier),
        name: name,
    }}
    .parse_next(i)
}

pub fn input(i: &mut Tokens) -> Result<Input> {
    seq! {Input {
        label: opt(terminated(name, Kind::Label)),
        value: expression
    }}
    .parse_next(i)
}

pub fn inputs0(i: &mut Tokens) -> Result<Vec<Input>> {
    separated(.., input, Kind::Comma).parse_next(i)
}

pub fn call(i: &mut Tokens) -> Result<Expression> {
    seq! {Expression::Call{
        name: name,
        _: Kind::Parenthesis(Handedness::Opening),
        inputs: inputs0,
        _: Kind::Parenthesis(Handedness::Closing),
    }}
    .context(StrContext::Label("call"))
    .parse_next(i)
}

pub fn grouping(i: &mut Tokens) -> Result<Expression> {
    delimited(
        Kind::Parenthesis(Handedness::Opening),
        expression,
        Kind::Parenthesis(Handedness::Closing),
    )
    .parse_next(i)
}

pub fn simple_expression(i: &mut Tokens) -> Result<Expression> {
    alt((
        grouping,
        Kind::Skip.map(|_| Expression::Skip),
        float.map(Expression::Float),
        integer.map(Expression::Integer),
        boolean.map(Expression::Boolean),
        string.map(Expression::String),
        call,
        variable,
    ))
    .parse_next(i)
}

pub fn prefix_expression(i: &mut Tokens) -> Result<Expression> {
    (
        repeat(
            ..,
            one_of([
                Kind::Operator(Operator::Not),
                Kind::Operator(Operator::Subtract),
            ]),
        ),
        simple_expression,
    )
        .map(|(ops, a): (Vec<&Token>, _)| {
            ops.iter().fold(a, |a, op| {
                let Kind::Operator(op) = op.kind else {
                    unreachable!();
                };
                Expression::Call {
                    name: match op {
                        Operator::Not => "not",
                        Operator::Subtract => "negate",
                        _ => unreachable!(),
                    }
                    .to_string(),
                    inputs: vec![Input {
                        label: None,
                        value: a,
                    }],
                }
            })
        })
        .parse_next(i)
}

pub fn exponentative_expression(i: &mut Tokens) -> Result<Expression> {
    separated_foldr1(
        prefix_expression,
        one_of([Kind::Operator(Operator::Power)]),
        |a, _, b| Expression::Call {
            name: "power".to_string(),
            inputs: vec![
                Input {
                    label: None,
                    value: a,
                },
                Input {
                    label: None,
                    value: b,
                },
            ],
        },
    )
    .parse_next(i)
}

pub fn multiplicative_expression(i: &mut Tokens) -> Result<Expression> {
    separated_foldl1(
        exponentative_expression,
        one_of([
            Kind::Operator(Operator::Multiply),
            Kind::Operator(Operator::Divide),
        ]),
        |a, op, b| {
            let Kind::Operator(op) = op.kind else {
                unreachable!();
            };
            Expression::Call {
                name: match op {
                    Operator::Multiply => "add",
                    Operator::Divide => "subtract",
                    _ => unreachable!(),
                }
                .to_string(),
                inputs: vec![
                    Input {
                        label: None,
                        value: a,
                    },
                    Input {
                        label: None,
                        value: b,
                    },
                ],
            }
        },
    )
    .parse_next(i)
}

pub fn additive_expression(i: &mut Tokens) -> Result<Expression> {
    separated_foldl1(
        multiplicative_expression,
        one_of([
            Kind::Operator(Operator::Add),
            Kind::Operator(Operator::Subtract),
        ]),
        |a, op, b| {
            let Kind::Operator(op) = op.kind else {
                unreachable!();
            };
            Expression::Call {
                name: match op {
                    Operator::Add => "add",
                    Operator::Subtract => "subtract",
                    _ => unreachable!(),
                }
                .to_string(),
                inputs: vec![
                    Input {
                        label: None,
                        value: a,
                    },
                    Input {
                        label: None,
                        value: b,
                    },
                ],
            }
        },
    )
    .parse_next(i)
}

pub fn relational_expression(i: &mut Tokens) -> Result<Expression> {
    separated_foldl1(
        additive_expression,
        one_of([
            Kind::Operator(Operator::LessThan),
            Kind::Operator(Operator::AtMost),
            Kind::Operator(Operator::GreaterThan),
            Kind::Operator(Operator::AtLeast),
        ]),
        |a, op, b| {
            let Kind::Operator(op) = op.kind else {
                unreachable!();
            };
            Expression::Call {
                name: match op {
                    Operator::LessThan => "less_than",
                    Operator::AtMost => "at_most",
                    Operator::GreaterThan => "greater_than",
                    Operator::AtLeast => "at_least",
                    _ => unreachable!(),
                }
                .to_string(),
                inputs: vec![
                    Input {
                        label: None,
                        value: a,
                    },
                    Input {
                        label: None,
                        value: b,
                    },
                ],
            }
        },
    )
    .parse_next(i)
}

pub fn equality_expression(i: &mut Tokens) -> Result<Expression> {
    separated_foldl1(
        relational_expression,
        one_of([
            Kind::Operator(Operator::Equal),
            Kind::Operator(Operator::NotEqual),
        ]),
        |a, op, b| {
            let Kind::Operator(op) = op.kind else {
                unreachable!();
            };
            Expression::Call {
                name: match op {
                    Operator::Equal => "equal",
                    Operator::NotEqual => "not_equal",
                    _ => unreachable!(),
                }
                .to_string(),
                inputs: vec![
                    Input {
                        label: None,
                        value: a,
                    },
                    Input {
                        label: None,
                        value: b,
                    },
                ],
            }
        },
    )
    .parse_next(i)
}

pub fn logical_expression(i: &mut Tokens) -> Result<Expression> {
    separated_foldl1(
        equality_expression,
        one_of([Kind::Operator(Operator::And), Kind::Operator(Operator::Or)]),
        |a, op, b| {
            let Kind::Operator(op) = op.kind else {
                unreachable!();
            };
            Expression::Call {
                name: match op {
                    Operator::And => "and",
                    Operator::Or => "or",
                    _ => unreachable!(),
                }
                .to_string(),
                inputs: vec![
                    Input {
                        label: None,
                        value: a,
                    },
                    Input {
                        label: None,
                        value: b,
                    },
                ],
            }
        },
    )
    .parse_next(i)
}

pub fn expression(i: &mut Tokens) -> Result<Expression> {
    logical_expression.parse_next(i)
}

pub fn output(i: &mut Tokens) -> Result<Output> {
    seq! {Output {
        label: opt(terminated(name, Kind::Label)),
        name: alt((
            name.map(Some),
            Kind::Skip.value(None),
        ))
    }}
    .parse_next(i)
}

pub fn outputs0(i: &mut Tokens) -> Result<Vec<Output>> {
    separated(.., output, Kind::Comma).parse_next(i)
}

pub fn outputs1(i: &mut Tokens) -> Result<Vec<Output>> {
    separated(1.., output, Kind::Comma).parse_next(i)
}

pub fn callback(i: &mut Tokens) -> Result<Callback> {
    seq! {Callback {
        label: opt(name),
        outputs: opt(delimited(Kind::Pipe, outputs0, Kind::Pipe)).map(Option::unwrap_or_default),
        _: Kind::Bracket(Handedness::Opening),
        statements: statements0,
        _: Kind::Bracket(Handedness::Closing),
    }}
    .parse_next(i)
}

pub fn callbacks0(i: &mut Tokens) -> Result<Vec<Callback>> {
    repeat(.., callback).parse_next(i)
}

pub fn comment(i: &mut Tokens) -> Result<Statement> {
    Kind::Comment
        .map(|v| Statement::Comment(v.value[1..].trim().to_string()))
        .parse_next(i)
}

pub fn assignement(i: &mut Tokens) -> Result<Statement> {
    seq! {Statement::Assignement {
        outputs: opt(terminated(outputs1, Kind::Assignement)).map(Option::unwrap_or_default),
        value: expression,
    }}
    .parse_next(i)
}

pub fn invocation(i: &mut Tokens) -> Result<Statement> {
    seq! {Statement::Invocation {
        outputs: opt(terminated(outputs0, Kind::Assignement)).map(Option::unwrap_or_default),
        name: name,
        _: Kind::Parenthesis(Handedness::Opening),
        inputs: inputs0,
        _: Kind::Parenthesis(Handedness::Closing),
        callbacks: callbacks0,
    }}
    .parse_next(i)
}

pub fn definition(i: &mut Tokens) -> Result<Statement> {
    seq! {Statement::Definition {
        _: Kind::Definition,
        name: name,
        _: Kind::Parenthesis(Handedness::Opening),
        inputs: names,
        _: Kind::Parenthesis(Handedness::Closing),
        callbacks: names,
        outputs: opt(delimited(Kind::Pipe, names, Kind::Pipe)).map(Option::unwrap_or_default),
        _: Kind::Bracket(Handedness::Opening),
        statements: statements0,
        _: Kind::Bracket(Handedness::Closing),
    }}
    .parse_next(i)
}

pub fn statement(i: &mut Tokens) -> Result<Statement> {
    alt((comment, invocation, assignement, definition)).parse_next(i)
}

pub fn statements0(i: &mut Tokens) -> Result<Vec<Statement>> {
    repeat(.., statement).parse_next(i)
}
