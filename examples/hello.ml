// Hello World em MiniLang
func main() {
    // Imprime uma mensagem
    print("Olá, Mundo!")
    
    // Calcula e mostra o fatorial
    let num = 5
    print("Fatorial de " + num.to_string() + " é: " + factorial(num).to_string())
}

func factorial(n: int) -> int {
    if n <= 1 { 1 }
    else { n * factorial(n - 1) }
}