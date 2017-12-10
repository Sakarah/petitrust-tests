fn ignore(a: i32, b: i32, c: i32) {}

fn f() -> i32
{
    ignore(3, {return 0; 1}, 4);
}

fn main()
{
    f();
}
