use minilang::{
    frontend::{lex, Parser},
    middle::{TypeEnv, optimize},
    backend::{CodeGenerator, WasmGenerator},
    tools::Repl,
};
use std::{
    fs,
    path::Path,
    env,
    process::exit
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    // Modo REPL interativo
    if args.len() == 1 {
        Repl::new().run();
        return Ok(());
    }

    // Comandos específicos
    match args[1].as_str() {
        "run" if args.len() > 2 => run_file(&args[2])?,
        "build" if args.len() > 2 => build_file(&args[2])?,
        "test" => run_tests()?,
        _ => {
            eprintln!("Uso: minilang [run|build] <arquivo.ml>");
            exit(1);
        }
    }

    Ok(())
}

fn run_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    let tokens = lex(&source);
    let ast = Parser::new(tokens).parse()?;
    
    let mut type_env = TypeEnv::new();
    type_env.check_expr(&ast)?;
    
    let optimized = optimize(ast);
    
    let mut vm = minilang::runtime::VirtualMachine::new();
    vm.execute(&optimized)?;
    
    Ok(())
}

fn build_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let source = fs::read_to_string(path)?;
    let tokens = lex(&source);
    let ast = Parser::new(tokens).parse()?;
    
    let mut type_env = TypeEnv::new();
    type_env.check_expr(&ast)?;
    
    let optimized = optimize(ast);
    
    // Gera WASM por padrão
    let wasm = WasmGenerator::generate(&optimized);
    fs::write("output.wasm", wasm)?;
    
    println!("Arquivo 'output.wasm' gerado com sucesso!");
    Ok(())
}

fn run_tests() -> Result<(), Box<dyn std::error::Error>> {
    println!("Executando testes...");
    // Implementação dos testes
    Ok(())
}