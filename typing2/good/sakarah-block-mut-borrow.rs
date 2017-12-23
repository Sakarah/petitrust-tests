fn main()
{
    let mut a = 0;
    {
        let b = &mut a;
    };
    {
        let c = &mut a;
    }
}

