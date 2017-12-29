fn main()
{
    let mut a = 0;
    let mut b = &a;
    let c = &mut b;
    *b;
}
