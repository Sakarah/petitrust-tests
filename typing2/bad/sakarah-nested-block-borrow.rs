fn main()
{
    let mut a = 1;
    let b = { let c = 3; &mut a };
    &mut a;
}
