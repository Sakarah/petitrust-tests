fn f() -> i32
{
    if { return 0; true }
    {
        print!("unreachable");
    };
}

fn main()
{
    f();
}
