fn foo()->Vec<i32>
{
	let mut x = vec![0];
	let y=x;
	{return y;};
	let z=x;
	z;
	let a = vec![0];
	let b = a;
	a[0];
}

fn main (){}
