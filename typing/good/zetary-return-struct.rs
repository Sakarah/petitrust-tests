struct S {a : i32, b : i32}

fn foo ()->bool
{
	let s = S{a : {return true;0},b : 1};
	s.a+1;
}

fn main ()
{
	foo();
}
