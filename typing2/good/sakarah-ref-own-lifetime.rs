fn main()
{
    let v1 = vec![0];
    let a = &v1;
    let v2 = vec![1];
    let mut b = a;
    b = &v2;
}
