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
    pub name: String,
}

#[derive(Debug)]
pub struct File {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Callback {
    pub label: Label,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Invocation {
    pub outputs: Vec<Output>,
    pub name: String,
    pub inputs: Vec<Input>,
    pub callbacks: Vec<Callback>,
}

#[derive(Debug)]
pub struct Skip {}

#[derive(Debug)]
pub struct Definition {
    pub name: String,
    pub outputs: Vec<Output>,
    pub inputs: Vec<Input>,
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
pub enum Literal {
    Integer(Integer),
    Float(Float),
}

#[derive(Debug)]
pub enum Statement {
    Invocation(Invocation),
    Definition(Definition),
    Comment(Comment),
}

#[derive(Debug, Clone)]
pub enum Modifier {
    Default,
    Global,
    Saved,
}

impl Default for Modifier {
    fn default() -> Self { Modifier::Default }
}


#[derive(Debug)]
pub struct Name {
    pub modifier: Modifier,
    pub value: String,
}

#[derive(Debug)]
pub enum Expression {
    Literal(Literal),
    Call(Call),
    Skip(Skip),
    Name(Name),
}
