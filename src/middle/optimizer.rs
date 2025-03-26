use super::ast::Expr;

pub fn optimize(expr: Expr) -> Expr {
    match expr {
        Expr::BinaryOp(left, op, right) => {
            let left_opt = optimize(*left);
            let right_opt = optimize(*right);
            
            match (&left_opt, &right_opt) {
                (Expr::Literal(Literal::Int(a)), Expr::Literal(Literal::Int(b))) => {
                    match op {
                        BinOp::Add => Expr::Literal(Literal::Int(a + b)),
                        BinOp::Sub => Expr::Literal(Literal::Int(a - b)),
                        _ => Expr::BinaryOp(Box::new(left_opt), op, Box::new(right_opt))
                    }
                }
                _ => Expr::BinaryOp(Box::new(left_opt), op, Box::new(right_opt))
            }
        }
        _ => expr
    }
}