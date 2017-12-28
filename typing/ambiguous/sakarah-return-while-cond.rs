fn f() -> i32
{
    while { return 0; true }
    {
        print!("unreachable");
    }
}

fn main()
{
    f();
}
