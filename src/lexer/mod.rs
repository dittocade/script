use token::*;
use winnow::{
    ascii::{digit1, multispace0}, combinator::{alt, dispatch, eof, opt, peek, preceded, repeat, terminated}, stream::AsChar, token::{any, take_while}, Parser, Result
};

pub mod token;

pub fn token<'s>(i: &mut &'s str) -> Result<Token<'s>> {
    dispatch!{peek(any);
        '0'..='9' | '.' => alt((
            (digit1, '.', digit1).take().value(Kind::Float),
            (digit1, '.').take().value(Kind::Float),
            ('.', digit1).take().value(Kind::Float),
            (digit1).take().value(Kind::Integer),
            '.'.value(Kind::Dot),
        )),
        '(' => any.value(Kind::Parenthesis(Handedness::Opening)),
        ')' => any.value(Kind::Parenthesis(Handedness::Closing)),
        '{' => any.value(Kind::Bracket(Handedness::Opening)),
        '}' => any.value(Kind::Bracket(Handedness::Closing)),
        '|' => any.value(Kind::Pipe),
        '+' => any.value(Kind::Operator(Operator::Add)),
        '-' => any.value(Kind::Operator(Operator::Subtract)),
        '*' => preceded(any, opt('*')).map(|v| Kind::Operator(match v {
            Some(_) => Operator::Power,
            None => Operator::Multiply,
        })),
        '/' => any.value(Kind::Operator(Operator::Divide)),
        '=' => preceded(any, opt('=')).map(|v| match v {
            Some(_) => Kind::Operator(Operator::Equal),
            None => Kind::Assignement
        }),
        '<' => preceded(any, opt('=')).map(|v| Kind::Operator(match v {
            Some(_) => Operator::AtMost,
            None => Operator::LessThan,
        })),
        '>' => preceded(any, opt('=')).map(|v| Kind::Operator(match v {
            Some(_) => Operator::AtLeast,
            None => Operator::GreaterThan,
        })),
        '!' => preceded(any, opt('=')).map(|v| match v {
            Some(_) => Kind::Operator(Operator::NotEqual),
            None => Kind::Modifier,
        }),
        '$' => any.value(Kind::Modifier),
        '&' => any.value(Kind::Reference),
        'a'..='z' | 'A'..='Z' => take_while(0.., |char: char| char.is_ascii_alphanumeric() || char == '_').map(|v| match v {
            "not" => Kind::Operator(Operator::Not),
            "and" => Kind::Operator(Operator::And),
            "or" => Kind::Operator(Operator::Or),
            "def" => Kind::Definition,
            "True" | "False" => Kind::Boolean,
            _ => Kind::Name,
        }),
        '_' => any.value(Kind::Skip),
        ',' => any.value(Kind::Comma),
        ':' => any.value(Kind::Label),
        '#' => preceded('#', take_while(.., |char: char| !char.is_newline())).value(Kind::Comment),
        _ => take_while(.., |char: char| !char.is_ascii_whitespace()).value(Kind::Unknown)
    }
    .with_taken()
    .map(|(kind, value)| Token { kind, value})
    .parse_next(i)
}

pub fn tokens<'s>(i: &mut &'s str) -> Result<Vec<Token<'s>>> {
    let mut tokens: Vec<_> = preceded(
        multispace0,
        repeat(1.., terminated(token, multispace0))
    ).parse_next(i)?;

    let eof = opt(eof.map(|value| Token { kind: Kind::EndOfFile, value } )).parse_next(i)?;
    if let Some(eof) = eof {
        tokens.push(eof);
    }
    
    Ok(tokens)
}