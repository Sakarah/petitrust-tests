fn main()
{
    let mut a = 0;
    let mut x = &mut a;

    x = &mut *x;
    *x = *x + 1;
}
