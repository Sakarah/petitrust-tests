struct A { a: i32 }

fn main()
{
    let mut a = A { a: 0 };
    let b = &mut a;
    a.a;
}
