fn foo()->Vec<i32>
{
	let mut x = vec![0];
	let y=x;
	{return y;};
	let z=x;
	z
}

fn main (){}
