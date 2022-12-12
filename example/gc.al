struct User {
    id: u64,
}

func main() u64 {
    let user: User = User {
        id: 1,
    }
    let result: u64 = do_something(user)
    result
}

func do_something(user1: User) u64 {
    let tmp_user: User = User {
        id: 2,
    }
    tmp_user.id + user1.id
}

func do_something2(user1: u64) u64 {
    let tmp_user: User = User {
        id: 2,
    }
    tmp_user.id + user1
}
