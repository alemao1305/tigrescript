use minilang::frontend::{lex, Parser, ast::{Expr, Literal}};

#[test]
fn test_parse_literals() {
    let tokens = lex("42 \"hello\" true");
    let mut parser = Parser::new(tokens);
    
    let expr = parser.parse().unwrap();
    
    assert!(matches!(expr, Expr::Block(exprs) if exprs.len() == 3));
    
    if let Expr::Block(exprs) = expr {
        assert!(matches!(&exprs[0], Expr::Literal(Literal::Int(42))));
        assert!(matches!(&exprs[1], Expr::Literal(Literal::String(s)) if s == "hello"));
        assert!(matches!(&exprs[2], Expr::Literal(Literal::Bool(true))));
    }
}

#[test]
fn test_parse_function() {
    let tokens = lex("fn sum(a: int, b: int) -> int { a + b }");
    let mut parser = Parser::new(tokens);
    
    let expr = parser.parse().unwrap();
    
    if let Expr::FnDef { name, params, body } = expr {
        assert_eq!(name, "sum");
        assert_eq!(params.len(), 2);
        assert!(matches!(*body, Expr::BinaryOp(..)));
    } else {
        panic!("Not a function definition");
    }
}

#[test]
fn test_parse_error() {
    let tokens = lex("fn {");
    let mut parser = Parser::new(tokens);
    
    assert!(parser.parse().is_err());
}