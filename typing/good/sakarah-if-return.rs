fn f(a: bool) -> i32
{
    if a { return 0; a; } else { return 42; }
    a;
}

fn main()
{
    f(true);
}
