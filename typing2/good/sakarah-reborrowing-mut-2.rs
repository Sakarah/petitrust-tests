fn main()
{
    let mut a = vec![0];
    let b = &mut a;
    &mut *b;
}
