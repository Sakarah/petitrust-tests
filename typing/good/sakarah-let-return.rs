fn f() -> i32
{
    let a = { if true { return 42; false } else { return 0; true } };
}

fn main()
{
    f();
}
