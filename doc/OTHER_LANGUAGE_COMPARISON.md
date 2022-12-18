## 他言語比較
<!--
- サンプルコードはできれば複数出す
-->

### C++
<!--
- 手動メモリ管理が原因の脆弱性が含まれるコードを書く
  - 参考: https://docs.google.com/presentation/d/1WHmCLeC5ZPiq2MBOQaZc-pNVWaJanx8eXAkViGl2zws/mobilepresent?slide=id.p
- その後、Altheaでそのような脆弱性が含まれるコードが書けないことを示す
-->

C++では、メモリの確保と解放を基本的には手動で行う必要があるため、use after free等のメモリ管理に関する脆弱性が含まれるコードを書けてしまう。

```cpp
struct Foo {
    int var;
};

int main() {
    Foo *foo = new Foo;
    // ...
    delete foo;
    // ...
    return foo->var;
}
```

TODO: use-after-freeによるvtable overwriteのサンプルコードを書く

Altheaでは、メモリの確保と解放を自動で行うため、メモリ管理に関する脆弱性が含まれるコードは基本的に書けない。

```althea
struct Foo {
    var: u64,
}

func main() u64 {
    let foo: Foo = Foo { 
        var: 1,
    }
    foo.var
}
```

### Rust
<!--
- 所有権によるコンパイルエラーが発生するコードを書く
- その後、Altheaでそのようなコンパイルエラーが発生しないことを示す
-->

Rustでは、所有権やライフタイムの管理を手動で行う必要があり、正しくハンドリングしないとコンパイルエラーが発生する。

```rust
struct Foo {
    var: i64,
}

fn main() {
    let foo = Foo {
        var: 1,
    };
    let var = foo;
    println!("{}", foo.var);
}
```
https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=9ac9efb976137278902ad4693166b8ff

Altheaでは、所有権やライフタイムの管理を自動で行うため、こういった場合にコンパイルエラーは発生しない。

```althea
struct Foo {
    var u64,
}

func main() u64 {
    let foo: Foo = Foo {
        var: 1,
    };
    let bar = foo
    println(foo.var)
    0
}
```

### Go
<!--
- GoとAltheaでパフォーマンスを計測して比較を行う
- できればビジュアライズされた結果を出す
-->

