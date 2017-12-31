fn foo(x: &i32) { }

fn main()
{
    let a = 0;
    let mut b = &a;
    let c = &mut b;
    let d = &*b;
    *d;
}
