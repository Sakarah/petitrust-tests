fn main()
{
    let mut a = 0;
    let mut b = 0;
    *({ if true { &mut a } else { &mut b } }) = 42;
}
