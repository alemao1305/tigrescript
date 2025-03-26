fn main() {
    // Verifica se o LLVM está instalado
    if std::env::var("LLVM_SYS_100_PREFIX").is_err() {
        println!("cargo:warning=LLVM not found. Some features may be disabled.");
    }
    
    // Gera código para a biblioteca padrão
    println!("cargo:rerun-if-changed=stdlib");
}