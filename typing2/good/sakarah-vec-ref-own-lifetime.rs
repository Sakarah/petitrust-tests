fn main()
{
    let a = 0;
    let b = vec![&a];
    let c = 0;
    let mut d = b;
    d = vec![&c];
}
