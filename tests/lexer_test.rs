use minilang::frontend::lex;

#[test]
fn test_lex_numbers() {
    let tokens = lex("123 456");
    assert_eq!(tokens.len(), 2);
    assert!(matches!(tokens[0], Token::Int(123)));
    assert!(matches!(tokens[1], Token::Int(456)));
}

#[test]
fn test_lex_keywords() {
    let tokens = lex("fn let if");
    assert_eq!(tokens.len(), 3);
    assert!(matches!(tokens[0], Token::KwFn));
    assert!(matches!(tokens[1], Token::KwLet));
}