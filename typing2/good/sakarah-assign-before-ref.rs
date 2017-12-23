struct A { v:i32 }

fn main()
{
    let mut a1 = A { v:0 };
    {
        if true
        {
            a1 = A { v:10 };
        }
        let b = &a1;
        b.v;
    }
}

