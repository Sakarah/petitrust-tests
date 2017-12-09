struct i32
{
    a: bool
}

fn not(a: i32) -> i32
{
    let b = i32 { a: !a.a };
    b
}

fn main()
{
    let a = i32 { a: true };
    not(a).a;
}
