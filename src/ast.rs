#[derive(Debug)]
pub enum Statement {
    LetDeclaration {
        var: String,
        r#type: Option<Primitive>,
        value: Expr,
    },
    Expression(Expr),
}

#[derive(Debug)]
pub enum Expr {
    Num(f64),
    Var(String),
    BinaryOp(Box<Expr>, BinaryOp, Box<Expr>),
    UnaryOp(UnaryOp, Box<Expr>),
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
