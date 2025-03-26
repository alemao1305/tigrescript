pub struct X86Generator;

impl X86Generator {
    pub fn generate(expr: &Expr) -> Vec<u8> {
        let mut asm = Vec::new();
        
        // Prologue
        asm.extend(b"\x55\x48\x89\xE5"); // push rbp; mov rbp, rsp
        
        match expr {
            Expr::Literal(Literal::Int(n)) => {
                // mov eax, <n>
                asm.push(0xB8);
                asm.extend(&n.to_le_bytes()[..4]);
            }
            _ => unimplemented!()
        }
        
        // Epilogue
        asm.extend(b"\x5D\xC3"); // pop rbp; ret
        
        asm
    }
}