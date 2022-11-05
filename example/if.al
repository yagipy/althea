struct Foo {
    bar: u64,
    test: u64,
}

func main() u64 {
    let foo: Foo = Foo {
        bar: 42,
        test: 32,
    }
    let x = 0
    if foo.bar {
        let x = foo.bar
        x
    } else {
        foo.test
    }
    x
}
