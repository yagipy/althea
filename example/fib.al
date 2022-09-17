func fib(n: u64) u64 {
    if n <= 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

func main() u64 {
    fib(10)
}
