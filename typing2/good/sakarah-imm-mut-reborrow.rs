fn main()
{
    let mut a = 0;
    let x = &mut a;
    let b = 0;
    let mut w = &b;

    let y = x;
    w = &*y;
}
