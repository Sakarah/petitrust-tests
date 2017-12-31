fn main()
{
    let a = 0;
    let mut b = &a;
    let c = 0;
    let d = &c;
    b = &*d;
}
