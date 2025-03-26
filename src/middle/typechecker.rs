use super::ast::{Expr, Type};

pub struct TypeEnv {
    vars: HashMap<String, Type>,
    functions: HashMap<String, (Vec<Type>, Type)>,
}

impl TypeEnv {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            functions: HashMap::new(),
        }
    }

    pub fn check_expr(&self, expr: &Expr) -> Result<Type, String> {
        match expr {
            Expr::Literal(Literal::Int(_)) => Ok(Type::Int),
            Expr::BinaryOp(left, op, right) => {
                let ltype = self.check_expr(left)?;
                let rtype = self.check_expr(right)?;
                match op {
                    BinOp::Add if ltype == Type::Int && rtype == Type::Int => Ok(Type::Int),
                    _ => Err(format!("Cannot apply {:?} to {} and {}", op, ltype, rtype)),
                }
            }
            _ => unimplemented!(),
        }
    }
}