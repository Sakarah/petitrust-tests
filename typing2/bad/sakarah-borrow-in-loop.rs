fn main()
{
    let a = 0;
    let mut b2 = &a;
    let mut b1 = &a;
    let mut c = &mut b1;
    while true
    {
        c = &mut b2;
    }
}
