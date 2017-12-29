fn main()
{
    let mut a = 0;
    let ref_a = &mut a;

    vec![ref_a, ref_a];
}
