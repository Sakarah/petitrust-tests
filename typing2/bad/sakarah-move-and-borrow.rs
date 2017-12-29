fn main()
{
    let mut a = vec![];
    let dummy = vec![];
    let mut b = &dummy;
    if true
    {
        a;
    }
    else
    {
        b = &a;
    }
    a = vec![42];
}
