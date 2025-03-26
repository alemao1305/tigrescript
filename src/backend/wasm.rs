use wasm_encoder::{
    Module, Instruction,
    ValType, Function, CodeSection
};

pub struct WasmGenerator;

impl WasmGenerator {
    pub fn generate(expr: &Expr) -> Vec<u8> {
        let mut module = Module::new();
        
        // Tipo da função (não recebe nada, retorna i32)
        let func_type = wasm_encoder::FuncType::new(
            vec![],
            vec![ValType::I32]
        );
        
        // Corpo da função
        let mut code = vec![];
        match expr {
            Expr::Literal(Literal::Int(n)) => {
                code.push(Instruction::I32Const(*n));
                code.push(Instruction::End);
            }
            _ => unimplemented!()
        };
        
        module.section(&CodeSection::new(vec![
            Function::new(code)
        ]));
        
        module.finish()
    }
}