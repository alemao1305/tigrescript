#[derive(Debug)]
pub enum Expr {
    Literal(Literal),
    BinaryOp(Box<Expr>, BinOp, Box<Expr>),
    FnDef {
        name: String,
        params: Vec<(String, Type)>,
        body: Box<Expr>,
    },
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    String(String),
    Bool(bool),
}

#[derive(Debug)]
pub enum BinOp {
    Add,
    Sub,
    // ... outros operadores
}