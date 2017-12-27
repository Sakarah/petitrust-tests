fn f() -> bool
{
    let v = vec![3,1,4,1,5];
    v[{ return true; 2 }];
}

fn main()
{
    f();
}
