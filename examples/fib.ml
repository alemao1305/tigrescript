func fib(n: int) -> int {
    if n <= 1 { n }
    else { fib(n - 1) + fib(n - 2) }
}

func main() {
    print(fib(10))
}