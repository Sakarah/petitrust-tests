fn foo(a: &i32) { }

fn main()
{
    let mut a = 0;
    let mut b = &mut a;
    let mut c = &mut b;
    foo(b);
}
