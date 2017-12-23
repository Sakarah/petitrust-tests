struct A { v:i32 }

fn take(a:A) {}

fn main()
{
    let mut a1 = A { v:0 };
    let mut a2 = A { v:32 };
    let mut b = &mut a1;
    if true
    {
        take(a2)
    }
    else
    {
        b = &mut a2;
    }

    b.v = 16;
}

