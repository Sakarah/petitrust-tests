fn main()
{
    let mut a = 0;
    let b = &mut a;
    let mut c = 0;
    let mut d = b;
    d = &mut c;
}
