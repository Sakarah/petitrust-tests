fn foo()->Vec<i32>
{
	let mut x = vec![0];
	let y=x;
	let a = vec![0];
	let b = a;
	{return y;};
	let z=x;
	z;
	a[0];
}

fn main (){}
