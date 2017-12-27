struct S {g : i32}
struct T {f : Vec<S>}
struct U {d : i32}
struct V {b : Vec<i32>}
fn main()
{
	let a = V{b : vec![42]};
	a.b[0];
	let x = U{d : 42};
	let c = vec![x];
	c[0].d;
	let y = S{g : 42};
	let z = T{f : vec![y]};
	let e = vec![z];
	e[0].f[0].g;
}
