#[derive(Debug)]
pub enum Statement {
    Assignment {
        var: String,
        r#type: Option<Primitive>,
        value: Box<Expr>,
    },
    Expression(Box<Expr>),
}

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Str(String),
    Bool(bool),
    Var(String),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
    LocalBinding {
        declarations: Vec<Statement>,
        expression: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Primitive {
    Num,
    Str,
    Bool,
}

#[derive(Debug)]
pub enum BinaryOp {
    Multiply,
    Divide,
    Add,
    Subtract,
}

#[derive(Debug)]
pub enum UnaryOp {
    Negate,
}
