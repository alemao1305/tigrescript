#!/bin/bash

# Verifica se o Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "Erro: Rust não está instalado. Instale primeiro:"
    echo "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

# Configurações
TARGET="${1:-release}"
EXAMPLE="${2:-hello}"
OUTPUT_DIR="./bin"
STDLIB_DIR="./stdlib"

# Função para compilar
compile() {
    echo "🛠  Compilando $EXAMPLE.ml..."
    
    # Compila o compilador se necessário
    if [ ! -f "./target/$TARGET/minilang" ]; then
        echo "🔨 Construindo o compilador MiniLang..."
        cargo build --$TARGET
    fi
    
    # Cria diretório de saída
    mkdir -p "$OUTPUT_DIR"
    
    # Compila o exemplo
    "./target/$TARGET/minilang" build "./examples/$EXAMPLE.ml" -o "$OUTPUT_DIR/$EXAMPLE"
    
    # Copia a stdlib
    cp -r "$STDLIB_DIR" "$OUTPUT_DIR/"
    
    echo "✅ Compilação concluída! Executável em: $OUTPUT_DIR/$EXAMPLE"
}

# Função para executar
run() {
    echo "🚀 Executando $EXAMPLE..."
    "./$OUTPUT_DIR/$EXAMPLE"
}

# Menu principal
case "$1" in
    build)
        compile
        ;;
    run)
        compile && run
        ;;
    clean)
        echo "🧹 Limpando..."
        cargo clean
        rm -rf "$OUTPUT_DIR"
        ;;
    *)
        echo "Uso: $0 [build|run|clean] [exemplo]"
        echo "Exemplos disponíveis:"
        ls -1 examples/*.ml | sed 's/^examples\///;s/\.ml$//'
        exit 1
        ;;
esac