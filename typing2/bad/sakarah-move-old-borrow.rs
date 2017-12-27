fn take(x: Vec<i32>) {}

fn foo(a: Vec<i32>, b: Vec<i32>)
{
    let mut r = &b;
    r = &a;
    take(b);
}

fn main() { }
