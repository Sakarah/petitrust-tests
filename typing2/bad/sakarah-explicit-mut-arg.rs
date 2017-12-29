fn foo(x: &i32, y: &i32) { }

fn main()
{
    let mut a = 0;
    foo(&mut a, &mut a);
}
