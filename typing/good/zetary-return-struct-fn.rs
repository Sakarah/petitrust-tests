struct S {a : i32, b : i32}

fn foo (x : i32, y : bool)->bool
{
	let c =
	{
		if y
		{
			let s = S{a : {return true;0},b : 1};
			s.a+x;
		}
		else
		{
			foo(1,{return true;false});
		}
	};
}

fn main ()
{
	foo(0,false);
}
