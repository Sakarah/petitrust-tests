fn main()
{
    let a = vec![0];
    let mut b = &a;
    let c = &mut b;
    &*b;
}
