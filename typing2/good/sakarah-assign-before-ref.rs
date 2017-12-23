struct A { v:i32 }

fn main()
{
    let mut a1 = A { v:0 };
    {
        if true
        {
            let aa = A { v:10 };
            a1 = aa;
        }
        let b = &a1;
        b.v;
    }
}

