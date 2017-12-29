struct S {a :Vec<i32>}
struct T {b :Vec<i32>}

fn main()
{
	let v = vec![];
	let s = S{a:v};
	let t = T{b:v};
}
