fn main()
{
    let mut a = 0;
    let mut b = &mut a;
    b = &mut *b;
}
