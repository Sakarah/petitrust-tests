struct S {a : Vec<i32>}
struct T {b : Vec<bool>}

fn main()
{
	let v = vec![];
	let s = S{a:v};
	let t = T{b:v};
}
