struct A
{
    x: i32
}

fn make(x: i32) -> A
{
    let a = A { x:x };
    let b = A { x:2*x };
    return a;
    b
}

fn main()
{
    let a = make(42);
    if a.x == 42 { print!("Ok") }
}
