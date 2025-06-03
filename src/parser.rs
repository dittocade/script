use crate::grammar::*;
use winnow::{
    ascii::{digit1, multispace0, till_line_ending}, combinator::{alt, empty, opt, repeat, separated, seq, terminated}, error::StrContext, token::{one_of, take_while}, Parser, Result
};

pub fn skip<'s>(i: &mut &'s str) -> Result<Skip> {
    seq!{Skip{
        _: '_'
    }}.context(StrContext::Label("placeholder")).parse_next(i)
}

pub fn identifier<'s>(i: &mut &'s str) -> Result<String> {
    seq!(
        one_of( |char: char| char.is_alphabetic()),
        take_while(0.., |char: char| char.is_alphanumeric() || matches!(char, '_' | ' '))
    ).take().context(StrContext::Label("identifier")).map(str::to_string).parse_next(i)
}

pub fn name<'s>(i: &mut &'s str) -> Result<Name> {
    seq!{Name{
        modifier: alt((
            '!'.value(Modifier::Saved),
            '$'.value(Modifier::Global),
            empty.default_value(),
        )),
        value: identifier,
    }}.context(StrContext::Label("name")).parse_next(i)
}

pub fn float<'s>(i: &mut &'s str) -> Result<Float> {
    (opt('-'), digit1, '.', digit1).context(StrContext::Label("floating-point number")).take().parse_to().map(|value| Float{value}).parse_next(i)
}

pub fn integer<'s>(i: &mut &'s str) -> Result<Integer> {
    (opt('-'), digit1).context(StrContext::Label("integer")).take().parse_to().map(|value| Integer{value}).parse_next(i)
}

pub fn literal<'s>(i: &mut &'s str) -> Result<Literal> {
    alt((
        float.map(Literal::Float),
        integer.map(Literal::Integer)
    )).context(StrContext::Label("literal")).parse_next(i)
}

pub fn label<'s>(i: &mut &'s str) -> Result<Label> {
    seq!{Label{
        name: opt(terminated(identifier,(':', multispace0)))
    }}.context(StrContext::Label("label")).parse_next(i)
}

pub fn input<'s>(i: &mut &'s str) -> Result<Input> {
    seq!{Input{
        label: label,
        value: expression,
    }}.context(StrContext::Label("input")).parse_next(i)
}

pub fn call<'s>(i: &mut &'s str) -> Result<Call> {
    seq!{Call{
        name: identifier,
        _: multispace0,
        _: '(',
        _: multispace0,
        inputs: separated(0.., input, (',', multispace0)),
        _: ')',
    }}.context(StrContext::Label("call")).parse_next(i)
}

pub fn expression<'s>(i: &mut &'s str) -> Result<Expression> {
    alt((
        skip.map(Expression::Skip),
        literal.map(Expression::Literal),
        call.map(Expression::Call),
        name.map(Expression::Name),
    )).context(StrContext::Label("expression")).parse_next(i)
}

pub fn callback<'s>(i: &mut &'s str) -> Result<Callback> {
    seq!{Callback{
        label: seq!{Label{
            name: opt(identifier),
            _: multispace0
        }},
        _: '{',
        _: multispace0,
        statements: repeat(0.., terminated(statement, multispace0)),
        _: '}'
    }}.context(StrContext::Label("callback")).parse_next(i)
}

pub fn invocation<'s>(i: &mut &'s str) -> Result<Invocation> {
    seq!{Invocation{
        outputs: opt(terminated(separated(1.., output, (',', multispace0)), (multispace0, '=', multispace0))).map(|v| v.unwrap_or_default()),
        name: identifier,
        _: multispace0,
        _: '(',
        _: multispace0,
        inputs: separated(0.., input, (',', multispace0)),
        _: ')',
        _: multispace0,
        callbacks: repeat(0.., terminated(callback, multispace0))
    }}.context(StrContext::Label("invocation")).parse_next(i)
}
pub fn output<'s>(i: &mut &'s str) -> Result<Output> {
    seq!{Output{
        label: label,
        name: identifier,
    }}.context(StrContext::Label("output")).parse_next(i)
}

pub fn definition<'s>(i: &mut &'s str) -> Result<Definition> {
    seq!{Definition{
        _: '@',
        outputs: separated(1.., output, (',', multispace0)),
        _: multispace0,
        _: '=',
        _: multispace0,
        name: identifier,
        _: multispace0,
        _: '(',
        _: multispace0,
        inputs: separated(0.., input, (',', multispace0)),
        _: multispace0,
        _: ')',
        _: multispace0,
        callbacks: repeat(0.., terminated(callback, multispace0))
    }}.context(StrContext::Label("definition")).parse_next(i)
}

pub fn comment<'s>(i: &mut &'s str) -> Result<Comment> {
    seq!{Comment{
        _: '#',
        content: till_line_ending.map(str::trim).map(str::to_string)
    }}.context(StrContext::Label("comment")).parse_next(i)
}

pub fn statement<'s>(i: &mut &'s str) -> Result<Statement> {
    alt((
        invocation.map(Statement::Invocation),
        definition.map(Statement::Definition),
        comment.map(Statement::Comment),
    )).context(StrContext::Label("statement")).parse_next(i)
}

pub fn file<'s>(i: &mut &'s str) -> Result<File> {
    seq!{File{
        _: multispace0,
        statements: repeat(0.., terminated(statement, multispace0)),
    }}.context(StrContext::Label("file")).parse_next(i)
}