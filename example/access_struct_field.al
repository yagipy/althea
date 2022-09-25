// rust: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=2c851c5bfdafd3be328f7e891ada4f8d
struct User {
    id: u64,
}

//struct Id {
//    id: u64,
//}

func main() u64 {
    let user1: User = User { // ヒープ領域割り当て(%1)
        id: 1,
    };
    let s = 0;
    do_something(s, s)
    //user1.id
    //user1.id.id;
    //let d: u64 = do_something(user1); // %1の参照カウントをインクリメント(1になる)
    //d // aの参照カウントをデクリメント(0になる)=>ヒープ領域解放
}

func do_something(a: u64, b: u64) u64 {
    a
}

//func do_something(a: Foo, b: Foo) u64 {
//    let aaa = Foo { // ヒープ領域割り当て(%2)
//        bar: 2,
//    };
//    // 何か処理を実行
//    0 // aの参照カウントをデクリメント(0になる)、aaaは解放
//}
