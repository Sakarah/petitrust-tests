fn f() -> i32
{
    let a = { if true { return 42; } else { return 0; }};
}

fn main()
{
    f();
}
