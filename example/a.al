struct Foo {
    bar: u64,
    test: u64,
}

func main() u64 {
    let foo: Foo = Foo {
        bar: 42,
        test: 32,
    };
    //match foo {
    //    Foo {
    //        bar: bar,
    //        test: test,
    //    } => bar + test,
    //}
    foo.bar + foo.test
}
