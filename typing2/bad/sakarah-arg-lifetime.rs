fn f(mut a: &i32)
{
    let b = 0;
    a = &b;
}

fn main()
{
    let a = 0;
    f(&a)
}
