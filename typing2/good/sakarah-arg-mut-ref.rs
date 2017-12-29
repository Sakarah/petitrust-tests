fn foo(x: &i32, y: &mut i32) { }

fn main()
{
    let mut a = 0;
    let ref_a = &mut a;

    let b = 0;
    let ref_b = &b;

    foo(ref_b, ref_a);
    ref_a;
}
