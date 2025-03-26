use minilang::{
    frontend::{lex, Parser},
    middle::TypeEnv,
    ast::{Expr, Type}
};

#[test]
fn test_typecheck_literals() {
    let tokens = lex("42");
    let expr = Parser::new(tokens).parse().unwrap();
    
    let mut type_env = TypeEnv::new();
    let ty = type_env.check_expr(&expr).unwrap();
    
    assert_eq!(ty, Type::Int);
}

#[test]
fn test_typecheck_binop() {
    let tokens = lex("1 + 2");
    let expr = Parser::new(tokens).parse().unwrap();
    
    let mut type_env = TypeEnv::new();
    let ty = type_env.check_expr(&expr).unwrap();
    
    assert_eq!(ty, Type::Int);
}

#[test]
fn test_typecheck_function() {
    let tokens = lex("fn sum(a: int, b: int) -> int { a + b }");
    let expr = Parser::new(tokens).parse().unwrap();
    
    let mut type_env = TypeEnv::new();
    let ty = type_env.check_expr(&expr).unwrap();
    
    assert_eq!(ty, Type::Fn(
        vec![Type::Int, Type::Int], 
        Box::new(Type::Int)
    ));
}

#[test]
fn test_typecheck_error() {
    let tokens = lex("1 + true");
    let expr = Parser::new(tokens).parse().unwrap();
    
    let mut type_env = TypeEnv::new();
    let result = type_env.check_expr(&expr);
    
    assert!(result.is_err());
}