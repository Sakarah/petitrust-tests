fn main()
{
    let mut a = 0;
    let mut x = &mut a;

    let y = x;
    x = &mut *y;
    *x = *x + 1;
}
