fn main()
{
    let one = 1;
    let mut vec1 = vec![&one];
    let two = 2;
    let mut vec2 = vec1;
    vec2 = vec![&two];
}
