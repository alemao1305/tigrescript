use rustyline::Editor;
use crate::frontend::{lex, Parser};
use crate::middle::TypeEnv;

pub struct Repl {
    rl: Editor<()>,
    type_env: TypeEnv,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            rl: Editor::<()>::new(),
            type_env: TypeEnv::new(),
        }
    }

    pub fn run(&mut self) {
        println!("MiniLang REPL (v0.1)");
        println!("Enter ':q' to quit");

        loop {
            let readline = self.rl.readline(">> ");
            match readline {
                Ok(line) if line.trim() == ":q" => break,
                Ok(line) => self.eval(&line),
                Err(_) => break,
            }
        }
    }

    fn eval(&mut self, input: &str) {
        let tokens = lex(input);
        let mut parser = Parser::new(tokens);
        match parser.parse() {
            Ok(expr) => {
                match self.type_env.check_expr(&expr) {
                    Ok(ty) => println!("=> {:?} : {}", expr, ty),
                    Err(e) => eprintln!("Type error: {}", e),
                }
            }
            Err(e) => eprintln!("Parse error: {}", e),
        }
    }
}