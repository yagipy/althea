struct User {
    name: u64,
    email: u64,
}

func main() -> u64 {
    allocate(1000)
}

func allocate(size: u64) -> u64 {
    let u = User {
        name: size,
        email: size,
    };
    0
}
