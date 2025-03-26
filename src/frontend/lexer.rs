use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token {
    #[regex(r"-?[0-9]+", |lex| lex.slice().parse().ok())]
    Int(i64),
    
    #[regex(r#""([^"\\]|\\.)*""#, |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    String(String),
    
    #[token("fn")] KwFn,
    #[token("let")] KwLet,
    #[token("+")] Plus,
    #[token("{")] BraceOpen,
    
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", |lex| lex.slice().to_string())]
    Ident(String),
}

pub fn lex(source: &str) -> Vec<Token> {
    Token::lexer(source).collect()
}