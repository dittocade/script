pub mod parse;

#[derive(Debug)]
#[allow(unused)]
pub struct Input {
    pub label: Option<String>,
    pub value: Expression,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Output {
    pub label: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Callback {
    pub label: Option<String>,
    pub outputs: Vec<Output>,
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Statement {
    Invocation {
        name: String,
        inputs: Vec<Input>,
        outputs: Vec<Output>,
        callbacks: Vec<Callback>,
    },
    Assignement {
        value: Expression,
        outputs: Vec<Output>,
    },
    Definition {
        name: String,
        inputs: Vec<String>,
        outputs: Vec<String>,
        callbacks: Vec<String>,
        statements: Vec<Statement>,
    },
    Comment(String),
}

#[derive(Debug)]
#[allow(unused)]
pub enum Modifier {
    Global,
    Saved,
}

#[derive(Debug)]
#[allow(unused)]
pub enum Expression {
    Skip,
    Float(f64),
    Integer(i32),
    Boolean(bool),
    String(String),
    Call {
        name: String,
        inputs: Vec<Input>,
    },
    Variable {
        modifier: Option<Modifier>,
        name: String,
    },
}
