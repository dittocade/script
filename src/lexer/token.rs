use std::fmt::{self, Debug};

use winnow::{error::ContextError, stream::{ContainsToken, TokenSlice}, token::literal, Parser, Result};

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Token<'s> {
    pub kind: Kind,
    pub value: &'s str
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Kind {
    Operator(Operator),
    Parenthesis(Handedness),
    Bracket(Handedness),
    Pipe,
    Comma,
    Dot,
    Name,
    Modifier,
    Reference,
    Label,
    Assignement,
    Definition,
    Float,
    Integer,
    Boolean,
    Skip,
    Comment,
    Unknown,
    EndOfFile,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    NotEqual,
    Equal,
    LessThan,
    AtMost,
    GreaterThan,
    AtLeast,
    Not,
    And,
    Or,
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Handedness {
    Opening,
    Closing,
}

impl PartialEq<Kind> for Token<'_> {
    fn eq(&self, other: &Kind) -> bool {
        self.kind == *other
    }
}

pub(crate) type Tokens<'i> = TokenSlice<'i, Token<'i>>;

impl<'i> Parser<Tokens<'i>, &'i Token<'i>, ContextError> for Kind {
    fn parse_next(&mut self, input: &mut Tokens<'i>) -> Result<&'i Token<'i>> {
        literal(*self).parse_next(input).map(|t| &t[0])
    }
}

impl fmt::Debug for Token<'_> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            Kind::Name | Kind::Float | Kind::Integer => Debug::fmt(self.value, fmt),
            _ => Debug::fmt(&self.kind, fmt),
        }
    }
}

impl ContainsToken<&'_ Token<'_>> for Kind {
    #[inline(always)]
    fn contains_token(&self, token: &'_ Token<'_>) -> bool {
        *self == token.kind
    }
}

impl ContainsToken<&'_ Token<'_>> for &'_ [Kind] {
    #[inline]
    fn contains_token(&self, token: &'_ Token<'_>) -> bool {
        self.iter().any(|t| *t == token.kind)
    }
}

impl<const LEN: usize> ContainsToken<&'_ Token<'_>> for &'_ [Kind; LEN] {
    #[inline]
    fn contains_token(&self, token: &'_ Token<'_>) -> bool {
        self.iter().any(|t| *t == token.kind)
    }
}

impl<const LEN: usize> ContainsToken<&'_ Token<'_>> for [Kind; LEN] {
    #[inline]
    fn contains_token(&self, token: &'_ Token<'_>) -> bool {
        self.iter().any(|t| *t == token.kind)
    }
}