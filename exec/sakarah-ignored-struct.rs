struct A
{
    x: i32
}

fn main()
{
    let a = A { x: 42 };
    let b = A { x: 32 };
    a;
    if b.x == 32 { print!("Ok") }
}
