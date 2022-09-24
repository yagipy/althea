struct User {
    id: u64,
}

func main() u64 {
    let user1: User = User {
	    id: 1,
    };
    do_something(user1)
} // user1の参照先はここで解放される

func do_something(user: User) u64 {
    let user2: User = User {
        id: 2,
    };
    // 何かしらの処理
    let result = 100;
    result
} // user2の参照先はここで解放される(user(user1)の参照先は解放されない)
