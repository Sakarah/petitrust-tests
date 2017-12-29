fn main()
{
    let mut a1 = vec![0];
    let b1 = &mut a1;
    let mut c = &*b1;

    let mut a2 = vec![1];
    let b2 = &mut a2;
    c = &*b2;
}
