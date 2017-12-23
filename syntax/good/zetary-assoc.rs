struct S {c:i32}
struct T {b:S}

fn main ()
{
    let s = S{c:1};
    let a = T{b:s};
    let b = S{c:0};
    b.c+1;
    a.b.c;
    
    let d = vec![0];
    let e = vec![d];
    e[0][0];
}
