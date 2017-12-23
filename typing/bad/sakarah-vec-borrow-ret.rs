fn make_vec(a: &i32, b: &i32) -> Vec<&i32>
{
    vec![a,b]
}

fn main()
{
    let a = 1;
    let b = 2;
    make_vec(&a, &b);
}

