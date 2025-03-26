# tigrescript - Linguagem de Programação Moderna
MiniLang Logo (adicione um logo depois)

MiniLang é uma linguagem de programação moderna projetada para ser segura, expressiva e de alto desempenho. Combina a simplicidade de sintaxe de Python com o sistema de tipos poderoso de Rust e a concorrência prática de Go.

✨ Características Principais
Sistema de tipos estático com inferência - Tipagem forte sem verbosidade

Gerenciamento de memória híbrido - Ownership + GC opcional

Concorrência de alto nível - Modelo de atores e async/await

Compilação para múltiplos alvos - Native (via LLVM), WASM e SPIR-V

Metaprogramação segura - Macros higiênicas e CTFE

Interoperabilidade - FFI seguro e suporte a WASM

📦 Instalação
Pré-requisitos
Rust nightly (para compilar o compilador)

LLVM 10+ (para geração de código nativo)

Clang (para linking)

bash
Copy
# Instale o Rust (se necessário)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install nightly
rustup default nightly

# Clone o repositório
git clone https://github.com/seu-usuario/minilang.git
cd minilang
🚀 Como Usar
Compilar e executar um programa:
bash
Copy
# Compilar para executável nativo
./run build exemplo

# Executar
./run run exemplo

# Compilar para WASM
./run build --target=wasm exemplo
Exemplos incluídos:
bash
Copy
# Hello World
./run run hello

# Servidor Web
./run run web_server

# Cálculo de Fibonacci
./run run fib
📚 Estrutura do Projeto
Copy
minilang/
├── src/              # Código fonte do compilador
├── stdlib/           # Biblioteca padrão
├── examples/         # Programas de exemplo
├── tests/            # Testes automatizados
├── run               # Script de compilação (Unix)
├── run.bat           # Script de compilação (Windows)
└── Cargo.toml        # Configuração do projeto Rust
🛠️ Componentes Principais
Frontend

Lexer/Parser com recuperação de erros

AST tipada

Diagnósticos ricos

Middle-end

Typechecker com inferência

Otimizações

Transformações de código

Backend

LLVM (para código nativo)

WASM (para web)

SPIR-V (para GPU)

Runtime

Gerenciamento de memória

Sistema de concorrência

Coletor de lixo opcional

📝 Exemplo de Código
rust
Copy
// Programa de exemplo - Fibonacci
func fib(n: int) -> int {
    if n <= 1 { n }
    else { fib(n-1) + fib(n-2) }
}

func main() {
    print("Fib(10) = ", fib(10))
}
🤝 Como Contribuir
Faça um fork do projeto

Crie uma branch (git checkout -b feature/AmazingFeature)

Commit suas mudanças (git commit -m 'Add some AmazingFeature')

Push para a branch (git push origin feature/AmazingFeature)

Abra um Pull Request

📄 Licença
Distribuído sob a licença MIT. Veja LICENSE para mais informações.

✉️ Contato
Seu Nome - @seu_twitter - seu-email@exemplo.com

Link do Projeto: https://github.com/seu-usuario/minilang

🎁 Agradecimentos
Equipe do Rust por inspirar o sistema de ownership

Desenvolvedores do LLVM pela infraestrutura de compilação

Comunidade de linguagens de programação por feedback valioso
