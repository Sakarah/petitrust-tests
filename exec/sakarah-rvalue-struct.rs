struct A
{
    x: i32
}

fn make() -> A
{
    let res = A { x:42 };
    res
}

fn main()
{
    if make().x == 42 { print!("Ok") }
}
