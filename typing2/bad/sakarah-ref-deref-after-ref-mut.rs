fn main()
{
    let mut a = vec![0];
    let mut b = &mut a;
    let c = &mut b;
    let d = &*b;
}
