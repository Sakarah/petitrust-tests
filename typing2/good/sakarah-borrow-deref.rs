fn main()
{
    let mut a = vec![0];
    let b = &mut a;
    let c = &b;
    let d = &*b;
}
