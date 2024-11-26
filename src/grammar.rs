#[derive(Debug)]
pub struct Label {
    pub name: Option<String>,
}

#[derive(Debug)]
pub struct Input {
    pub label: Label,
    pub value: Expression,
}

#[derive(Debug)]
pub struct Output {
    pub label: Label,
    pub name: Name,
}

#[derive(Debug)]
pub struct Assignment {
    pub labels: Vec<Output>,
    pub value: Invocation,
}

#[derive(Debug)]
pub struct Code {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Callback {
    pub label: Label,
    pub content: Code,
}

#[derive(Debug)]
pub struct Invocation {
    pub name: String,
    pub inputs: Vec<Input>,
    pub callbacks: Vec<Callback>,
}

#[derive(Debug)]
pub struct Skip {}

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub outputs: Vec<Label>,
    pub inputs: Vec<Expression>,
    pub callbacks: Vec<Callback>,
}

#[derive(Debug)]
pub struct Comment {
    pub content: String,
}

#[derive(Debug)]
pub struct Call {
    pub name: String,
    pub inputs: Vec<Input>,
}

#[derive(Debug)]
pub struct Integer {
    pub value: i32,
}

#[derive(Debug)]
pub struct Float {
    pub value: f64,
}

#[derive(Debug)]
pub struct Name {
    pub value: String,
}

#[derive(Debug)]
pub enum Literal {
    Integer(Integer),
    Float(Float),
    Name(Name),
}

#[derive(Debug)]
pub enum Statement {
    Assignment(Assignment),
    Invocation(Invocation),
    Definition(Definition),
    Comment(Comment),
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Call(Call),
    Skip(Skip),
}
