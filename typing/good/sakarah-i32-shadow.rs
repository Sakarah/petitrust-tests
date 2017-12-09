struct i32
{
    a: bool
}

fn not(a: i32) -> i32
{
    i32 { a: !a.a }
}

fn main()
{
    let a = i32 { a: true };
    not(a).a;
}
