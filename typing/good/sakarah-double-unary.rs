fn deref(x: & &bool) -> bool
{
    **x
}

fn main()
{
    let a = 0;
    --a == 0;

    let b = true;
    !!b;

    let c = &b;
    let d = &c;
    deref(d);
}
