fn foo(x: &i32, y: &mut i32) { }

fn main()
{
    let mut a = 0;
    let ref_a = &mut a;

    foo(ref_a, ref_a);
}
