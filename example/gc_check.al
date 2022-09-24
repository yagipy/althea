struct Foo {
    bar: u64,
}

func main() u64 {
    let a: Foo = Foo { // ヒープ領域割り当て(a)
        bar: 1, // 束縛
    };
    let b: Foo = a; // aの参照カウントをインクリメント(1になる)
    let d: u64 = do_something(a, b);
    d // aの参照カウントをデクリメント(0になる)=>ヒープ領域解放
}

func do_something(a: Foo, b: Foo) u64 {
    let c: Foo = a; // aの参照カウントをインクリメント(2になる)
    // 何か処理を実行
    0 // aの参照カウントをデクリメント(1になる)
}
