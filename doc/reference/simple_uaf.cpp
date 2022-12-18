struct Foo {
    int var;
};

int main() {
    Foo *foo = new Foo;
    delete foo;

    return foo->var;
}
